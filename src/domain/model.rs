use super::{
    AuthorityObservationId, ContractId, DomainError, ErrorCode, EvidenceId, ExecutionId, FindingId,
    WorkItemId,
};

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct RevisionRef {
    pub kind: String,
    pub value: String,
    pub immutable: bool,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct AuthorityObservation {
    id: AuthorityObservationId,
    source: String,
    revision: RevisionRef,
    facts: String,
}

impl AuthorityObservation {
    pub fn new(
        id: AuthorityObservationId,
        source: impl Into<String>,
        revision: RevisionRef,
        facts: impl Into<String>,
    ) -> Result<Self, DomainError> {
        let source = source.into();
        let facts = facts.into();
        if source.trim().is_empty() || revision.value.trim().is_empty() || facts.trim().is_empty() {
            return Err(DomainError::new(
                ErrorCode::InvalidAuthority,
                "authority.observe",
                "authority observations require source, revision, and facts",
                false,
                "supply the observed authority identity, revision, and bounded facts",
            ));
        }
        Ok(Self {
            id,
            source,
            revision,
            facts,
        })
    }

    pub fn id(&self) -> AuthorityObservationId {
        self.id
    }

    pub fn source(&self) -> &str {
        &self.source
    }

    pub fn revision(&self) -> &RevisionRef {
        &self.revision
    }

    pub fn facts(&self) -> &str {
        &self.facts
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct WorkContract {
    id: ContractId,
    work_item: WorkItemId,
    authority_revision: RevisionRef,
    version: u32,
}

impl WorkContract {
    pub fn new(
        id: ContractId,
        work_item: WorkItemId,
        authority_revision: RevisionRef,
        version: u32,
    ) -> Result<Self, DomainError> {
        if version == 0
            || !authority_revision.immutable
            || authority_revision.value.trim().is_empty()
        {
            return Err(DomainError::new(
                ErrorCode::InvalidContract,
                "contract.create",
                "work contracts require a positive version and immutable authority revision",
                false,
                "bind the contract to a complete immutable authority observation",
            ));
        }
        Ok(Self {
            id,
            work_item,
            authority_revision,
            version,
        })
    }

    pub fn id(&self) -> ContractId {
        self.id
    }

    pub fn work_item(&self) -> WorkItemId {
        self.work_item
    }

    pub fn authority_revision(&self) -> &RevisionRef {
        &self.authority_revision
    }

    pub fn version(&self) -> u32 {
        self.version
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum WorkState {
    Proposed,
    Ready,
    Completed,
    Cancelled,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum LifecycleStage {
    Implement,
    Review,
    Accepted,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct WorkItem {
    id: WorkItemId,
    state: WorkState,
    lifecycle_stage: LifecycleStage,
}

impl WorkItem {
    pub fn new(id: WorkItemId, state: WorkState, lifecycle_stage: LifecycleStage) -> Self {
        Self {
            id,
            state,
            lifecycle_stage,
        }
    }

    pub fn id(&self) -> WorkItemId {
        self.id
    }

    pub fn state(&self) -> WorkState {
        self.state
    }

    pub fn lifecycle_stage(&self) -> LifecycleStage {
        self.lifecycle_stage
    }

    pub fn observe_execution_success(&self, execution: &Execution) -> Result<(), DomainError> {
        if execution.state != ExecutionState::Succeeded {
            return Err(DomainError::new(
                ErrorCode::InvalidTransition,
                "work.observe_execution_success",
                "only a succeeded execution may be recorded as successful",
                false,
                "finish the execution before recording its outcome",
            ));
        }
        Ok(())
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum SynchronizationState {
    Current,
    Stale,
    Conflict,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum ReconciliationState {
    NotStarted,
    Pending,
    Reconciled,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum ExecutionState {
    Prepared,
    Running,
    Succeeded,
    Failed,
    Cancelled,
}

impl ExecutionState {
    pub fn terminal(self) -> bool {
        matches!(self, Self::Succeeded | Self::Failed | Self::Cancelled)
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Execution {
    pub id: ExecutionId,
    pub contract_id: ContractId,
    pub state: ExecutionState,
}

impl Execution {
    pub fn transition(&mut self, target: ExecutionState) -> Result<(), DomainError> {
        if self.state.terminal() {
            return Err(DomainError::new(
                ErrorCode::TerminalExecution,
                "execution.transition",
                "terminal attempts cannot be reopened",
                false,
                "create a new execution attempt",
            ));
        }
        let valid = matches!(
            (self.state, target),
            (
                ExecutionState::Prepared,
                ExecutionState::Running | ExecutionState::Cancelled
            ) | (
                ExecutionState::Running,
                ExecutionState::Succeeded | ExecutionState::Failed | ExecutionState::Cancelled
            )
        );
        if !valid {
            return Err(DomainError::new(
                ErrorCode::InvalidTransition,
                "execution.transition",
                "transition is not allowed",
                false,
                "use a permitted execution transition",
            ));
        }
        self.state = target;
        Ok(())
    }

    pub fn retry(&self) -> Result<Self, DomainError> {
        if !self.state.terminal() {
            return Err(DomainError::new(
                ErrorCode::InvalidTransition,
                "execution.retry",
                "only terminal attempts can be retried",
                false,
                "finish or cancel the current attempt",
            ));
        }
        Ok(Self {
            id: ExecutionId::new(),
            contract_id: self.contract_id,
            state: ExecutionState::Prepared,
        })
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum EvidenceResult {
    Passed,
    Failed,
    NotRun,
    Unavailable,
    Stale,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum EvidenceSubject {
    LocalExecutor,
    IndependentValidation,
    GeneratedReviewPacket,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Evidence {
    pub id: EvidenceId,
    pub subject: EvidenceSubject,
    pub revision: RevisionRef,
    pub result: EvidenceResult,
}

impl Evidence {
    pub fn invalidated_by(&mut self, moved_to: &RevisionRef) {
        if self.revision != *moved_to {
            self.result = EvidenceResult::Stale;
        }
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum FindingSeverity {
    Informational,
    Warning,
    Blocking,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Finding {
    pub id: FindingId,
    pub severity: FindingSeverity,
    pub resolved: bool,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum Readiness {
    Ready,
    Blocked,
}

pub fn readiness(evidence: &[Evidence], findings: &[Finding]) -> Readiness {
    let evidence_satisfies = evidence.iter().any(|item| {
        item.subject == EvidenceSubject::IndependentValidation
            && item.result == EvidenceResult::Passed
    });
    let blocking = findings
        .iter()
        .any(|finding| finding.severity == FindingSeverity::Blocking && !finding.resolved);
    if evidence_satisfies && !blocking {
        Readiness::Ready
    } else {
        Readiness::Blocked
    }
}
