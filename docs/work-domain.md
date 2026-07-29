# Werkstatt Work Domain

## Status

Proposed W1 domain authority. This document defines domain vocabulary, ownership, invariants, state machines, transitions, and structured errors. It contains no Rust implementation.

## Design rules

- concepts exist only when they carry distinct ownership or behavior;
- external authority and local operational state use different types;
- humans and automated actors use the same entities;
- roles express responsibility, capabilities express possible operations, and policies decide permission;
- state changes occur through named transitions, not arbitrary field mutation;
- exact revisions and observations are first-class;
- failures are explicit, typed, and actionable;
- adapter vocabulary cannot leak into core domain semantics.

## Identity model

Every durable or operational entity uses a typed opaque identifier.

| Identifier | Identifies | Stability |
|---|---|---|
| `ProjectId` | local Werkstatt project registration | stable until project deletion |
| `RepositoryId` | local repository binding | stable across path relocation |
| `WorkSourceId` | one authoritative work-source binding | stable while binding exists |
| `ProgramId` | local projection identity for a multi-phase outcome | stable across refreshes |
| `WorkItemId` | local identity for one external or local work item | stable across refreshes |
| `ContractId` | versioned accepted work contract snapshot | immutable once recorded |
| `ActorId` | human, agent, script, or service identity | provider-independent |
| `ActorSessionId` | one connected actor-runtime or human session | operational |
| `WorkspaceId` | one registered working environment | stable until removal |
| `LeaseId` | one writer-ownership grant | immutable record |
| `ExecutionId` | one attempt to perform work | immutable identity |
| `ActivityId` | one observed execution activity | immutable identity |
| `ArtifactId` | one produced or referenced artifact | immutable identity |
| `EvidenceId` | one evidence record | immutable identity |
| `FindingId` | one review or validation finding | immutable identity |
| `ReviewId` | one assessment against a contract/revision | immutable identity |
| `ApprovalId` | one permission decision | immutable identity |
| `AcceptanceId` | one observed or explicit acceptance record | immutable identity |

External provider identifiers are stored in `ExternalRef` values and never substituted for domain identifiers.

## Source and revision primitives

### ExternalRef

Identifies an object owned outside Werkstatt.

Fields:

- `provider_kind` — for example `git`, `github`, `local_file`, `codex`, `mcp`, or `a2a`;
- `provider_instance` — stable forge, repository, service, or local-source identity;
- `resource_kind` — repository, issue, pull request, workflow run, thread, remote task, and similar;
- `opaque_id` — provider-owned identifier;
- `location` — optional human navigation URI;
- `schema_version` — adapter payload version when persisted.

### RevisionRef

Identifies immutable or explicitly versioned source state.

Fields:

- `kind` — Git commit, issue update token, document digest, workflow head, provider version, or explicit unversioned observation;
- `value` — opaque revision value;
- `observed_at`;
- `is_immutable`;
- `verification` — verified, provider-reported, derived, or unavailable.

A mutable branch name, issue URL, or file path is not sufficient as a revision.

### AuthorityObservation

Records what Werkstatt observed from an external authority.

Fields:

- source reference;
- observed revision;
- observation time;
- normalized facts used by Werkstatt;
- raw adapter payload reference where retained;
- freshness policy;
- synchronization state;
- optional conflict.

Observations are appendable evidence. The current projection points to the newest accepted observation; old observations remain historical until retention removes them.

## Project and repository model

### Project

A local Werkstatt registration for one engineering product or repository context.

Owns:

- display name and local preferences;
- repository bindings;
- exactly zero or one active authoritative work-source binding;
- policy profile references;
- retention preferences;
- current projection metadata.

Does not own external issues, repository source, architecture, PRs, CI, or merge state.

Invariants:

1. A project has at least one repository binding before workspace operations are available.
2. At most one work source is authoritative at a time.
3. Replacing the authoritative work source requires an explicit migration decision and conflict check.
4. Removing a project deletes local state only; external sources remain untouched.

### RepositoryBinding

Connects a project to a Git repository or future repository adapter.

Owns:

- local repository identity;
- canonical remote identity when available;
- local path observations;
- default branch observation;
- repository trust classification;
- adapter configuration;
- last inspected revision.

A project may bind multiple repositories, but W2 supports exactly one.

### WorkSourceBinding

Connects a project to its authoritative work source.

Kinds initially designed:

- GitHub issues;
- repository-local work documents;
- future forge or tracker adapter.

Owns:

- source identity and adapter;
- authoritative status;
- refresh cursor or update token;
- mapping rules;
- synchronization policy.

It does not copy editable work authority into Werkstatt.

## Program and work model

### Program

A projection of a multi-phase outcome.

