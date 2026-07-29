//! Concrete infrastructure adapters.

mod sqlite;

pub use sqlite::{APPLICATION_ID, Database, DatabaseDiagnostics, SCHEMA_VERSION, StorageError};
