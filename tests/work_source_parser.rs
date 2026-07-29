use werkstatt::{
    domain::{RevisionRef, SourceIdentity, SourceKind, SynchronizationState},
    work_source::{MAX_SOURCE_BYTES, parse_json, parse_markdown, to_versioned_json},
};

fn identity() -> SourceIdentity {
    SourceIdentity::new("local_file", "work_document", "fixture", None).unwrap()
}

fn table_markdown(goal: &str) -> String {
    format!(
        "# W2B fixture\n\n\
## Work contract\n\n\
| Field | Value |\n\
|---|---|\n\
| Owner | dornglut/werkstatt |\n\
| Work class | Product / implementation |\n\
| Lifecycle stage | Implement |\n\
| Accepted base | bd4f12f770fa82d25657f41fd8cff5e3a299a8a1 |\n\
| Exit gate | Exact-head validation succeeds |\n\
| Next transition | Independent review |\n\n\
## Goal\n\n{goal}\n\n\
## Scope\n\n- Git observation\n- Work-source observation\n\n\
## Non-goals\n\n- No Git mutation\n- No GitHub mutation\n\n\
## Validation\n\n- `python scripts/validate.py`\n\n\
## Stop conditions\n\n- Stop on authority conflict\n\n\
## Repository notes\n\nAdditional repository-specific context.\n"
    )
}

fn w2_issue_markdown() -> &'static str {
    "# W2: Implement and prove the human-first headless CLI\n\n\
Parent: #1\n\
Kind: Implementation / product proof\n\
Lifecycle stage: Implement and prove\n\
Work class: Product and architectural\n\
Owner: dornglut/werkstatt headless application\n\
Accepted design base: a87d8fbc5ac1e4e31d5a3f6b601237cf8e3b8cd9\n\
Exit gate: complete W2 acceptance matrix and counterbalanced human pilot\n\
Next transition: explicit owner decision on W3 authorization\n\n\
## Objective\n\nProve a useful human-first, model-free headless CLI without duplicating external authority.\n\n\
## Executable contract\n\nDeliver one Rust package with a synchronous CLI and read-only repository observation.\n\n\
## Acceptance and review\n\nReview authority separation, path minimization, dependency scope, and exact-head validation.\n\n\
## Exclusions and stop conditions\n\nNo model runtime, GitHub mutation, arbitrary command execution, GUI, or autonomy. Stop for moved authority or missing exact-head validation.\n"
}

fn w2b_issue_markdown() -> &'static str {
    "# W2B: Repository and work-source observation\n\n\
Parent: #8\n\
Kind: bounded implementation delivery\n\
Lifecycle stage: Review\n\
Work class: Product / implementation\n\
Owner: dornglut/werkstatt headless application\n\
Accepted base: bd4f12f770fa82d25657f41fd8cff5e3a299a8a1\n\
Exit gate: independent review, exact-head validation, merge, and accepted-main validation\n\
Next transition: activate W2C only after accepted-main validation\n\n\
## Objective\n\nObserve repositories and authoritative work sources without mutation.\n\n\
## Scope\n\n- Fixed-argv Git observation\n- Bounded Markdown and JSON source observation\n- Optional read-only gh issue view\n\n\
## Exclusions\n\n- No Git or GitHub mutation\n- No W2C commands or W3 execution policy\n\n\
## Validation\n\n- python scripts/validate.py\n- Exact-head CI on the reviewed revision\n\n\
## Stop conditions\n\n- Stop on authority conflict or unbounded provider behavior\n"
}

#[test]
fn parses_contract_table_and_reuses_w2a_authority_types() {
    let bytes = table_markdown("Observe repositories without mutation");
    let observed = parse_markdown(bytes.as_bytes(), identity(), 7, false).unwrap();
    let observation = observed.observation();
    assert_eq!(observation.kind(), SourceKind::Markdown);
    assert_eq!(
        observation.facts().goal(),
        "Observe repositories without mutation"
    );
    assert_eq!(observation.facts().scope().len(), 2);
    assert!(
        observation
            .facts()
            .supplemental()
            .contains_key("repository notes")
    );
    assert_eq!(
        observation.authority().source(),
        "local_file:work_document:fixture"
    );
    assert_eq!(observation.authority().revision(), observation.revision());
    assert_eq!(observation.revision().kind, "sha256");
    assert!(observation.revision().immutable);
    assert!(!observation.is_mutable());
    assert_eq!(observed.payload(), bytes.as_bytes());
}

#[test]
fn parses_real_dornglut_leading_metadata_forms() {
    let w2 = parse_markdown(w2_issue_markdown().as_bytes(), identity(), 1, true).unwrap();
    assert_eq!(
        w2.observation().facts().accepted_base(),
        "a87d8fbc5ac1e4e31d5a3f6b601237cf8e3b8cd9"
    );
    assert_eq!(
        w2.observation().facts().owner(),
        "dornglut/werkstatt headless application"
    );
    assert!(!w2.observation().facts().scope().is_empty());
    assert!(!w2.observation().facts().validation().is_empty());
    assert!(!w2.observation().facts().stop_conditions().is_empty());
    assert_eq!(
        w2.observation().facts().supplemental()["metadata.parent"],
        ["#1"]
    );

    let w2b = parse_markdown(w2b_issue_markdown().as_bytes(), identity(), 2, true).unwrap();
    assert_eq!(w2b.observation().facts().lifecycle_stage(), "Review");
    assert_eq!(w2b.observation().facts().scope().len(), 3);
    assert_eq!(w2b.observation().facts().non_goals().len(), 2);
}

