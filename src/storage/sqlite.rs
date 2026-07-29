use std::{path::Path, time::Duration};

use rusqlite::{Connection, Error as SqlError, OptionalExtension, TransactionBehavior, params};
use thiserror::Error;
use uuid::Uuid;

use crate::domain::SafeContext;

pub const APPLICATION_ID: i32 = 0x574B5354;
pub const SCHEMA_VERSION: i32 = 1;
const SCHEMA: &str = include_str!("schema_v1.sql");
const BUSY_TIMEOUT: Duration = Duration::from_millis(250);

#[derive(Debug, Error)]
pub enum StorageError {
    #[error(
        "storage format rejected during {operation}; correction: select a Werkstatt database with a supported schema"
    )]
    Format { operation: &'static str },
    #[error(
        "storage is busy during {operation}; correction: close the other local Werkstatt process and retry"
    )]
    Busy { operation: &'static str },
    #[error("storage conflict during {operation}; correction: refresh local state and retry")]
    Conflict { operation: &'static str },
    #[error("storage failure during {operation}; correction: run `werkstatt doctor`")]
    Sql {
        operation: &'static str,
        #[source]
        source: SqlError,
    },
}

impl StorageError {
    fn format(operation: &'static str) -> Self {
        Self::Format { operation }
    }

    fn sql(operation: &'static str, source: SqlError) -> Self {
        if matches!(
            source,
            SqlError::SqliteFailure(ref error, _)
                if matches!(
                    error.code,
                    rusqlite::ErrorCode::DatabaseBusy | rusqlite::ErrorCode::DatabaseLocked
                )
        ) {
            Self::Busy { operation }
        } else {
            Self::Sql { operation, source }
        }
    }

    pub const fn code(&self) -> &'static str {
        match self {
            Self::Format { .. } => "storage.format",
            Self::Busy { .. } => "storage.busy",
            Self::Conflict { .. } => "storage.conflict",
            Self::Sql { .. } => "storage.failure",
        }
    }

    pub const fn operation(&self) -> &'static str {
        match self {
            Self::Format { operation }
            | Self::Busy { operation }
            | Self::Conflict { operation }
            | Self::Sql { operation, .. } => operation,
        }
    }

    pub const fn retryable(&self) -> bool {
        matches!(self, Self::Busy { .. } | Self::Conflict { .. })
    }

    pub const fn correction(&self) -> &'static str {
        match self {
            Self::Format { .. } => "select a Werkstatt database with a supported schema",
            Self::Busy { .. } => "close the other local Werkstatt process and retry",
            Self::Conflict { .. } => "refresh local state and retry",
            Self::Sql { .. } => "run `werkstatt doctor`",
        }
    }

    pub fn safe_context(&self) -> SafeContext {
        SafeContext::default()
    }
}

#[derive(Debug, Eq, PartialEq)]
pub struct DatabaseDiagnostics {
    pub integrity_ok: bool,
    pub foreign_key_ok: bool,
    pub clean_shutdown: bool,
    pub opened_after_clean_shutdown: bool,
    pub journal_mode: String,
}

pub struct Database {
    connection: Connection,
    opened_after_clean_shutdown: bool,
}

impl Database {
    pub fn open(path: &Path) -> Result<Self, StorageError> {
        let connection =
            Connection::open(path).map_err(|error| StorageError::sql("database.open", error))?;
        connection
            .busy_timeout(BUSY_TIMEOUT)
            .map_err(|error| StorageError::sql("database.configure", error))?;
        connection
            .pragma_update(None, "foreign_keys", "ON")
            .map_err(|error| StorageError::sql("database.configure", error))?;
        let foreign_keys: i32 = connection
            .pragma_query_value(None, "foreign_keys", |row| row.get(0))
            .map_err(|error| StorageError::sql("database.configure", error))?;
        if foreign_keys != 1 {
            return Err(StorageError::format("database.configure"));
        }

        let journal_mode = query_journal_mode(&connection)?;
        if !journal_mode.eq_ignore_ascii_case("delete") {
            return Err(StorageError::format("database.configure"));
        }

        let application_id: i32 = connection
            .pragma_query_value(None, "application_id", |row| row.get(0))
            .map_err(|error| StorageError::sql("database.inspect", error))?;
        let user_version: i32 = connection
            .pragma_query_value(None, "user_version", |row| row.get(0))
            .map_err(|error| StorageError::sql("database.inspect", error))?;

        let opened_after_clean_shutdown = if application_id == 0 && user_version == 0 {
            initialize_schema(&connection, &journal_mode)?;
            false
        } else {
            if application_id != APPLICATION_ID || user_version != SCHEMA_VERSION {
                return Err(StorageError::format("database.open"));
            }
            let previous_clean = inspect_existing_schema(&connection, &journal_mode)?;
            if !previous_clean {
                let diagnostics = collect_diagnostics(&connection, false)?;
                if !diagnostics.integrity_ok || !diagnostics.foreign_key_ok {
                    return Err(StorageError::format("database.recover"));
                }
            }
            let updated = connection
                .execute(
                    "UPDATE application_metadata SET clean_shutdown = 0 WHERE id = 1",
                    [],
                )
                .map_err(|error| StorageError::sql("database.open", error))?;
            if updated != 1 {
                return Err(StorageError::format("database.open"));
            }
            previous_clean
        };

        Ok(Self {
            connection,
            opened_after_clean_shutdown,
        })
    }

