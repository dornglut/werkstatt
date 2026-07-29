# Werkstatt Authority and Workspace Model

## Status

Proposed W1 authority for external-source references, synchronization, freshness, workspace identity, writer leases, cancellation, handoff, and recovery.

## Boundary

Werkstatt coordinates work without replacing the systems that own it.

For Dornglut:

| Fact | Authority | Werkstatt responsibility |
|---|---|---|
| Repository source and history | Git | inspect, reference revisions, register workspace |
| Current behavior | code and executable tests | navigate and collect evidence |
| Durable architecture | accepted ADR/design | reference and detect revision changes |
| Accepted work | GitHub issue | import normalized snapshot; never create a parallel editable issue |
| Durable sequence | repository roadmap | project and link phases |
| Proposed delivery | branch and pull request | observe and later publish through adapter policy |
| Independent validation | repository command and CI | request/observe exact-revision evidence |
| Accepted delivery | merge commit and issue closure | observe and reconcile local operational state |
| Workspace and execution | Werkstatt | own local registration and execution records |

A project chooses one authoritative work source. Other sources may supply supporting evidence but cannot compete as work-state authority.

## Authority classification

Each displayed fact carries one classification:

| Classification | Meaning |
|---|---|
| Authoritative | directly observed from the configured owner for that fact |
| Derived | computed from one or more identified authoritative observations |
| Cached | retained copy of a prior observation; age and source shown |
| Generated | actor/tool-produced assistance with explicit provenance |
| Observed | recorded behavior or output not itself project authority |
| Unverified | source or revision could not be verified |
| Stale | freshness requirement has expired or a newer source revision is known |
| Conflicted | applicable authorities or observations disagree |
| Retired | source binding or artifact is no longer active authority |

Classification is metadata about a fact, not a trust score for the entire document or provider.

## Authority-reference model

`AuthorityRef` combines:

- authority kind;
- `ExternalRef` identity;
- expected ownership question;
- observed `RevisionRef`;
- observation timestamp;
- freshness policy;
- normalization version;
- verification status;
- optional navigation location;
- optional sensitivity classification.

Authority kinds initially recognized:

```text
repository_source
behavior_contract
architecture_decision
work_item
program_sequence
change_proposal
validation_result
acceptance_record
security_policy
repository_policy
```

A single external resource can serve different authority kinds only when the repository model explicitly assigns them.

## Refresh policy

`FreshnessPolicy` variants:

- `Immutable` — a verified immutable revision never becomes stale by age;
- `OnDemand` — no age threshold; operation requests refresh when needed;
- `MaximumAge(duration)` — display and operation thresholds use observation age;
- `BeforeSensitiveOperation` — always refresh before write, publication, review, approval, or acceptance;
- `EventInvalidated` — provider notification or known head movement invalidates the observation;
- `Manual` — user controls refresh and stale state is always visible.

Recommended defaults:

| Source | Display policy | Sensitive-operation policy |
|---|---|---|
| Git commit | immutable after local object verification | verify object and repository identity |
| Git branch head | short maximum age | refresh before diff, publication, or review |
| GitHub issue | bounded cache | refresh before deriving an executable contract or updating source |
| ADR/document at commit | immutable at commit | compare active branch/default revision when current authority is required |
| Pull-request head | event-invalidated | refresh immediately before review response or merge |
| CI result | immutable result bound to head | confirm result belongs to current expected head |
| Merge record | immutable after provider verification | refresh issue/roadmap reconciliation separately |

## Synchronization workflow

```text
select source
    -> read external resource
        -> verify identity and revision
            -> normalize relevant facts
                -> compare prior observation
                    -> record fresh, stale, or conflict result
                        -> update derived projections
```

Synchronization never writes external authority unless a separate future mutation operation is explicitly requested and permitted.

### Refresh result

Variants:

- `Unchanged` — same verified revision/facts;
- `Advanced` — newer compatible source revision;
- `MovedUnexpectedly` — revision changed outside expected operation;
- `Deleted` — source no longer exists;
- `PermissionLost` — identity known but inaccessible;
- `Unavailable` — provider failure;
- `Conflict` — normalized source conflicts with another applicable authority;
- `IncompatibleSchema` — adapter cannot normalize provider response safely.

### Conflict examples

