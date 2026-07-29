use super::{DomainError, ErrorCode};

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct RepositoryIdentity {
    observation_fingerprint: String,
    common_directory_key: String,
    display: String,
}

impl RepositoryIdentity {
    pub fn new(
        observation_fingerprint: impl Into<String>,
        common_directory_key: impl Into<String>,
        display: impl Into<String>,
    ) -> Result<Self, DomainError> {
        let observation_fingerprint = observation_fingerprint.into();
        let common_directory_key = common_directory_key.into();
        let display = display.into();
        if observation_fingerprint.trim().is_empty()
            || common_directory_key.trim().is_empty()
            || display.trim().is_empty()
        {
            return Err(DomainError::new(
                ErrorCode::InvalidObservation,
                "repository.identity",
                "repository identity observation is incomplete",
                false,
                "observe the repository fingerprint, common directory, and a safe display label",
            ));
        }
        Ok(Self {
            observation_fingerprint,
            common_directory_key,
            display,
        })
    }

    pub fn observation_fingerprint(&self) -> &str {
        &self.observation_fingerprint
    }

    pub fn common_directory_key(&self) -> &str {
        &self.common_directory_key
    }

    pub fn display(&self) -> &str {
        &self.display
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct HeadObservation {
    revision: String,
    branch: Option<String>,
}

impl HeadObservation {
    pub fn new(revision: impl Into<String>, branch: Option<String>) -> Result<Self, DomainError> {
        let revision = revision.into();
        if revision.trim().is_empty() {
            return Err(DomainError::new(
                ErrorCode::InvalidObservation,
                "repository.head",
                "repository head revision is unavailable",
                false,
                "inspect a repository with at least one committed revision",
            ));
        }
        Ok(Self { revision, branch })
    }

    pub fn revision(&self) -> &str {
        &self.revision
    }

    pub fn branch(&self) -> Option<&str> {
        self.branch.as_deref()
    }

    pub fn is_detached(&self) -> bool {
        self.branch.is_none()
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct RemoteObservation {
    name: String,
    identity: String,
}

impl RemoteObservation {
    pub fn new(name: impl Into<String>, identity: impl Into<String>) -> Result<Self, DomainError> {
        let name = name.into();
        let identity = identity.into();
        if name.trim().is_empty() || identity.trim().is_empty() {
            return Err(DomainError::new(
                ErrorCode::InvalidObservation,
                "repository.remote",
                "remote observation requires name and credential-safe identity",
                false,
                "observe and sanitize the configured remote identity",
            ));
        }
        Ok(Self { name, identity })
    }

    pub fn name(&self) -> &str {
        &self.name
    }

    pub fn identity(&self) -> &str {
        &self.identity
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum PathChangeKind {
    Modified,
    Added,
    Deleted,
    Renamed,
    Copied,
    Unmerged,
    Untracked,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct PathChange {
    kind: PathChangeKind,
    path_key: String,
    display: String,
    original_display: Option<String>,
}

impl PathChange {
    pub fn new(
        kind: PathChangeKind,
        path_key: impl Into<String>,
        display: impl Into<String>,
        original_display: Option<String>,
    ) -> Result<Self, DomainError> {
        let path_key = path_key.into();
        let display = display.into();
        if path_key.trim().is_empty() || display.trim().is_empty() {
            return Err(DomainError::new(
                ErrorCode::InvalidObservation,
                "repository.path_change",
                "path change requires opaque identity and safe display",
                false,
                "retain the repository-relative path as a bounded observation",
            ));
        }
        Ok(Self {
            kind,
            path_key,
            display,
            original_display,
        })
    }

    pub fn kind(&self) -> PathChangeKind {
        self.kind
    }

    pub fn path_key(&self) -> &str {
        &self.path_key
    }

    pub fn display(&self) -> &str {
        &self.display
    }

    pub fn original_display(&self) -> Option<&str> {
        self.original_display.as_deref()
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum GitOperation {
    None,
    Merge,
    Rebase,
    CherryPick,
    Revert,
    Bisect,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct WorktreeObservation {
    identity: String,
    head: Option<String>,
    branch: Option<String>,
    current: bool,
}

impl WorktreeObservation {
    pub fn new(
        identity: impl Into<String>,
        head: Option<String>,
        branch: Option<String>,
        current: bool,
    ) -> Result<Self, DomainError> {
        let identity = identity.into();
        if identity.trim().is_empty() {
            return Err(DomainError::new(
                ErrorCode::InvalidObservation,
                "repository.worktree",
                "worktree identity is unavailable",
                false,
                "observe the worktree administrative identity",
            ));
        }
        Ok(Self {
            identity,
            head,
            branch,
            current,
        })
    }

    pub fn identity(&self) -> &str {
        &self.identity
    }

    pub fn head(&self) -> Option<&str> {
        self.head.as_deref()
    }

    pub fn branch(&self) -> Option<&str> {
        self.branch.as_deref()
    }

    pub fn is_current(&self) -> bool {
        self.current
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct DiffObservation {
    statistics: String,
    text: String,
    truncated: bool,
}

impl DiffObservation {
    pub fn new(statistics: String, text: String, truncated: bool) -> Self {
        Self {
            statistics,
            text,
            truncated,
        }
    }

    pub fn statistics(&self) -> &str {
        &self.statistics
    }

    pub fn text(&self) -> &str {
        &self.text
    }

    pub fn is_truncated(&self) -> bool {
        self.truncated
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct RepositoryObservation {
    identity: RepositoryIdentity,
    head: HeadObservation,
    remotes: Vec<RemoteObservation>,
    changes: Vec<PathChange>,
    worktrees: Vec<WorktreeObservation>,
    operation: GitOperation,
    diff: DiffObservation,
    limitations: Vec<String>,
}

impl RepositoryObservation {
    #[allow(clippy::too_many_arguments)]
    pub fn new(
        identity: RepositoryIdentity,
        head: HeadObservation,
        remotes: Vec<RemoteObservation>,
        changes: Vec<PathChange>,
        worktrees: Vec<WorktreeObservation>,
        operation: GitOperation,
        diff: DiffObservation,
        limitations: Vec<String>,
    ) -> Self {
        Self {
            identity,
            head,
            remotes,
            changes,
            worktrees,
            operation,
            diff,
            limitations,
        }
    }

    pub fn identity(&self) -> &RepositoryIdentity {
        &self.identity
    }

    pub fn head(&self) -> &HeadObservation {
        &self.head
    }

    pub fn remotes(&self) -> &[RemoteObservation] {
        &self.remotes
    }

    pub fn changes(&self) -> &[PathChange] {
        &self.changes
    }

    pub fn worktrees(&self) -> &[WorktreeObservation] {
        &self.worktrees
    }

    pub fn operation(&self) -> GitOperation {
        self.operation
    }

    pub fn diff(&self) -> &DiffObservation {
        &self.diff
    }

    pub fn limitations(&self) -> &[String] {
        &self.limitations
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct RevisionRelation {
    base: String,
    head: String,
    ancestor: bool,
    merge_base: Option<String>,
}

impl RevisionRelation {
    pub fn new(
        base: impl Into<String>,
        head: impl Into<String>,
        ancestor: bool,
        merge_base: Option<String>,
    ) -> Result<Self, DomainError> {
        let base = base.into();
        let head = head.into();
        if base.trim().is_empty() || head.trim().is_empty() {
            return Err(DomainError::new(
                ErrorCode::InvalidObservation,
                "repository.relation",
                "revision relation requires base and head",
                false,
                "resolve both revisions before comparing them",
            ));
        }
        Ok(Self {
            base,
            head,
            ancestor,
            merge_base,
        })
    }

    pub fn base(&self) -> &str {
        &self.base
    }

    pub fn head(&self) -> &str {
        &self.head
    }

    pub fn is_ancestor(&self) -> bool {
        self.ancestor
    }

    pub fn merge_base(&self) -> Option<&str> {
        self.merge_base.as_deref()
    }
}
