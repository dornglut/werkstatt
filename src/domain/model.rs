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
    pub id: AuthorityObservationId,
    pub source: String,
    pub revision: RevisionRef,
    pub facts: String,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct WorkContract {
    pub id: ContractId,
    pub work_item: WorkItemId,
    pub authority_revision: RevisionRef,
    pub version: u32,
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
