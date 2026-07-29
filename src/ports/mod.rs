mod repository;
mod work_source;

pub use repository::RepositoryReader;
pub use work_source::{GithubWorkSourceReader, LocalWorkSourceReader};