Fields:

- external or local authority reference;
- purpose;
- closure condition;
- ordered phase references;
- dependency projection;
- latest authority observation.

A Program is not required for ordinary work.

### Phase

A coherent program slice with a durable outcome and exit gate.

Fields:

- program identity;
- stable phase key such as `W1`;
- purpose;
- dependency keys;
- owning work item reference;
- projected phase state.

Phase state is a projection from authority, not an independently editable local field.

### WorkItem

A local identity and projection for one authorized unit of work.

Owns local:

- source binding and external reference;
- latest `WorkSnapshot`;
- local execution associations;
- derived packets and projections;
- local annotations that are explicitly non-authoritative.

Does not own:

- the external issue body;
- authoritative priority or status;
- accepted architecture;
- merge state.

### WorkSnapshot

An immutable normalized observation of the authoritative work item.

Fields:

- source and revision;
- title and goal;
- owner reference;
- work class;
- lifecycle stage;
- work-item state;
- parent/program references;
- dependencies and blockers;
- contract reference;
- relevant authority links;
- observation time;
- synchronization state.

### WorkContract

A versioned immutable contract used for execution and review.

Required fields for significant work:

- goal;
- owning domain or role;
- accepted authority references;
- accepted base revision;
- included scope;
- non-goals;
- requirements;
- validation requirements;
- review dimensions;
- stop conditions;
- exit gate;
- next authorized transition.

Contract provenance:

- source work item and revision;
- derived-at timestamp;
- normalization version;
- author or deriving actor;
- acceptance status.

A derived contract is executable only when the authoritative source makes its terms accepted and no source conflict exists.

### Requirement

A testable or reviewable condition inside a contract.

Kinds:

- behavior;
- architecture;
- compatibility;
- migration;
- deletion;
- security;
- usability;
- validation;
- documentation;
- operational.

Fields:

- stable contract-local key;
- statement;
- kind;
- required evidence categories;
- owner;
- status projection;
- optional dependencies.

### Plan

A proposed ordering of operations for an execution.

A Plan:

- is derived;
- never expands the accepted contract;
- may be revised during execution;
- records unresolved decisions as blockers rather than guesses;
- is not required for trivial manual operations.

## Actor and responsibility model

### Actor

An identity capable of participating in work.

Kinds:

- human;
- model-backed agent;
- deterministic script;
- validation service;
- external service.

Actor kind does not determine authority.

### Role

A responsibility exercised for a specific operation or record.

Initial roles:

| Role | Responsibility |
|---|---|
| Requester | states desired outcome or reported problem |
| Owner | owns product or architecture decision |
| Investigator | establishes current reality and uncertainty |
| Designer | defines target behavior and boundaries |
| Executor | performs work and produces artifacts |
| Validator | produces independent validation observations |
| Reviewer | assesses conformance and raises findings |
| Approver | authorizes a gated operation |
| Acceptor | authorizes transition into accepted project state |
| Operator | owns runtime availability and operational recovery |

`RoleAssignment` binds actor, role, project/work scope, validity interval, provenance, and optional authority reference.

### Capability

A named technical operation, scoped independently from role.

A capability grant is not a policy decision by itself. See [Policy and security](policy-and-security.md).

## Workspace and execution model

### Workspace

A registered environment bound to one repository observation and exact base revision.

Fields:

- repository binding;
- workspace kind;
- canonicalized location;
- base revision;
- current branch or detached state;
- current observed head;
- ownership state;
- trust and isolation classification;
- adapter reference;
- last audit.

Kinds:

- registered checkout;
- Git worktree;
- sandboxed local workspace;
- remote isolated workspace.

W2 supports only a registered human-owned checkout.

### WriterLease

A Werkstatt concurrency-control record. It is distinct from Git's administrative worktree lock.

Fields:

- workspace;
- holder actor/session;
- role;
- work item and execution;
- expected repository and head;
- acquired time;
- expiry;
- heartbeat deadline;
- state;
- release reason.

Invariants:

1. At most one active writer lease exists per workspace.
2. A lease never grants product or architectural authority.
3. Head movement outside the owning execution marks the lease conflicted.
4. Expiry prevents new writes until recovery or takeover is resolved.
5. Force takeover requires explicit policy and a recorded conflict review.

W2 records a human ownership assertion but does not implement automated lease enforcement.

### Execution

One attempt to perform a WorkContract in a Workspace under a PolicySnapshot.

Fields:

- work item and contract;
- actor session and active role;
- workspace and expected revisions;
- policy snapshot;
- state;
- start/end time;
- resource budget;
- activities;
- artifacts;
- execution receipt;
- handoff;
- failure or cancellation.