#[test]
fn rejects_missing_duplicate_cross_form_and_malformed_values() {
    let missing = table_markdown("goal")
        .replace("## Stop conditions\n\n- Stop on authority conflict\n\n", "");
    assert_eq!(
        parse_markdown(missing.as_bytes(), identity(), 1, false)
            .unwrap_err()
            .code(),
        "source.missing_field"
    );

    let duplicate = table_markdown("goal").replacen(
        "# W2B fixture\n\n",
        "# W2B fixture\n\nOwner: duplicate\n\n",
        1,
    );
    assert_eq!(
        parse_markdown(duplicate.as_bytes(), identity(), 1, false)
            .unwrap_err()
            .code(),
        "source.duplicate_field"
    );

    let malformed = table_markdown("goal").replace(
        "| Owner | dornglut/werkstatt |",
        "| Owner | dornglut/werkstatt | extra |",
    );
    assert_eq!(
        parse_markdown(malformed.as_bytes(), identity(), 1, false)
            .unwrap_err()
            .code(),
        "source.malformed_table"
    );
}

#[test]
fn rejects_non_utf8_and_oversized_sources() {
    assert_eq!(
        parse_markdown(&[0xff, 0xfe], identity(), 1, false)
            .unwrap_err()
            .code(),
        "source.non_utf8"
    );
    assert_eq!(
        parse_markdown(&vec![b'x'; MAX_SOURCE_BYTES + 1], identity(), 1, false)
            .unwrap_err()
            .code(),
        "source.too_large"
    );
}

#[test]
fn json_round_trip_is_versioned_and_deterministic() {
    let bytes = table_markdown("Observe repositories without mutation");
    let original = parse_markdown(bytes.as_bytes(), identity(), 7, false).unwrap();
    let json = to_versioned_json(original.observation());
    let parsed = parse_json(json.as_bytes(), identity(), 8, false).unwrap();
    assert_eq!(parsed.observation().kind(), SourceKind::Json);
    assert_eq!(
        parsed.observation().facts().title(),
        original.observation().facts().title()
    );
    assert_eq!(
        parsed.observation().facts().accepted_base(),
        original.observation().facts().accepted_base()
    );
    assert_eq!(
        parsed.observation().facts().supplemental(),
        original.observation().facts().supplemental()
    );
    assert_eq!(to_versioned_json(parsed.observation()), json);
    assert_eq!(parsed.payload(), json.as_bytes());
}

#[test]
fn revision_movement_uses_w2a_synchronization_state() {
    let first = parse_markdown(table_markdown("first").as_bytes(), identity(), 1, false).unwrap();
    let same = parse_markdown(table_markdown("first").as_bytes(), identity(), 2, false).unwrap();
    let second = parse_markdown(table_markdown("second").as_bytes(), identity(), 3, false).unwrap();
    assert_eq!(
        same.observation()
            .synchronization_against(Some(first.observation().revision())),
        SynchronizationState::Current
    );
    assert_eq!(
        second
            .observation()
            .synchronization_against(Some(first.observation().revision())),
        SynchronizationState::Stale
    );
    let incompatible = RevisionRef {
        kind: "git".into(),
        value: "abc".into(),
        immutable: true,
    };
    assert_eq!(
        second
            .observation()
            .synchronization_against(Some(&incompatible)),
        SynchronizationState::Conflict
    );
}

#[test]
fn json_does_not_infer_missing_decisions() {
    let json = br#"{"schemaVersion":1,"title":"x"}"#;
    assert_eq!(
        parse_json(json, identity(), 1, false).unwrap_err().code(),
        "source.missing_field"
    );
}

#[test]
fn github_issue_payload_is_mutable_and_revision_honest() {
    let body = w2b_issue_markdown();
    let escaped_body = body
        .replace('\\', "\\\\")
        .replace('"', "\\\"")
        .replace('\n', "\\n");
    let payload = format!(
        "{{\"number\":10,\"title\":\"W2B: Repository and work-source observation\",\"body\":\"{escaped_body}\",\"state\":\"OPEN\",\"updatedAt\":\"2026-07-29T10:00:00Z\",\"url\":\"https://github.com/dornglut/werkstatt/issues/10\"}}"
    );
    let observed =
        werkstatt::work_source::parse_github_issue_payload(payload.as_bytes(), 9).unwrap();
    let observation = observed.observation();
    assert_eq!(observation.kind(), SourceKind::GithubIssue);
    assert!(observation.is_mutable());
    assert!(observation.revision().immutable);
    assert!(
        observation
            .limitations()
            .iter()
            .any(|value| value.contains("not an immutable revision"))
    );
    assert_eq!(observed.payload(), payload.as_bytes());
}
