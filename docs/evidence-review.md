# Werkstatt Evidence, Review, and Acceptance Model

## Status

Proposed W1 authority for activities, artifacts, receipts, evidence, claims, findings, reviews, validation, acceptance, and reconciliation.

## Core separation

```text
Execution activity
    records what an actor attempted or observed

Execution receipt
    summarizes one execution attempt

Evidence
    supports a specific claim at a specific revision/environment

Validation receipt
    records independent repository-defined checks

Review record
    assesses artifacts and evidence against a contract

Acceptance record
    records the authorized transition into accepted project state

Reconciliation record
    aligns local operational state with accepted external authority
```

No single record is a universal truth certificate.

## Activity model

An `Activity` is an immutable ordered observation within an Execution.

Fields:

- activity ID;
- execution ID;
- sequence number;
- activity kind;
- actor/session and role;
- start and optional end time;
- workspace and expected revision;
- operation-request reference;
- policy decision and approval references;
- redacted input summary;
- outcome;
- produced artifacts/evidence references;
- adapter diagnostic reference;
- sensitivity and retention class.

Activity kinds initially include:

```text
authority_refresh
workspace_audit
plan_created
file_observed
file_changed
command_requested
command_started
command_completed
approval_requested
approval_resolved
artifact_created
evidence_recorded
finding_raised
finding_resolved
validation_requested
validation_observed
publication_requested
publication_observed
handoff_created
cancellation_requested
recovery_performed
```

Activities describe observable actions. They do not store private chain-of-thought.

## Activity ordering

Each Execution maintains a monotonic local sequence. Adapter event timestamps may be retained but do not define canonical ordering by themselves.

Remote events record:

- remote sequence/token when available;
- receive time;
- correlation ID;
- possible duplicate or gap status.

Idempotent ingestion deduplicates by provider event ID or a stable adapter-generated digest.

## Artifact model

An `Artifact` is a produced or referenced result.

Fields:

- artifact ID and kind;
- producer actor/execution/activity;
- work item and contract;
- exact repository/workspace revision where relevant;
- content identity or ExternalRef;
- media/type information;
- size;
- generated/derived/authoritative classification;
- sensitivity;
- retention policy;
- source inputs and revisions;
- creation time;
- staleness/invalidation state.

Kinds:

- work packet;
- plan;
- document;
- patch/diff;
- branch reference;
- pull-request reference;
- command output;
- validation diagnostic;
- execution receipt;
- review packet;
- report;
- binary/media result.

Generated artifacts are replaceable projections unless the external authority explicitly accepts them.

## Claim model

Evidence supports an explicit `Claim`, not a vague statement that work is correct.

Claim fields:

- stable claim ID;
- requirement or review-dimension reference;
- statement;
- subject artifact/revision/environment;
- required evidence categories;
- owner;
- current assessment;
- invalidation rules.

Claim assessment:

```text
unassessed
supported
partially_supported
unsupported
contradicted
stale
not_applicable
unavailable
```

An assessment records the reviewer/validator and evidence used. It is not recomputed silently after source movement.

## Evidence categories

| Category | Meaning | Typical producer |
|---|---|---|
| Source inspection | observed files, history, issue, design, or dependency facts | investigator/executor/reviewer |
| Local check | command result from authoring workspace | executor or local validator |
| Exact-head CI | independent workflow result for exact feature head | CI validator |
| Accepted-main CI | independent workflow result for accepted revision | CI validator |
| Manual verification | human-observed UI, docs, workflow, or behavior | reviewer/user |
| Runtime proof | executed product/integration behavior | test/runtime adapter |
| Security evidence | policy, permission, sandbox, redaction, or threat-control result | security validator |
| Performance evidence | reproducible measurement under stated conditions | benchmark adapter/reviewer |
| Usability evidence | task observation or comparison against criteria | human evaluator |
| Provenance | origin, build, transfer, or artifact-production data | build/publisher adapter |

The producer and independence classification are separate from category.

## Evidence record

Required fields:

- evidence ID;
- category;
- claim/requirement references;
- producer actor/role/adapter;
- independence classification;
- exact subject revision or environment;
- command/workflow/manual procedure identity;
- result;
- start/end time;
- output or diagnostic artifact;
- limitations and unavailable portions;
- freshness and invalidation rules;
- sensitivity and retention.

Result variants:

```text
pass
fail
warning
observed
not_run
unavailable
cancelled
inconclusive
```

`not_run` and `unavailable` are evidence results, not passes.

## Independence classification

