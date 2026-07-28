# Organization Decision Input for the Werkstatt Pilot

## Purpose

This document supplies concrete decision input to [dornglut/engineering#23](https://github.com/dornglut/engineering/issues/23). It does not itself change organization policy or supersede ADR 0003.

## Decision required

Dornglut should decide whether to authorize Werkstatt as a bounded application experiment for human-first engineering coordination and policy-controlled execution while retaining the authority, validation, and safety corrections established after the previous workflow and ForgeOps programs were retired.

## Recommended decision

Authorize W0 and W1 immediately as investigation and design.

Authorize W2 only after W0 and W1 are accepted and the organization records the repository profile and product boundary.

Require a further readiness gate before W3 or W4 introduces persistent execution state or delegated command execution.

Do not authorize automatic merge, issue selection, dependent-work activation, multi-agent scheduling, or a provider-neutral publisher as part of the initial pilot.

## Relationship to ADR 0003

ADR 0003 correctly rejected:

- a permanent ForgeOps repository and provider-neutral protocol before sustained need existed;
- a change-bundle publisher as a product-delivery prerequisite;
- source-writing validation workflows;
- direct protected-branch writes;
- arbitrary authority derived from issue or model text;
- product dependencies on one authoring tool.

Those constraints should remain.

ADR 0003 also states that future automation may be reconsidered with repeated evidence, a bounded first use case, an explicit maintenance owner, and proof that ordinary tooling is insufficient.

Werkstatt should be treated as that bounded evidence program, not as an implicit revival of ADR 0002.

## Proposed organization policy

### Permitted derived artifacts

Repositories and tools may create revision-bound:

- work packets;
- current-state and ownership matrices;
- implementation checklists;
- review packets;
- dependency and program projections;
- execution summaries;
- validation summaries.

Each derived artifact must identify source authority, source revision, generation time, and staleness. It must not become independently editable project authority.

### Permitted operational state

An execution product may own:

- workspace registration;
- actor sessions;
- bounded leases;
- capabilities and policies;
- approvals;
- retries and cancellation;
- resource limits;
- logs and activity history;
- generated artifacts and local evidence;
- cached projections and local preferences.

Operational state must not replace GitHub issues, repository roadmaps, architecture documents, pull requests, validation, or merge records.

### Mandatory safety boundaries

- repository validation remains read-only and independent from source authorship;
- protected default branches receive no direct actor writes;
- publication targets task branches and reviewable pull requests;
- issue and model text do not grant command, credential, publication, or merge authority;
- exact reviewed heads own validation and acceptance evidence;
- authoring tools remain replaceable and are not build, test, release, or consumption dependencies of product repositories;
- command execution, filesystem, network, secrets, dependencies, workflows, publication, and merge require explicit policy;
- generated execution claims do not become truth certificates;
- one writer owns a workspace and branch at a time;
- stale authority, moved heads, and conflicting writers fail closed.

## Repository profile

Add a general `application` profile to the repository standard.

An application repository owns:

- a user-facing product;
- application state and UX;
- integrations and adapters;
- releases and compatibility;
- local architecture, roadmap, issues, and validation;
- product documentation.

It must not own organization policy or copied authority from integrated repositories.

Recommended Werkstatt classification:

```text
profile: application
lifecycle: experimental
contribution: maintainer-led
```

The exact configured property values remain a native GitHub administration action where the connector cannot maintain them safely.

## General change lifecycle relationship

The Dornglut change lifecycle should remain project-neutral and risk-scaled.

Werkstatt may implement views and assistance for:

```text
intake
-> investigate
-> decide
-> deliver
-> verify
-> accept
-> reconcile
-> observe or reevaluate when required
```

Werkstatt does not become the authority for that lifecycle. The organization standard owns the semantics; repository issues, designs, code, PRs, and validation own each project's facts.

## Evidence gates

### Before W2

- W0 product boundary accepted;
- W1 domain and security design accepted;
- application profile or explicit temporary classification decided;
- human-only success criteria bound;
- no unresolved authority conflict with ADR 0003.

### Before W3

- W2 proves manual utility on a real Dornglut issue;
- ordinary repository tooling limitations are recorded rather than assumed;
- local operational state is shown to reduce work rather than duplicate authority.

### Before W4

- workspace, lease, command, approval, resource, cancellation, recovery, and evidence controls pass;
- the first delegated task is bounded to one issue, workspace, branch, and draft PR;
- independent validation is available.

### Before autonomous merge

- connected review-loop reliability is demonstrated repeatedly;
- permitted work classes and exclusions are accepted;
- exact-head merge, rollback, reconciliation, and security evidence exist;
- the owner explicitly authorizes the policy.

## Alternatives

### Leave ADR 0003 unchanged and prohibit the pilot

Not recommended. It would treat a previous oversized platform failure as evidence against all bounded workbench and execution assistance, despite recurring context, handoff, and overview problems.

### Supersede ADR 0003 completely

Not recommended. Its safety and proportionality findings remain valid.

### Restore ADR 0002

Rejected. Werkstatt should prove one concrete application and workflow before general provider, forge, executor, and publisher protocols are considered.

### Authorize all autonomous features now

Rejected. Human utility, domain correctness, workspace safety, and delegated execution must be proven in sequence.

## Proposed decision form

The organization may either:

1. accept a new ADR that narrows ADR 0003 and authorizes Werkstatt under the constraints above; or
2. accept a dedicated pilot ADR while retaining ADR 0003 for organization-wide automation platforms.

The second form is likely clearer because Werkstatt is an application experiment, not yet a general organization execution platform.

## Recommended outcome

Accept the bounded Werkstatt pilot through W2, with W3 and later execution phases gated by explicit evidence and further owner review.
