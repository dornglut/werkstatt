use rusqlite::Connection;
use tempfile::tempdir;
use werkstatt::storage::{APPLICATION_ID, Database, SCHEMA_VERSION, StorageError};

fn path() -> (tempfile::TempDir, std::path::PathBuf) {
    let directory = tempdir().unwrap();
    let path = directory.path().join("state.sqlite");
    (directory, path)
}

#[test]
fn initializes_identifiable_foreign_key_database() {
    let (_directory, path) = path();
    let database = Database::open(&path).unwrap();
    assert_eq!(database.application_id().unwrap(), APPLICATION_ID);
    assert_eq!(database.schema_version().unwrap(), SCHEMA_VERSION);
    assert!(database.foreign_keys_enabled().unwrap());
    let diagnostics = database.diagnostics().unwrap();
    assert!(diagnostics.integrity_ok && diagnostics.foreign_key_ok);
    assert!(!diagnostics.clean_shutdown);
}

#[test]
fn rejects_wrong_application_and_future_schema() {
    let (_directory, wrong) = path();
    let connection = Connection::open(&wrong).unwrap();
    connection.pragma_update(None, "application_id", 1).unwrap();
    connection.pragma_update(None, "user_version", 1).unwrap();
    drop(connection);
    assert!(matches!(Database::open(&wrong), Err(StorageError::Format)));
    let (_directory, future) = path();
    let database = Database::open(&future).unwrap();
    drop(database);
    let connection = Connection::open(&future).unwrap();
    connection.pragma_update(None, "user_version", 2).unwrap();
    drop(connection);
    assert!(matches!(Database::open(&future), Err(StorageError::Format)));
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
fn clean_shutdown_and_local_removal_are_local_only() {
    let (_directory, path) = path();
    let mut database = Database::open(&path).unwrap();
    database.register_project("project", "local").unwrap();
    database.mark_clean_shutdown().unwrap();
    assert!(database.diagnostics().unwrap().clean_shutdown);
    database.remove_project("project").unwrap();
    assert_eq!(database.project_name("project").unwrap(), None);
}
