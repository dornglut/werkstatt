mod error;
mod ids;
mod model;
mod repository;
mod work_source;

pub use error::{DomainError, ErrorCode, SafeContext};
pub use ids::{
    AuthorityObservationId, ContractId, EvidenceId, ExecutionId, FindingId, ProjectId, WorkItemId,
};
pub use model::{
    AuthorityObservation, Evidence, EvidenceResult, EvidenceSubject, Execution, ExecutionState,
    Finding, FindingSeverity, LifecycleStage, Readiness, ReconciliationState, RevisionRef,
    SynchronizationState, WorkContract, WorkItem, WorkState, readiness,
};
pub use repository::{
    DiffObservation, GitOperation, HeadObservation, PathChange, PathChangeKind, RemoteObservation,
    RepositoryIdentity, RepositoryObservation, RevisionRelation, WorktreeObservation,
};
pub use work_source::{SourceIdentity, SourceKind, WorkFacts, WorkSourceObservation};