    pub fn application_id(&self) -> Result<i32, StorageError> {
        self.connection
            .pragma_query_value(None, "application_id", |row| row.get(0))
            .map_err(|error| StorageError::sql("database.inspect", error))
    }

    pub fn schema_version(&self) -> Result<i32, StorageError> {
        self.connection
            .pragma_query_value(None, "user_version", |row| row.get(0))
            .map_err(|error| StorageError::sql("database.inspect", error))
    }

    pub fn migration_checksum(&self) -> Result<String, StorageError> {
        self.connection
            .query_row(
                "SELECT checksum FROM schema_migrations WHERE version = 1",
                [],
                |row| row.get(0),
            )
            .map_err(|error| StorageError::sql("database.inspect", error))
    }

    pub fn foreign_keys_enabled(&self) -> Result<bool, StorageError> {
        self.connection
            .pragma_query_value(None, "foreign_keys", |row| row.get::<_, i32>(0))
            .map(|value| value == 1)
            .map_err(|error| StorageError::sql("database.inspect", error))
    }

    pub fn diagnostics(&self) -> Result<DatabaseDiagnostics, StorageError> {
        collect_diagnostics(&self.connection, self.opened_after_clean_shutdown)
    }

    pub fn mark_clean_shutdown(&self) -> Result<(), StorageError> {
        let updated = self
            .connection
            .execute(
                "UPDATE application_metadata SET clean_shutdown = 1 WHERE id = 1",
                [],
            )
            .map_err(|error| StorageError::sql("database.shutdown", error))?;
        if updated != 1 {
            return Err(StorageError::format("database.shutdown"));
        }
        Ok(())
    }

    pub fn register_project(&mut self, id: &str, name: &str) -> Result<(), StorageError> {
        let transaction = self
            .connection
            .transaction_with_behavior(TransactionBehavior::Immediate)
            .map_err(|error| StorageError::sql("project.register", error))?;
        transaction
            .execute(
                "INSERT INTO projects(id, name, version) VALUES (?1, ?2, 0)",
                params![id, name],
            )
            .map_err(|error| StorageError::sql("project.register", error))?;
        transaction
            .commit()
            .map_err(|error| StorageError::sql("project.register", error))?;
        Ok(())
    }

    pub fn update_project_with_activity(
        &mut self,
        id: &str,
        expected_version: i64,
        name: &str,
        fail_after_update: bool,
    ) -> Result<(), StorageError> {
        let transaction = self
            .connection
            .transaction_with_behavior(TransactionBehavior::Immediate)
            .map_err(|error| StorageError::sql("project.update", error))?;
        let count = transaction
            .execute(
                "UPDATE projects SET name = ?1, version = version + 1 WHERE id = ?2 AND version = ?3",
                params![name, id, expected_version],
            )
            .map_err(|error| StorageError::sql("project.update", error))?;
        if count != 1 {
            return Err(StorageError::Conflict {
                operation: "project.update",
            });
        }
        if fail_after_update {
            return Err(StorageError::Conflict {
                operation: "project.update",
            });
        }
        transaction
            .execute(
                "INSERT INTO activities(project_id, execution_id, kind, message) VALUES (?1, NULL, 'project.updated', ?2)",
                params![id, "project updated"],
            )
            .map_err(|error| StorageError::sql("project.update", error))?;
        transaction
            .commit()
            .map_err(|error| StorageError::sql("project.update", error))?;
        Ok(())
    }

    pub fn remove_project(&mut self, id: &str) -> Result<(), StorageError> {
        self.connection
            .execute("DELETE FROM projects WHERE id = ?1", [id])
            .map_err(|error| StorageError::sql("project.remove", error))?;
        Ok(())
    }

    pub fn project_name(&self, id: &str) -> Result<Option<String>, StorageError> {
        self.connection
            .query_row("SELECT name FROM projects WHERE id = ?1", [id], |row| {
                row.get(0)
            })
            .optional()
            .map_err(|error| StorageError::sql("project.inspect", error))
    }