An execution may succeed even when the work item is not accepted. `succeeded` means the execution reached its configured handoff gate.

## Artifact, evidence, and decision model

### Artifact

A produced or referenced result.

Kinds include:

- plan;
- document;
- patch or diff;
- branch;
- pull request;
- log;
- report;
- generated packet;
- binary or media artifact.

Each Artifact records provenance, content identity or external reference, sensitivity, retention class, and relation to work/execution.

### Evidence

An observation supporting a specific claim.

Each evidence record binds:

- claim or requirement;
- category;
- source;
- exact revision or environment;
- observer;
- observation time;
- result;
- limitations;
- freshness and invalidation rules.

### Finding

A problem, risk, uncertainty, or nonconformance.

Fields:

- category and severity;
- statement;
- source evidence;
- affected requirement or scope;
- owner;
- status;
- resolution evidence;
- disposition.

### Review

An assessment against one contract and one exact artifact/revision set.

A Review owns findings and verdict, but does not itself perform acceptance unless the reviewer also acts under an explicit Acceptor role and policy.

### Approval

A decision on one proposed gated operation. Approval does not imply acceptance of the resulting work.

### Acceptance

An explicit or externally observed transition that makes a result accepted project state.

For GitHub delivery, acceptance is normally observed from an exact merged revision plus applicable issue closure and accepted-main validation evidence.

## State machines

### Work-item lifecycle

States:

```text
captured
ready
active
blocked
in_review
accepted
completed
cancelled
```

Allowed transitions:

| From | To | Required meaning |
|---|---|---|
| captured | ready | owner/source accepts the work contract for pickup |
| captured | cancelled | source rejects or withdraws the work |
| ready | active | an authorized execution or investigation starts |
| ready | blocked | prerequisite becomes unavailable before start |
| ready | cancelled | source withdraws work |
| active | blocked | stop condition or unresolved dependency prevents progress |
| active | in_review | configured delivery/handoff gate is reached |
| active | cancelled | authorized cancellation |
| blocked | ready | blocker resolved before execution resumes |
| blocked | active | same authorized execution resumes after resolution |
| blocked | cancelled | owner abandons work |
| in_review | active | actionable findings require revision |
| in_review | accepted | authoritative acceptance occurs |
| in_review | cancelled | proposal rejected without continuation |
| accepted | completed | reconciliation and required post-acceptance checks finish |
| accepted | active | forbidden; correction requires a new work item or explicit reopen authority |

Werkstatt does not directly mutate externally authoritative work-item state unless a future adapter operation and policy explicitly permit it. W2 observes state only.

### Lifecycle stage

Stages:

```text
intake
investigation
decision
delivery
verification
reconciliation
observation
reevaluation
```

Stage is not a progression counter. A new work item may start at any stage allowed by its source. A stage change requires an updated authoritative contract or a new work item when the ownership boundary changes materially.

### Execution lifecycle

States:

```text
planned
queued
claimed
preparing
running
waiting_for_input
waiting_for_approval
validating
publishing
handing_off
succeeded
failed
cancelled
expired
conflicted
```

Core transitions:

| From | To | Preconditions |
|---|---|---|
| planned | queued | contract executable; actor/policy selected |
| queued | claimed | workspace and lease available |
| claimed | preparing | expected authority and revision refreshed |
| preparing | running | workspace audit and policy checks pass |
| running | waiting_for_input | material information is missing |
| running | waiting_for_approval | operation requires approval |
| running | validating | execution reaches validation gate |
| running | publishing | publication permitted without additional validation gate |
| running | handing_off | partial or manual handoff requested |
| waiting_for_input | running | accepted input supplied and authority rechecked |
| waiting_for_approval | running | valid approval granted and preconditions still current |
| waiting_for_approval | handing_off | approval denied but safe partial handoff exists |
| validating | running | failed validation is correctable within contract |
| validating | publishing | required validation passes and publication permitted |
| validating | handing_off | validation unavailable or decision required |
| publishing | handing_off | publication completes or external review becomes owner |
| handing_off | succeeded | required receipt and artifact handoff complete |
| any nonterminal | cancelled | authorized cancellation and cleanup complete |
| claimed/preparing/running/waiting/validating/publishing | expired | lease or execution deadline expires |
| preparing/running/waiting/validating/publishing | conflicted | authority/head/workspace conflict invalidates safe continuation |
| any nonterminal | failed | unrecoverable adapter, policy, or invariant failure |

Terminal states are `succeeded`, `failed`, `cancelled`, `expired`, and `conflicted`. Retry creates a new Execution linked to the prior attempt; it does not reset terminal history.

W2 uses only `planned`, `running`, `validating`, `handing_off`, `succeeded`, `failed`, and `cancelled` for a human execution record.

