use std::{path::Path, time::Duration};

use rusqlite::{Connection, Error as SqlError, TransactionBehavior, params};
use thiserror::Error;

pub const APPLICATION_ID: i32 = 0x574B5354;
pub const SCHEMA_VERSION: i32 = 1;
const SCHEMA: &str = include_str!("schema_v1.sql");

#[derive(Debug, Error)]
pub enum StorageError {
    #[error(
        "storage format rejected during database.open; correction: select a Werkstatt database with a supported schema"
    )]
    Format,
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
    fn sql(operation: &'static str, source: SqlError) -> Self {
        if matches!(source, SqlError::SqliteFailure(ref error, _) if error.code == rusqlite::ErrorCode::DatabaseBusy)
        {
            Self::Busy { operation }
        } else {
            Self::Sql { operation, source }
        }
    }
}

#[derive(Debug, Eq, PartialEq)]
pub struct DatabaseDiagnostics {
    pub integrity_ok: bool,
    pub foreign_key_ok: bool,
    pub clean_shutdown: bool,
    pub journal_mode: String,
}

pub struct Database {
    connection: Connection,
}

impl Database {
    pub fn open(path: &Path) -> Result<Self, StorageError> {
        let connection =
            Connection::open(path).map_err(|error| StorageError::sql("database.open", error))?;
        connection
            .busy_timeout(Duration::from_millis(250))
            .map_err(|error| StorageError::sql("database.configure", error))?;
        connection
            .pragma_update(None, "foreign_keys", "ON")
            .map_err(|error| StorageError::sql("database.configure", error))?;
        let foreign_keys: i32 = connection
            .pragma_query_value(None, "foreign_keys", |row| row.get(0))
            .map_err(|error| StorageError::sql("database.configure", error))?;
        if foreign_keys != 1 {
            return Err(StorageError::Format);
        }
        let application_id: i32 = connection
            .pragma_query_value(None, "application_id", |row| row.get(0))
            .map_err(|error| StorageError::sql("database.inspect", error))?;
        let user_version: i32 = connection
            .pragma_query_value(None, "user_version", |row| row.get(0))
            .map_err(|error| StorageError::sql("database.inspect", error))?;
        if application_id == 0 && user_version == 0 {
            connection
                .execute_batch("BEGIN IMMEDIATE;")
                .map_err(|error| StorageError::sql("schema.initialize", error))?;
            let initialization = (|| {
                connection.pragma_update(None, "application_id", APPLICATION_ID)?;
                connection.execute_batch(SCHEMA)?;
                connection.execute("INSERT INTO schema_migrations(version, checksum, applied_at) VALUES (1, ?1, 'initial')", [schema_checksum()])?;
                connection.execute("INSERT INTO application_metadata(id, clean_shutdown, journal_mode) VALUES (1, 0, 'delete')", [])?;
                connection.pragma_update(None, "user_version", SCHEMA_VERSION)?;
                Ok::<(), SqlError>(())
            })();
            match initialization {
                Ok(()) => connection
                    .execute_batch("COMMIT;")
                    .map_err(|error| StorageError::sql("schema.initialize", error))?,
                Err(error) => {
                    let _ = connection.execute_batch("ROLLBACK;");
                    return Err(StorageError::sql("schema.initialize", error));
                }
            }
        } else if application_id != APPLICATION_ID || user_version != SCHEMA_VERSION {
            return Err(StorageError::Format);
        }
        Ok(Self { connection })
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
    pub fn foreign_keys_enabled(&self) -> Result<bool, StorageError> {
        self.connection
            .pragma_query_value(None, "foreign_keys", |row| row.get::<_, i32>(0))
            .map(|value| value == 1)
            .map_err(|error| StorageError::sql("database.inspect", error))
    }
    pub fn diagnostics(&self) -> Result<DatabaseDiagnostics, StorageError> {
        let integrity_ok: String = self
            .connection
            .pragma_query_value(None, "integrity_check", |row| row.get(0))
            .map_err(|error| StorageError::sql("database.diagnostics", error))?;
        let foreign_key_ok = !self
            .connection
            .prepare("PRAGMA foreign_key_check")
            .map_err(|error| StorageError::sql("database.diagnostics", error))?
            .exists([])
            .map_err(|error| StorageError::sql("database.diagnostics", error))?;
        let clean_shutdown: i32 = self
            .connection
            .query_row(
                "SELECT clean_shutdown FROM application_metadata WHERE id = 1",
                [],
                |row| row.get(0),
            )
            .map_err(|error| StorageError::sql("database.diagnostics", error))?;
        Ok(DatabaseDiagnostics {
            integrity_ok: integrity_ok == "ok",
            foreign_key_ok,
            clean_shutdown: clean_shutdown == 1,
            journal_mode: "delete".into(),
        })
    }
    pub fn mark_clean_shutdown(&self) -> Result<(), StorageError> {
        self.connection
            .execute(
                "UPDATE application_metadata SET clean_shutdown = 1 WHERE id = 1",
                [],
            )
            .map_err(|error| StorageError::sql("database.shutdown", error))?;
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
        let count = transaction.execute("UPDATE projects SET name = ?1, version = version + 1 WHERE id = ?2 AND version = ?3", params![name, id, expected_version]).map_err(|error| StorageError::sql("project.update", error))?;
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
                "INSERT INTO activities(execution_id, message) VALUES (NULL, ?1)",
                params!["project updated"],
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

trait OptionalRow<T> {
    fn optional(self) -> Result<Option<T>, SqlError>;
}
impl<T> OptionalRow<T> for Result<T, SqlError> {
    fn optional(self) -> Result<Option<T>, SqlError> {
        match self {
            Ok(value) => Ok(Some(value)),
            Err(SqlError::QueryReturnedNoRows) => Ok(None),
            Err(error) => Err(error),
        }
    }
}
fn schema_checksum() -> String {
    let mut hash = 0xcbf2_9ce4_8422_2325_u64;
    for byte in SCHEMA.bytes() {
        hash = (hash ^ u64::from(byte)).wrapping_mul(0x0000_0100_0000_01b3);
    }
    format!("fnv1a64:{hash:016x}")
}