- work issue accepted base differs from current repository base after a decision-changing update;
- parent issue marks a phase blocked while child issue claims active without resolution;
- two accepted architecture documents own the same responsibility;
- PR head differs from review and CI evidence;
- local workspace remote points to a different repository than the project binding.

Werkstatt stops affected transitions and identifies the owning source to correct. It does not resolve project conflicts by last-write-wins.

## Work-contract derivation

Derivation inputs:

- current WorkItem authority observation;
- accepted architecture and policy references;
- accepted base revision;
- program/parent context when relevant;
- repository validation declaration;
- normalization rules.

The derived `WorkContract` records input references and revisions. It is marked:

- `Executable` only when required fields are complete and source state permits pickup;
- `Incomplete` when material fields are absent;
- `Stale` when any required input is stale or moved;
- `Conflicted` when source facts disagree;
- `Historical` after supersession or acceptance.

Regeneration produces a new immutable contract version. It never silently modifies an active Execution contract.

## Workspace identity

A workspace is identified independently from a path string.

`WorkspaceIdentity` includes:

- `WorkspaceId`;
- repository binding;
- workspace kind;
- canonical path or remote workspace reference;
- Git common-directory identity where applicable;
- repository object-format and remote identity observations;
- base revision;
- branch/detached state;
- observed head;
- creation/registration source;
- trust and isolation class.

Path normalization requirements:

- resolve symbolic links where the platform supports reliable canonicalization;
- reject paths that escape configured project roots when policy requires containment;
- compare filesystem identity where path aliases are possible;
- do not expose private local paths in public packets unless explicitly permitted.

## Repository/workspace audit

Before a write-capable execution:

1. verify path exists and adapter recognizes the repository;
2. verify repository identity matches the project binding;
3. verify expected remote identity where required;
4. verify current branch or detached state;
5. verify base/head relationship;
6. identify changed and untracked paths;
7. identify Git operation state such as merge, rebase, cherry-pick, bisect, or conflict;
8. inspect worktree administrative state;
9. check active Werkstatt lease;
10. compare protected paths and policy;
11. record an immutable audit observation.

A dirty workspace is not universally invalid. The work contract and registration mode decide whether existing changes are expected. Unexpected changes fail closed.

## Git worktree mapping

Git worktrees are one concrete workspace adapter.

Werkstatt observes:

- main versus linked worktree;
- worktree path;
- HEAD and branch;
- locked/prunable administrative state;
- Git common directory;
- porcelain worktree listing for stable machine parsing;
- move, repair, prune, and remove outcomes.

Werkstatt must not equate Git's worktree `lock` with its WriterLease:

- Git lock protects worktree administrative metadata from automatic pruning/move/delete;
- WriterLease coordinates who may write for a Werkstatt execution;
- either may exist without the other;
- future adapters may use both, but each retains separate state and failure semantics.

## Workspace modes

### Observe-only

- no writer lease;
- source and diff inspection permitted;
- no mutation attributed to Werkstatt;
- suitable for review and architecture analysis.

### Registered human workspace

- human states ownership of an existing checkout;
- Werkstatt audits repository and revision;
- no strong isolation claim;
- human remains responsible for editor, terminal, Git, and credentials;
- W2 mode.

### Managed worktree

- Werkstatt creates or registers a linked worktree;
- exact base, task branch, and lifecycle are managed;
- writer lease enforced;
- W3 mode.

### Sandboxed workspace

- adapter provides process/filesystem/network isolation properties;
- required for untrusted repositories or stronger autonomous command policy;
- W3+ mode.

### Remote workspace

- adapter exposes stable workspace identity, operations, evidence, and recovery;
- local path is optional;
- later phase.

## Writer lease contract

### Acquisition preconditions

- workspace audit is current;
- no active lease exists;
- actor/session identity is current;
- work contract is executable;
- expected repository/base/head match;
- selected policy permits acquisition;
- workspace isolation satisfies requested operations;
- no unresolved Git operation or protected-path conflict exists.

### Lease scope

- one workspace;
- one execution;
- one holder actor/session;
- one role;
- one expected repository and head lineage;
- expiration and heartbeat policy;
- allowed handoff target classes.

### Heartbeat

A heartbeat proves session liveness only. It does not prove correct work, unchanged authority, or safe workspace state.

Heartbeat updates:

- last-seen time;
- optional observed head;
- optional current execution state;
- no project-authority mutation.

### Release

Release reasons:

- completed handoff;
- voluntary pause;
- cancellation;
- owner handoff;
- failure cleanup;
- expiry recovery;
- conflict recovery.

Release requires a final workspace audit when possible. Failure to audit is recorded rather than fabricated as clean.

### Expiry and takeover

On expiry:

1. block new managed writes;
2. inspect process/session liveness where permitted;
3. audit workspace and current head;
4. preserve partial artifacts and activity;
5. classify clean release, recoverable state, or conflict;
6. require policy/approval for takeover;
7. create a new lease and usually a new Execution;
8. never erase the expired lease history.

## Head movement

Head movement categories:

- `ExpectedByExecution` — operation and resulting head match recorded activity;
- `ExpectedExternalAcceptance` — PR merge or source update observed from authority;
- `BenignReadOnlyChange` — no repository head impact;
- `UnexpectedLocalMovement` — another writer or manual Git operation changed head;
- `UnexpectedRemoteMovement` — task branch or source branch moved externally;
- `HistoryRewrite` — ancestry no longer contains expected revision.

Unexpected movement:

- marks active lease/execution conflicted;
- invalidates diff snapshots, revision-bound approvals, validation, and reviews;
- preserves all observations;
- requires reconciliation rather than automatic rebase or force push.

## Cancellation model

Cancellation has three layers:

1. `RequestCancellation` — intent recorded; new operations denied;
2. `InterruptActiveOperation` — adapter attempts graceful interruption;
3. `FinalizeCancellation` — process state, workspace, artifacts, evidence, and lease audited.

Cancellation outcomes:

- `CancelledCleanly`;
- `CancelledWithPartialArtifacts`;
- `CancellationPending`;
- `CancellationFailed`;
- `WorkspaceConflictAfterCancellation`.

An adapter timeout does not imply the process stopped. Operator recovery must verify actual state.

## Handoff model

`Handoff` records:

- source execution and actor;
- target actor/role or unassigned review queue;
- work contract and current authority revisions;
- workspace and head;
- completed operations;
- partial artifacts;
- evidence and findings;
- commands not run;
- unavailable validation;
- active blockers and decisions required;
- lease release/transfer status;
- recommended next valid transition.

A handoff is derived operational material. It does not mark the external work item accepted or complete.

## Recovery model

Recovery begins from observed state, not expected state.

### Recovery inputs

- last local transaction and activity journal position;
- current database integrity and schema version;
- active leases/executions;
- actual process/session state where observable;
- current workspace repository/head/changes;
- external authority refresh;
- pending approvals and expiry;
- retained artifacts.

### Recovery decisions

- resume same nonterminal execution when identity, lease, policy, authority, and workspace are intact;
- create a new execution from a safe known state;
- hand off partial work;
- release abandoned resources;
- mark conflicted and require owner resolution;
- mark failed when required state is unavailable or unsafe.

### Idempotency

Externally retried operations use an idempotency key scoped to adapter, resource, and intended transition.

The application records:

- request key;
- request digest;
- adapter attempt;
- confirmed result reference;
- uncertain result requiring reconciliation.

An uncertain publish/update operation is observed before retrying to avoid duplicate issues, comments, branches, or PRs.

## Authority and workspace projections

Human-facing projections should answer:

- What is authoritative?
- Which revision was observed?
- How fresh is it?
- What is derived?
- What changed since the prior observation?
- Which workspace corresponds to this work?
- Who currently owns writes?
- What exact base/head does evidence cover?
- What must be refreshed or resolved before continuing?

Advanced provider details remain available but do not replace these questions.

## W2 authority/workspace subset

W2 implements:

- one Git repository binding;
- one read-only GitHub issue source or repository-local work source;
- immutable authority observations and freshness display;
- derived WorkContract generation;
- registration of one existing human checkout;
- repository identity, branch, head, dirty-path, and Git-operation audit;
- a human ownership assertion instead of enforced lease;
- named validation observation;
- head-movement detection between snapshots;
- manual handoff/reconciliation summary.

W2 excludes:

- worktree creation/removal;
- automated leases and heartbeats;
- process supervision and cancellation enforcement;
- source-authority mutation;
- branch/PR publication;
- sandbox management;
- remote workspaces;
- automated recovery.

## W3 boundary

W3 will implement managed worktrees, WriterLease enforcement, command policy, process cancellation, SQLite recovery, and fault tests using these contracts. It must not redesign W1 semantics merely to fit one Git or process library.