### Decision-artifact lifecycle

```text
draft -> proposed -> accepted
                  -> rejected
accepted -> superseded
proposed -> withdrawn
```

Accepted artifacts are immutable except factual correction and lifecycle metadata. Replacement creates a new version and explicit supersession relation.

### Authority synchronization lifecycle

States:

```text
unobserved
fresh
stale
refreshing
conflicted
unavailable
retired
```

Rules:

- a successful observation produces `fresh`;
- freshness expiry produces `stale` without changing the last observed facts;
- a write-sensitive operation requires `fresh` unless policy explicitly permits a weaker condition;
- incompatible source changes produce `conflicted`, not silent overwrite;
- provider failure produces `unavailable` while retaining the last observation with its age;
- source removal or binding replacement produces `retired`.

### Approval lifecycle

```text
requested -> granted
          -> denied
          -> cancelled
requested/granted -> expired
granted -> consumed
```

A granted approval may be consumed only when actor, role, operation, scope, work item, workspace, revision, and policy still match.

### Lease lifecycle

```text
requested -> active -> released
          -> denied
active -> expiring -> released
active/expiring -> expired
active -> conflicted
expired/conflicted -> recovered
```

Recovery does not reactivate the same lease. It closes the old record and creates a new lease after workspace audit.

## Transition command contract

A domain transition request contains:

- transition name;
- target entity and expected version;
- actor and role;
- current authority observations;
- policy decision or required approval;
- timestamp supplied by the clock port;
- idempotency key for externally retried application operations.

A successful transition returns:

- updated immutable entity version;
- emitted domain events or activity requests;
- invalidated evidence or approvals;
- required follow-up operations.

A failed transition changes no domain state.

## Error taxonomy

| Code family | Meaning | Typical correction |
|---|---|---|
| `authority.*` | source absent, stale, moved, conflicting, or unverified | refresh or resolve authority |
| `contract.*` | missing goal, scope, owner, requirement, gate, or unsupported work class | correct authoritative contract |
| `state.*` | transition not allowed from current state | use valid transition or create new work item |
| `role.*` | actor lacks required responsibility | assign authorized role |
| `policy.*` | capability denied or approval required | narrow request or obtain approval |
| `approval.*` | approval absent, expired, mismatched, denied, or consumed | request a current approval |
| `workspace.*` | missing, dirty, wrong repository/base, protected path, or invalid location | inspect or repair workspace |
| `lease.*` | occupied, expired, conflicted, or stale | hand off, release, or recover |
| `revision.*` | expected base/head differs from observation | refresh, reconcile, or stop |
| `execution.*` | cancelled, expired, budget exceeded, or unrecoverable failure | create bounded retry or handoff |
| `evidence.*` | evidence missing, stale, wrong revision, or wrong category | rerun required proof |
| `review.*` | unresolved blocking finding or invalid review scope | resolve finding or request new review |
| `acceptance.*` | acceptance authority absent or accepted state not observed | obtain/refresh authoritative acceptance |
| `storage.*` | schema, transaction, integrity, or persistence failure | rollback, repair, migrate, or stop |
| `adapter.*` | provider unavailable, incompatible, overloaded, or protocol failure | retry within policy or change adapter |
| `security.*` | trust boundary, secret, path, network, or sandbox violation | deny and require explicit correction |

## Domain-event policy

Domain events are structured observations used by application services and the activity journal. They are not a separate project authority.

Examples:

- `ProjectRegistered`;
- `AuthorityObserved`;
- `AuthorityBecameStale`;
- `WorkContractDerived`;
- `WorkspaceRegistered`;
- `WriterLeaseGranted`;
- `ExecutionStarted`;
- `ApprovalRequested`;
- `EvidenceRecorded`;
- `FindingRaised`;
- `ReviewCompleted`;
- `AcceptanceObserved`;
- `ReconciliationCompleted`.

Events contain entity IDs and versions, not mutable full project copies unless required for an immutable audit snapshot.

## W2 domain subset

W2 implements only the subset needed for manual proof:

- one Project;
- one Git RepositoryBinding;
- one GitHub or local WorkSourceBinding in read-only mode;
- WorkItem and immutable WorkSnapshot;
- derived WorkContract;
- one registered human Workspace;
- one human Execution;
- named validation observation;
- Evidence, Finding, ReviewPacket projection, and Reconciliation observation;
- authority freshness and revision conflict errors;
- no automated writer lease, actor runtime, approval engine, arbitrary command policy, publication mutation, or acceptance mutation.

The complete domain vocabulary remains designed now so W2 data and APIs do not require replacement when later phases add enforcement.