    pub fn activity_count(&self) -> Result<i64, StorageError> {
        self.connection
            .query_row("SELECT count(*) FROM activities", [], |row| row.get(0))
            .map_err(|error| StorageError::sql("activity.inspect", error))
    }
}

fn initialize_schema(connection: &Connection, journal_mode: &str) -> Result<(), StorageError> {
    connection
        .execute_batch("BEGIN IMMEDIATE;")
        .map_err(|error| StorageError::sql("schema.initialize", error))?;
    let initialization = (|| {
        connection.pragma_update(None, "application_id", APPLICATION_ID)?;
        connection.execute_batch(SCHEMA)?;
        connection.execute(
            "INSERT INTO schema_migrations(version, checksum, application_version) VALUES (1, ?1, ?2)",
            params![schema_checksum(), env!("CARGO_PKG_VERSION")],
        )?;
        connection.execute(
            "INSERT INTO application_metadata(id, database_id, profile_name, clean_shutdown, journal_mode) VALUES (1, ?1, 'default', 0, ?2)",
            params![Uuid::new_v4().to_string(), journal_mode],
        )?;
        connection.pragma_update(None, "user_version", SCHEMA_VERSION)?;
        Ok::<(), SqlError>(())
    })();
    match initialization {
        Ok(()) => connection
            .execute_batch("COMMIT;")
            .map_err(|error| StorageError::sql("schema.initialize", error)),
        Err(error) => {
            let _ = connection.execute_batch("ROLLBACK;");
            Err(StorageError::sql("schema.initialize", error))
        }
    }
}

fn inspect_existing_schema(
    connection: &Connection,
    journal_mode: &str,
) -> Result<bool, StorageError> {
    let migration: Option<(String, String)> = connection
        .query_row(
            "SELECT checksum, application_version FROM schema_migrations WHERE version = 1",
            [],
            |row| Ok((row.get(0)?, row.get(1)?)),
        )
        .optional()
        .map_err(|error| StorageError::sql("database.inspect", error))?;
    let Some((checksum, application_version)) = migration else {
        return Err(StorageError::format("database.open"));
    };
    if checksum != schema_checksum() || application_version.trim().is_empty() {
        return Err(StorageError::format("database.open"));
    }

    let metadata: Option<(String, String, i32, String)> = connection
        .query_row(
            "SELECT database_id, profile_name, clean_shutdown, journal_mode FROM application_metadata WHERE id = 1",
            [],
            |row| Ok((row.get(0)?, row.get(1)?, row.get(2)?, row.get(3)?)),
        )
        .optional()
        .map_err(|error| StorageError::sql("database.inspect", error))?;
    let Some((database_id, profile_name, clean_shutdown, recorded_journal_mode)) = metadata else {
        return Err(StorageError::format("database.open"));
    };
    if database_id.trim().is_empty()
        || profile_name.trim().is_empty()
        || !recorded_journal_mode.eq_ignore_ascii_case(journal_mode)
    {
        return Err(StorageError::format("database.open"));
    }
    Ok(clean_shutdown == 1)
}

fn collect_diagnostics(
    connection: &Connection,
    opened_after_clean_shutdown: bool,
) -> Result<DatabaseDiagnostics, StorageError> {
    let integrity: String = connection
        .pragma_query_value(None, "integrity_check", |row| row.get(0))
        .map_err(|error| StorageError::sql("database.diagnostics", error))?;
    let foreign_key_ok = !connection
        .prepare("PRAGMA foreign_key_check")
        .map_err(|error| StorageError::sql("database.diagnostics", error))?
        .exists([])
        .map_err(|error| StorageError::sql("database.diagnostics", error))?;
    let clean_shutdown: i32 = connection
        .query_row(
            "SELECT clean_shutdown FROM application_metadata WHERE id = 1",
            [],
            |row| row.get(0),
        )
        .map_err(|error| StorageError::sql("database.diagnostics", error))?;
    Ok(DatabaseDiagnostics {
        integrity_ok: integrity == "ok",
        foreign_key_ok,
        clean_shutdown: clean_shutdown == 1,
        opened_after_clean_shutdown,
        journal_mode: query_journal_mode(connection)?,
    })
}

fn query_journal_mode(connection: &Connection) -> Result<String, StorageError> {
    connection
        .pragma_query_value(None, "journal_mode", |row| row.get(0))
        .map_err(|error| StorageError::sql("database.inspect", error))
}

fn schema_checksum() -> String {
    let mut hash = 0xcbf2_9ce4_8422_2325_u64;
    for byte in SCHEMA.bytes() {
        hash = (hash ^ u64::from(byte)).wrapping_mul(0x0000_0100_0000_01b3);
    }
    format!("fnv1a64:{hash:016x}")
}
