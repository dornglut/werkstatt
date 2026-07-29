use std::fs;

use rusqlite::Connection;
use tempfile::tempdir;
use werkstatt::storage::{APPLICATION_ID, Database, SCHEMA_VERSION, StorageError};

fn path() -> (tempfile::TempDir, std::path::PathBuf) {
    let directory = tempdir().unwrap();
    let path = directory.path().join("state.sqlite");
    (directory, path)
}

#[test]
fn initializes_identifiable_versioned_foreign_key_database() {
    let (_directory, path) = path();
    let database = Database::open(&path).unwrap();
    assert_eq!(database.application_id().unwrap(), APPLICATION_ID);
    assert_eq!(database.schema_version().unwrap(), SCHEMA_VERSION);
    assert!(database.foreign_keys_enabled().unwrap());
    assert!(database.migration_checksum().unwrap().starts_with("fnv1a64:"));
    let diagnostics = database.diagnostics().unwrap();
    assert!(diagnostics.integrity_ok && diagnostics.foreign_key_ok);
    assert!(!diagnostics.clean_shutdown);
    assert!(!diagnostics.opened_after_clean_shutdown);
    assert_eq!(diagnostics.journal_mode, "delete");
}

#[test]
fn rejects_wrong_application_future_schema_and_migration_mismatch() {
    let (_directory, wrong) = path();
    let connection = Connection::open(&wrong).unwrap();
    connection.pragma_update(None, "application_id", 1).unwrap();
    connection.pragma_update(None, "user_version", 1).unwrap();
    drop(connection);
    assert!(matches!(
        Database::open(&wrong),
        Err(StorageError::Format { .. })
    ));

    let (_directory, future) = path();
    let database = Database::open(&future).unwrap();
    database.mark_clean_shutdown().unwrap();
    drop(database);
    let connection = Connection::open(&future).unwrap();
    connection.pragma_update(None, "user_version", 2).unwrap();
    drop(connection);
    assert!(matches!(
        Database::open(&future),
        Err(StorageError::Format { .. })
    ));

    let (_directory, mismatched) = path();
    let database = Database::open(&mismatched).unwrap();
    database.mark_clean_shutdown().unwrap();
    drop(database);
    let connection = Connection::open(&mismatched).unwrap();
    connection
        .execute(
            "UPDATE schema_migrations SET checksum = 'wrong' WHERE version = 1",
            [],
        )
        .unwrap();
    drop(connection);
    assert!(matches!(
        Database::open(&mismatched),
        Err(StorageError::Format { .. })
    ));
}

#[test]
fn rollback_and_optimistic_conflict_leave_no_partial_project_update() {
    let (_directory, path) = path();
    let mut database = Database::open(&path).unwrap();
    database.register_project("project", "before").unwrap();
    assert!(matches!(
        database.update_project_with_activity("project", 0, "after", true),
        Err(StorageError::Conflict { .. })
    ));
    assert_eq!(
        database.project_name("project").unwrap().as_deref(),
        Some("before")
    );
    assert_eq!(database.activity_count().unwrap(), 0);
    assert!(matches!(
        database.update_project_with_activity("project", 4, "after", false),
        Err(StorageError::Conflict { .. })
    ));
    database
        .update_project_with_activity("project", 0, "after", false)
        .unwrap();
    assert_eq!(
        database.project_name("project").unwrap().as_deref(),
        Some("after")
    );
    assert_eq!(database.activity_count().unwrap(), 1);
}

#[test]
fn bounded_busy_handling_returns_retryable_structured_error() {
    let (_directory, path) = path();
    let mut database = Database::open(&path).unwrap();
    database.register_project("project", "before").unwrap();

    let locker = Connection::open(&path).unwrap();
    locker.execute_batch("BEGIN IMMEDIATE;").unwrap();
    let error = database
        .update_project_with_activity("project", 0, "after", false)
        .unwrap_err();
    assert!(matches!(error, StorageError::Busy { .. }));
    assert_eq!(error.code(), "storage.busy");
    assert_eq!(error.operation(), "project.update");
    assert!(error.retryable());
    locker.execute_batch("ROLLBACK;").unwrap();
}

#[test]
fn diagnostics_detect_foreign_key_violations() {
    let (_directory, path) = path();
    let database = Database::open(&path).unwrap();
    let raw = Connection::open(&path).unwrap();
    raw.pragma_update(None, "foreign_keys", "OFF").unwrap();
    raw.execute(
        "INSERT INTO work_items(id, project_id, contract_revision, version) VALUES ('work', 'missing', 'revision', 0)",
        [],
    )
    .unwrap();
    drop(raw);
    let diagnostics = database.diagnostics().unwrap();
    assert!(diagnostics.integrity_ok);
    assert!(!diagnostics.foreign_key_ok);
}

#[test]
fn clean_shutdown_tracking_and_local_removal_leave_external_data_untouched() {
    let (directory, path) = path();
    let external = directory.path().join("repository.txt");
    fs::write(&external, "external authority").unwrap();

    {
        let mut database = Database::open(&path).unwrap();
        database.register_project("project", "local").unwrap();
        database.remove_project("project").unwrap();
        assert_eq!(database.project_name("project").unwrap(), None);
        assert_eq!(fs::read_to_string(&external).unwrap(), "external authority");
        database.mark_clean_shutdown().unwrap();
        assert!(database.diagnostics().unwrap().clean_shutdown);
    }

    {
        let reopened = Database::open(&path).unwrap();
        let diagnostics = reopened.diagnostics().unwrap();
        assert!(diagnostics.opened_after_clean_shutdown);
        assert!(!diagnostics.clean_shutdown);
    }

    let reopened_after_unclean_drop = Database::open(&path).unwrap();
    assert!(
        !reopened_after_unclean_drop
            .diagnostics()
            .unwrap()
            .opened_after_clean_shutdown
    );
}

#[test]
fn public_storage_errors_are_actionable_and_redact_private_paths() {
    let (_directory, path) = path();
    let connection = Connection::open(&path).unwrap();
    connection.pragma_update(None, "application_id", 1).unwrap();
    connection.pragma_update(None, "user_version", 1).unwrap();
    drop(connection);

    let error = match Database::open(&path) {
        Ok(_) => panic!("wrong application identity must be rejected"),
        Err(error) => error,
    };
    let rendered = error.to_string();
    assert_eq!(error.code(), "storage.format");
    assert_eq!(error.operation(), "database.open");
    assert!(!error.retryable());
    assert!(!error.correction().is_empty());
    assert!(error.safe_context().is_empty());
    assert!(!rendered.contains(&path.display().to_string()));
}
