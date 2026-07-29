use std::{error::Error, path::Path};

use crate::domain::{RepositoryObservation, RevisionRelation};

pub trait RepositoryReader {
    type Error: Error + Send + Sync + 'static;

    fn observe(&self, repository: &Path) -> Result<RepositoryObservation, Self::Error>;

    fn relation(
        &self,
        repository: &Path,
        base: &str,
        head: Option<&str>,
    ) -> Result<RevisionRelation, Self::Error>;
}
