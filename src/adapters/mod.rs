//! Concrete infrastructure adapters.

mod error;
mod git_cli;
mod github_gh;
mod local_work_source;
mod process;
mod sqlite;

pub use error::{AdapterError, AdapterErrorKind};
pub use git_cli::GitCliAdapter;
pub use github_gh::GithubGhAdapter;
pub use local_work_source::LocalWorkSourceAdapter;
pub use sqlite::{APPLICATION_ID, Database, DatabaseDiagnostics, SCHEMA_VERSION, StorageError};
