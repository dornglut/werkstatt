use werkstatt::{domain::SourceIdentity, work_source::parse_markdown};

#[test]
fn leading_issue_metadata_normalizes_revision_presentation_markup() {
    let markdown = "# W2B: Repository and work-source observation\n\n\
Parent: #8\n\
Kind: bounded implementation delivery\n\
Lifecycle stage: Review\n\
Work class: Product / implementation\n\
Owner: dornglut/werkstatt headless application\n\
Accepted base: `bd4f12f770fa82d25657f41fd8cff5e3a299a8a1`\n\
Exit gate: independent review and exact-head validation\n\
Next transition: merge after acceptance\n\n\
## Objective\n\nObserve accepted work without mutation.\n\n\
## Scope\n\n- Read-only source observation\n\n\
## Exclusions\n\n- No source mutation\n\n\
## Validation\n\n- python scripts/validate.py\n\n\
## Stop conditions\n\n- Stop on authority conflict\n";
    let identity = SourceIdentity::new("local_file", "work_document", "issue-10", None).unwrap();
    let observed = parse_markdown(markdown.as_bytes(), identity, 1, true).unwrap();

    assert_eq!(
        observed.observation().facts().accepted_base(),
        "bd4f12f770fa82d25657f41fd8cff5e3a299a8a1"
    );
    assert_eq!(
        observed.observation().facts().supplemental()["metadata.parent"],
        ["#8"]
    );
}
