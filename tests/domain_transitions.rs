use std::any::TypeId;

use werkstatt::domain::*;

fn revision(value: &str) -> RevisionRef {
    RevisionRef {
        kind: "git_commit".into(),
        value: value.into(),
        immutable: true,
    }
}

fn execution(state: ExecutionState) -> Execution {
    let mut execution = Execution::new(ExecutionId::new(), ContractId::new());
    match state {
        ExecutionState::Prepared => {}
        ExecutionState::Running => execution.transition(ExecutionState::Running).unwrap(),
        ExecutionState::Succeeded | ExecutionState::Failed => {
            execution.transition(ExecutionState::Running).unwrap();
            execution.transition(state).unwrap();
        }
        ExecutionState::Cancelled => execution.transition(ExecutionState::Cancelled).unwrap(),
    }
    execution
}

#[test]
fn typed_ids_are_distinct_types() {
    assert_ne!(TypeId::of::<ProjectId>(), TypeId::of::<WorkItemId>());
    assert_ne!(TypeId::of::<ExecutionId>(), TypeId::of::<EvidenceId>());
}

#[test]
fn immutable_authority_and_contracts_require_complete_inputs() {
    assert!(
        AuthorityObservation::new(AuthorityObservationId::new(), "", revision("a"), "facts")
            .is_err()
    );
    assert!(
        WorkContract::new(
            ContractId::new(),
            WorkItemId::new(),
            RevisionRef {
                kind: "issue".into(),
                value: "1".into(),
                immutable: false,
            },
            1,
        )
        .is_err()
    );
    let contract = WorkContract::new(
        ContractId::new(),
        WorkItemId::new(),
        revision("accepted"),
        1,
    )
    .unwrap();
    assert_eq!(contract.version(), 1);
    assert!(contract.authority_revision().immutable);
}

#[test]
fn execution_transitions_are_explicit_and_terminal_retries_have_new_identity() {
    let mut attempt = execution(ExecutionState::Prepared);
    assert!(attempt.transition(ExecutionState::Succeeded).is_err());
    attempt.transition(ExecutionState::Running).unwrap();
    attempt.transition(ExecutionState::Succeeded).unwrap();
    assert!(attempt.transition(ExecutionState::Running).is_err());
    let retry = attempt.retry().unwrap();
    assert_ne!(attempt.id(), retry.id());
    assert_eq!(retry.state(), ExecutionState::Prepared);
}

#[test]
fn execution_success_does_not_accept_or_complete_work() {
    let work_item = WorkItem::new(WorkItemId::new(), WorkState::Ready, LifecycleStage::Review);
    let successful = execution(ExecutionState::Succeeded);
    work_item.observe_execution_success(&successful).unwrap();
    assert_eq!(work_item.state(), WorkState::Ready);
    assert_eq!(work_item.lifecycle_stage(), LifecycleStage::Review);
}

#[test]
fn readiness_requires_independent_passing_evidence_and_no_blocking_finding() {
    let local = Evidence::new(
        EvidenceId::new(),
        EvidenceSubject::LocalExecutor,
        revision("a"),
        EvidenceResult::Passed,
    );
    assert_eq!(readiness(&[local], &[]), Readiness::Blocked);

    let unavailable = Evidence::new(
        EvidenceId::new(),
        EvidenceSubject::IndependentValidation,
        revision("a"),
        EvidenceResult::Unavailable,
    );
    assert_eq!(readiness(&[unavailable], &[]), Readiness::Blocked);

    let generated = Evidence::new(
        EvidenceId::new(),
        EvidenceSubject::GeneratedReviewPacket,
        revision("a"),
        EvidenceResult::Passed,
    );
    assert_eq!(readiness(&[generated], &[]), Readiness::Blocked);

    let independent = Evidence::new(
        EvidenceId::new(),
        EvidenceSubject::IndependentValidation,
        revision("a"),
        EvidenceResult::Passed,
    );
    let blocking = Finding::new(FindingId::new(), FindingSeverity::Blocking, false);
    assert_eq!(
        readiness(std::slice::from_ref(&independent), &[blocking]),
        Readiness::Blocked
    );
    assert_eq!(readiness(&[independent], &[]), Readiness::Ready);
}

#[test]
fn moved_revision_invalidates_evidence_and_error_is_stable_and_actionable() {
    let mut evidence = Evidence::new(
        EvidenceId::new(),
        EvidenceSubject::GeneratedReviewPacket,
        revision("a"),
        EvidenceResult::Passed,
    );
    evidence.invalidated_by(&revision("b"));
    assert_eq!(evidence.result(), EvidenceResult::Stale);

    let error = execution(ExecutionState::Succeeded)
        .transition(ExecutionState::Running)
        .unwrap_err()
        .with_context("execution", "terminal");
    assert_eq!(error.code.as_str(), "execution.terminal");
    assert_eq!(error.operation, "execution.transition");
    assert!(!error.retryable);
    assert!(!error.correction.is_empty());
    assert_eq!(error.context.get("execution"), Some("terminal"));
}

#[test]
fn state_categories_are_separate_types() {
    let work = WorkState::Ready;
    let lifecycle = LifecycleStage::Review;
    let execution = ExecutionState::Running;
    assert_eq!(work, WorkState::Ready);
    assert_eq!(lifecycle, LifecycleStage::Review);
    assert_eq!(execution, ExecutionState::Running);
}