```text
self_reported
executor_observed
independent_local
independent_ci
human_independent
external_authority
unknown
```

Rules:

- an executor running a local command produces `executor_observed` evidence;
- exact-head CI configured independently from the author produces `independent_ci`;
- a merge record or accepted issue transition is `external_authority` evidence for acceptance;
- actor identity alone does not prove independence;
- policy defines required separation for each claim/work class.

## Revision binding and invalidation

Evidence may bind to:

- exact Git commit;
- branch head plus exact commit;
- document digest;
- issue revision/update token;
- workflow run and tested head;
- environment/image/toolchain identity;
- manual procedure version.

Invalidation triggers:

- subject revision moved;
- required authority changed;
- command or workflow definition changed;
- environment/toolchain changed beyond allowed compatibility;
- artifact content changed;
- evidence freshness expired;
- finding reveals evidence procedure invalid;
- acceptance scope changed.

Invalidated evidence remains historical and is marked stale. It is not deleted or silently retargeted.

## Execution receipt

An `ExecutionReceipt` summarizes one terminal or handed-off execution.

Required content:

- execution, work item, contract, actor, role, workspace;
- accepted base and final observed head;
- execution state and terminal reason;
- operations attempted;
- files/artifacts changed or produced;
- commands requested and observed outcomes;
- approvals used and denied;
- evidence produced;
- validation not run or unavailable;
- findings and blockers;
- resource use and limits reached;
- handoff target and next valid action;
- cleanup/lease status.

It cannot state that work is accepted unless it references an independent AcceptanceRecord.

## Validation receipt

A `ValidationReceipt` represents repository-defined independent validation.

Fields:

- validator identity and adapter;
- repository and exact revision;
- validation contract/command/workflow revision;
- run identity;
- result and conclusion;
- checked steps/categories;
- diagnostic artifacts;
- start/end time;
- environment identity;
- limitations;
- provider verification status.

For pull requests, exact-feature-head evidence is valid only for the tested head. Synthetic merge-result evidence is separate and cannot replace exact feature-head review when the repository requires it.

## Finding model

### Finding fields

- finding ID;
- source review/evidence/activity;
- category;
- severity;
- statement and rationale;
- affected requirement/artifact/path/revision;
- owner;
- status;
- required disposition;
- resolution artifact/evidence;
- created and resolved times.

Categories:

```text
correctness
scope
architecture
ownership
dependency
compatibility
security
validation
documentation
usability
performance
operations
provenance
process_integrity
```

Severity:

```text
blocking
major
minor
advisory
```

Status:

```text
open
acknowledged
in_progress
resolved
wont_fix
deferred
invalid
superseded
```

A blocking finding must be resolved, invalidated with rationale, or explicitly accepted by authorized risk ownership before acceptance.

### Finding resolution

Resolution records:

- disposition;
- changed artifact/revision;
- evidence supporting resolution;
- resolver actor/role;
- reviewer confirmation where required;
- whether prior review/evidence was invalidated.

Changing the reviewed revision normally requires a new or updated review against that revision.

## Review model

A `Review` is scoped to:

- one work contract version;
- one exact artifact/revision set;
- selected review dimensions;
- evidence snapshot;
- reviewer actor and role;
- review policy.

Review states:

```text
requested
in_progress
changes_requested
approved
commented
withdrawn
stale
completed
```

Review verdict:

- pass;
- pass with nonblocking findings;
- changes required;
- decision required;
- inconclusive.

The external provider's review-state vocabulary maps through the adapter but does not replace these meanings.

## Review dimensions

Core dimensions:

- correctness;
- scope coherence;
- ownership and dependency direction;
- migration/deletion completeness;
- validation sufficiency;
- documentation truth.

Conditional dimensions:

- security;
- compatibility;
- operations and recovery;
- performance;
- ergonomics and comprehension;
- accessibility;
- agent executability;
- strategic value.

Each dimension records required claims, observed evidence, findings, and verdict.

## Review packet

A generated `ReviewPacket` is a projection containing:

- work contract and source revisions;
- accepted base and current head;
- scope/non-goals;
- requirement-to-artifact mapping;
- diff/change summary;
- evidence and validation matrix;
- migration/deletion evidence;
- unresolved findings;
- public API/dependency/workflow changes;
- security/ergonomics evidence where required;
- known limitations and deferred work;
- staleness status.

It is regenerated when the head or contract changes. It is not accepted authority by itself.

## Acceptance model

Acceptance requires:

