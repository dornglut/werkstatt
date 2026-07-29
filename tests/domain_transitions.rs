use werkstatt::domain::*;

fn revision(value: &str) -> RevisionRef {
    RevisionRef {
        kind: "git_commit".into(),
        value: value.into(),
        immutable: true,
    }
}
fn execution(state: ExecutionState) -> Execution {
    Execution {
        id: ExecutionId::new(),
        contract_id: ContractId::new(),
        state,
    }
}

#[test]
fn execution_transitions_are_explicit_and_terminal_retries_have_new_identity() {
    let mut attempt = execution(ExecutionState::Prepared);
    assert!(attempt.transition(ExecutionState::Succeeded).is_err());
    attempt.transition(ExecutionState::Running).unwrap();
    attempt.transition(ExecutionState::Succeeded).unwrap();
    assert!(attempt.transition(ExecutionState::Running).is_err());
    let retry = attempt.retry().unwrap();
    assert_ne!(attempt.id, retry.id);
    assert_eq!(retry.state, ExecutionState::Prepared);
}

#[test]
fn readiness_requires_independent_passing_evidence_and_no_blocking_finding() {
    let local = Evidence {
        id: EvidenceId::new(),
        subject: EvidenceSubject::LocalExecutor,
        revision: revision("a"),
        result: EvidenceResult::Passed,
    };
    assert_eq!(readiness(&[local], &[]), Readiness::Blocked);
    let independent = Evidence {
        id: EvidenceId::new(),
        subject: EvidenceSubject::IndependentValidation,
        revision: revision("a"),
        result: EvidenceResult::Passed,
    };
    let blocking = Finding {
        id: FindingId::new(),
        severity: FindingSeverity::Blocking,
        resolved: false,
    };
    assert_eq!(
        readiness(std::slice::from_ref(&independent), &[blocking]),
        Readiness::Blocked
    );
    assert_eq!(readiness(&[independent], &[]), Readiness::Ready);
}

#[test]
fn moved_revision_invalidates_evidence_and_error_is_actionable() {
    let mut evidence = Evidence {
        id: EvidenceId::new(),
        subject: EvidenceSubject::GeneratedReviewPacket,
        revision: revision("a"),
        result: EvidenceResult::Passed,
    };
    evidence.invalidated_by(&revision("b"));
    assert_eq!(evidence.result, EvidenceResult::Stale);
    let error = execution(ExecutionState::Succeeded)
        .transition(ExecutionState::Running)
        .unwrap_err();
    assert!(!error.correction.is_empty());
    assert!(error.context.0.is_empty());
}

#[test]
fn state_categories_are_separate_types() {
    let work = WorkState::Ready;
    let lifecycle = LifecycleStage::Review;
    assert_eq!(work, WorkState::Ready);
    assert_eq!(lifecycle, LifecycleStage::Review);
}
