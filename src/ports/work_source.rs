use std::{error::Error, path::Path};

use crate::work_source::ObservedWorkSource;

pub trait LocalWorkSourceReader {
    type Error: Error + Send + Sync + 'static;

    fn observe_file(&self, source: &Path) -> Result<ObservedWorkSource, Self::Error>;
}

pub trait GithubWorkSourceReader {
    type Error: Error + Send + Sync + 'static;

    fn observe_issue(&self, issue_url: &str) -> Result<ObservedWorkSource, Self::Error>;
}