1. an accepted work contract and authority;
2. exact reviewed artifact/revision identity;
3. required independent validation evidence;
4. required review verdicts;
5. no unresolved blocking finding unless authorized risk acceptance exists;
6. an actor/source with Acceptor authority;
7. an authoritative acceptance operation or observation;
8. reconciliation requirements.

Acceptance sources may include:

- squash merge at exact expected head;
- accepted architecture decision and merge;
- issue state transition owned by the work source;
- explicit owner decision for non-code work;
- repository-defined release acceptance.

Local execution state `succeeded` is never sufficient.

## Acceptance record

Fields:

- acceptance ID;
- work item/contract;
- acceptor actor/role or external authority;
- exact accepted artifact/revision;
- acceptance source/reference;
- validation and review references;
- accepted limitations/risk decisions;
- acceptance time;
- required reconciliation operations;
- accepted-main evidence status.

Acceptance is immutable. Reversal or correction creates a new work item/decision, not mutation of history.

## Risk acceptance

A blocking finding may be accepted only when:

- policy permits risk acceptance for its category;
- owning risk role is identified;
- impact, scope, duration, mitigation, and follow-up are recorded;
- acceptance does not violate non-overridable product/organization invariants;
- follow-up work has authoritative ownership where required.

Risk acceptance does not convert failed validation into pass.

## Reconciliation model

Reconciliation aligns Werkstatt operational state after an external outcome.

Inputs:

- acceptance or rejection observation;
- current work source and roadmap state;
- PR/branch/merge records;
- current workspace/head;
- active executions/leases;
- artifacts, evidence, and findings;
- retention policy.

Actions may include:

- mark work snapshot accepted/completed after source refresh;
- close local execution and release workspace ownership;
- mark derived packets historical;
- invalidate obsolete approvals/evidence;
- retain accepted revision and provenance;
- surface stale roadmap/document claims for authoritative correction;
- archive or delete local logs according to retention;
- identify next authorized child work.

Werkstatt does not silently edit external issues or roadmaps in W2. It renders a reconciliation checklist and observes manual corrections.

## Reconciliation states

```text
not_required
pending
in_progress
complete
blocked
conflicted
```

Completion requires all configured local cleanup and authoritative observation steps. Missing external permissions produce `blocked`, not false completion.

## Activity journal policy

The append-only activity journal supports:

- execution timeline;
- crash recovery;
- adapter diagnostics;
- approvals and denials;
- evidence provenance;
- audit and usability analysis.

It does not replace current relational state or external authority.

Journal requirements:

- monotonic local sequence;
- schema-versioned event payload;
- bounded payload size;
- secret redaction before persistence;
- artifact references instead of large inline output;
- retention classification;
- no private chain-of-thought;
- tolerant readers for unknown future event kinds;
- transactional append with associated current-state update where required.

## Retention and sensitivity

Retention classes:

```text
ephemeral
execution_short
project_standard
audit_long
external_reference_only
```

Sensitivity classes:

```text
public
project_internal
confidential
secret_reference
personal_data
```

Secret values are never valid evidence/artifact payloads. Personal data and model transcripts require explicit retention policy.

Deleting local records:

- must preserve foreign-key/invariant consistency;
- may replace payload with a tombstone and external reference where audit continuity is needed;
- cannot delete external authority;
- must not change accepted project state.

## W2 evidence/review subset

W2 implements:

- human execution activities;
- command observation for `python scripts/validate.py` or imported repository canonical command;
- source-inspection and local-check evidence;
- evidence independence labels;
- diff/change summary from Git;
- generated review packet;
- manual findings;
- observed PR/merge/issue state where available read-only;
- reconciliation checklist;
- structured disclosure of evidence not run or unavailable.

W2 does not implement:

- CI dispatch;
- GitHub write publication;
- automated review ingestion;
- risk-acceptance workflow;
- secret-bearing evidence;
- binary artifact storage;
- automatic issue closure;
- autonomous acceptance.

## W2 acceptance-test implications

The implementation must prove:

1. moved workspace head marks prior diff/evidence stale;
2. local command success is labeled executor-observed, not independent CI;
3. command failure remains evidence and prevents a pass summary;
4. unavailable command or provider is reported honestly;
5. review packet binds contract/base/head/source revisions;
6. unresolved blocking finding prevents ready-for-acceptance projection;
7. observed merge can produce pending reconciliation without mutating the issue;
8. local record deletion cannot delete or rewrite external authority;
9. generated packet regeneration supersedes the prior projection without changing the contract;
10. errors and logs do not expose secrets or private local paths in public output mode.
