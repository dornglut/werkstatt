# Werkstatt Ports and Storage Design

## Status

Proposed W1 authority for application ports, adapter responsibilities, protocol mappings, SQLite logical schema, transactions, migrations, integrity, backup, and recovery. This document does not add database migrations or implementation.

## Port design principles

1. Ports describe Werkstatt use cases, not provider APIs.
2. Provider identifiers and payloads remain opaque adapter values.
3. Read and mutation operations are separate.
4. Observation and command results identify exact source revisions where possible.
5. Cancellation, timeout, retryability, idempotency, and partial failure are explicit.
6. Sensitive operations require a prior domain policy decision.
7. Adapters return structured diagnostics without deciding project authority.
8. Protocol version drift is contained by adapter schema versions and compatibility tests.
9. Domain code does not depend on Git, GitHub, SQLite, Codex, MCP, A2A, process, or Runenwerk libraries.
10. W2 implements only the minimum read-only/manual adapters needed for human proof.

## Common port contract

Every port operation receives an `OperationContext` containing:

- project, work item, contract, execution, and workspace IDs where relevant;
- actor/session and active role;
- policy-decision reference;
- expected authority/repository revisions;
- deadline and cancellation token;
- idempotency key when mutation/retry can duplicate effects;
- tracing/correlation ID;
- sensitivity classification.

Every result provides:

- success value or typed adapter error;
- provider/adapter identity and version;
- observed source revision;
- start/end time;
- retryability and safe retry conditions;
- warnings/limitations;
- redacted diagnostic reference.

## Repository port

### Purpose

Inspect and later mutate version-controlled repositories without leaking Git-specific structures into the domain.

### Read operations

- identify repository at path;
- report repository/common-directory identity;
- observe remotes and default branch;
- resolve revision expression to immutable revision;
- inspect commit ancestry;
- observe current branch, head, and operation state;
- list changed/untracked paths;
- produce diff between exact revisions/workspace;
- read file at exact revision;
- list workspaces/worktrees;
- inspect administrative lock/prunable state.

### Later mutation operations

- create branch/worktree;
- stage/commit changes;
- move or remove managed worktree;
- publish branch;
- repair managed worktree metadata.

Mutation is outside W2.

### Stable mapping

- Git commit SHA maps to `RevisionRef(kind=git_commit)`;
- branch name remains a mutable locator plus observed commit;
- Git worktree identity maps to Workspace adapter metadata;
- Git worktree administrative lock remains adapter state, not WriterLease;
- `git worktree list --porcelain` or equivalent stable library output is preferred for machine parsing.

### Errors

```text
repository.not_found
repository.identity_mismatch
repository.revision_not_found
repository.operation_in_progress
repository.dirty_unexpected
repository.ancestry_conflict
repository.remote_mismatch
repository.provider_failure
```

## Work-source port

### Purpose

Read one authoritative work source and later apply explicit source mutations.

### Read operations

- identify source and capabilities;
- fetch work item by external reference;
- fetch parent/program relationships;
- list eligible work by explicit query;
- fetch comments/reviews used as evidence;
- observe update token/revision;
- normalize state, stage, owner, dependencies, contract fields, and links;
- detect deletion, permission loss, or schema incompatibility.

### Later mutation operations

- comment;
- update accepted fields;
- create child work;
- close/reopen;
- reconcile source state.

All mutations are outside W2.

### GitHub mapping

- issue number plus repository identity is the source identity;
- issue updated timestamp alone is not assumed immutable; adapter should retain provider node/database identity and normalized-content digest or stronger token where available;
- pull requests are change-proposal resources, not work items unless repository policy explicitly uses them as such;
- native sub-issue relationships are optional; explicit links remain valid authority when repository policy permits.

## Workspace port

### Purpose

Register, audit, create, own, and recover working environments.

Operations:

- register existing workspace;
- inspect identity and isolation capabilities;
- audit repository/base/head/changes;
- create managed workspace;
- list active processes where supported;
- acquire/release adapter-side resources;
- remove/recover workspace;
- report canonical path or remote reference.

W2 implements register and audit only.

The port reports enforceable isolation properties individually:

```text
filesystem_boundary
process_boundary
network_boundary
credential_boundary
resource_limits
workspace_persistence
remote_attestation
```

It must not return a generic `sandboxed=true` without defined properties.

## Command port

### Purpose

Run repository-defined named commands and later approved arbitrary commands.

Operations:

- describe command capability;
- validate command declaration;
- start command;
- stream bounded stdout/stderr/events;
- observe exit and resource use;
- request cancellation;
- confirm process termination;
- collect declared artifacts.

W2 supports synchronous invocation of one named validation command in a human-owned checkout. It records the observed result but makes no sandbox claim.

Command result includes:

- command ID and resolved invocation digest;
- workspace and working directory;
- exact head before and after;
- exit status/signal;
- duration;
- bounded/redacted output artifact;
- changed-path observation;
- cancellation state;
- limitations.

## Actor-runtime port

### Purpose

Connect model-backed agents, scripts, or remote actor services to an Execution without adopting their session model.

Operations:

- describe runtime capabilities and version;
- create/resume actor session;
- start bounded interaction;
- send accepted input;
- stream normalized activities;
- receive approval/permission requests;
- respond to supported requests;
- cancel/interrupt;
- observe terminal runtime state;
- collect artifacts and handoff;
- dispose session.

No actor-runtime implementation exists in W2.

### Codex App Server mapping

Current Codex App Server uses bidirectional message exchange with version-specific schemas, threads, turns, streamed item/turn notifications, and server-initiated approvals. Werkstatt maps:

| Codex concept | Werkstatt mapping |
|---|---|
| transport connection/initialize | AdapterSession |
| thread | provider session reference on ActorSession |
| turn | one or more interaction activities inside an Execution |
| item events and deltas | normalized Activity and Artifact updates |
| turn diff snapshot | derived diff Artifact |
| command/file approval request | OperationRequest requiring PolicyEvaluation/Approval |
| turn completed/interrupted/failed | adapter interaction result, not WorkItem acceptance |
| generated TypeScript/JSON schema | version-pinned adapter compatibility input |

Rules:

- app-server schema is generated/validated against the installed Codex version during adapter implementation;
- experimental methods/fields require explicit adapter capability negotiation;
- transport overload is retryable only within ExecutionBudget and idempotency rules;
- Codex authentication state is adapter/secret-provider concern;
- Codex approval choices cannot exceed Werkstatt policy;
- provider thread history is not the Werkstatt activity journal or project authority.

Primary reference: [Codex App Server README](https://github.com/openai/codex/blob/main/codex-rs/app-server/README.md).

### MCP mapping

MCP defines host/client/server communication, lifecycle, capability negotiation, tools, resources, prompts, and JSON-RPC transport.

Werkstatt uses MCP only through adapters:

| MCP concept | Werkstatt mapping |
|---|---|
| host/client connection | AdapterSession |
| server capability | adapter technical capability |
| tool | candidate capability operation requiring policy |
| resource | external context/artifact reference |
| prompt | generated assistance, never authority |
| elicitation/approval-like interaction | actor input or OperationRequest, subject to policy |

MCP does not define WorkItem, WriterLease, ValidationReceipt, AcceptanceRecord, or Werkstatt lifecycle state.

Primary reference: [MCP architecture](https://modelcontextprotocol.io/docs/learn/architecture).

### A2A mapping

A2A defines remote agent discovery, messages, stateful tasks, artifacts, updates, streaming, cancellation, and terminal task states.

Werkstatt maps:

| A2A concept | Werkstatt mapping |
|---|---|
| Agent Card/capabilities | remote ActorRuntime capability observation |
| A2A Task | remote execution correlation owned by adapter |
| task status | adapter runtime status mapped to Execution activity/state where valid |
| message | actor input/output Artifact or Activity |
| artifact | referenced/ingested Werkstatt Artifact |
| status/artifact update | normalized activity stream |
| cancellation | actor-runtime cancel request and observed outcome |

An A2A Task is not a Werkstatt WorkItem and its terminal status cannot accept project work.

Primary reference: [A2A specification](https://a2a-protocol.org/latest/specification/).

## Validation port

### Purpose

Observe independent validation from repository commands, CI, or future specialized validators.

Operations:

- describe validation contract;
- request validation where policy permits;
- fetch run by external reference;
- list runs for exact revision;
- verify tested revision;
- fetch steps, conclusion, diagnostics, and artifacts;
- map result into ValidationReceipt;
- report provider limitations.

W2 invokes only the local named command via CommandPort and may observe existing CI references read-only when supplied. CI dispatch belongs to later phases.

## Editor and terminal launcher ports

Purpose:

- open the human’s existing editor/IDE at workspace/path;
- open terminal at workspace;
- report launch success/failure;
- avoid pretending to own the editor session.

Launchers receive no project credentials beyond the user’s existing environment. W2 may implement platform-configurable command launchers or render the command without launching when unsupported.

## Publication port

Purpose:

- create/update branch, pull request, comments, review responses, merge, issue closure, and reconciliation operations.

It is fully designed but not implemented in W2.

Every mutation requires:

- exact expected source/head revision;
- idempotency key;
- scoped capability and policy decision;
- provider result observation;
- uncertain-result reconciliation before retry.

## Clock and identifier ports

### Clock

Supplies monotonic duration measurement and wall-clock timestamps separately. Domain tests use deterministic clocks.

### Identifier generator

Produces typed IDs. IDs must not encode provider, time, or security-sensitive data unless explicitly specified. Tests use deterministic generators.

## Storage port

### Purpose

Persist local operational state and activity without becoming external project authority.

Operations:

- open/initialize storage;
- read current entity/version;
- transact domain state changes and activity append atomically;
- query projections;
- manage retention/tombstones;
- migrate schema;
- integrity check;
- backup/restore metadata;
- close cleanly.

Domain services depend on transaction-oriented repository interfaces, not SQL.

## SQLite decision

SQLite is selected for initial local operational storage because Werkstatt is initially a single-user local application and requires transactional relational state, simple deployment, and recoverable on-disk persistence.

W1 binds these rules:

- one database file per Werkstatt user/profile by default, not per repository;
- external authority remains referenced, not copied as editable truth;
- foreign-key enforcement is enabled explicitly on every connection;
- writes use short explicit transactions;
- the application handles `SQLITE_BUSY` with bounded retry/backoff;
- current state and associated activity append commit atomically;
- schema migrations are ordered, transactional where SQLite permits, and application-owned;
- `PRAGMA application_id` identifies Werkstatt database files;
- `PRAGMA user_version` records schema version;
- integrity and foreign-key checks are part of backup/recovery diagnostics;
- database files are local-host state; network-filesystem operation is unsupported;
- WAL is an implementation option for local concurrency, not a distributed coordination mechanism.

SQLite allows multiple readers but only one write transaction at a time. WAL permits readers and a writer concurrently but still has one writer, adds WAL/shared-memory files, requires same-host shared memory, and requires checkpoint awareness. These properties reinforce short writes and local-only storage rather than actor coordination through database locking.

Primary references:

- [SQLite transactions](https://www.sqlite.org/lang_transaction.html)
- [SQLite WAL](https://www.sqlite.org/wal.html)
- [SQLite foreign keys](https://www.sqlite.org/foreignkeys.html)
- [SQLite pragmas](https://www.sqlite.org/pragma.html)

## Logical schema

The schema below is a design contract, not executable SQL.

### Metadata and migrations

#### `schema_migrations`

- version integer primary key;
- migration name;
- checksum;
- applied time;
- application version;
- result metadata.

`PRAGMA user_version` equals the highest successfully applied schema version. The table provides audit/checksum detail.

#### `application_metadata`

- singleton key/value records for created time, database ID, profile ID, last clean shutdown, and format features;
- no secret values.

### Projects and external authority

#### `projects`

- project ID;
- display name;
- lifecycle state;
- active work-source binding ID nullable;
- policy profile reference;
- created/updated timestamps;
- entity version.

#### `repository_bindings`

- repository ID;
- project ID;
- provider kind;
- canonical local path reference;
- remote identity;
- trust/isolation classification;
- current observation ID;
- entity version.

#### `work_source_bindings`

- binding ID;
- project ID;
- provider kind/instance;
- external source reference;
- authoritative flag;
- refresh cursor/token;
- synchronization state;
- current observation ID;
- entity version.

Unique constraint: at most one authoritative active work source per project.

#### `authority_observations`

- observation ID;
- project/source/resource identity;
- authority kind;
- external reference fields;
- revision kind/value/verification;
- observation time;
- freshness policy and expiry;
- synchronization state;
- normalized payload artifact/reference;
- previous observation ID;
- conflict ID nullable.

Observations are immutable except retention/tombstone metadata.

#### `authority_conflicts`

- conflict ID;
- affected project/work item;
- fact kind;
- competing observation IDs;
- owner authority references;
- status/disposition;
- created/resolved times.

### Work and contracts

#### `programs`

- program ID;
- project ID;
- authority observation ID;
- external reference;
- projection fields;
- entity version.

#### `phases`

- phase ID;
- program ID;
- stable phase key;
- owner work item ID nullable;
- projection fields;
- entity version.

#### `phase_dependencies`

- phase ID;
- depends-on phase ID;
- dependency kind.

#### `work_items`

- work item ID;
- project/source binding;
- external reference;
- current snapshot ID;
- entity version.

#### `work_snapshots`

- snapshot ID;
- work item ID;
- authority observation ID;
- normalized state/stage/class/owner/relations;
- contract ID nullable;
- created time.

Immutable.

#### `work_contracts`

- contract ID;
- work item ID;
- source snapshot ID;
- contract status;
- accepted base revision;
- scope/non-goals/validation/review/stop/gate structured payload references;
- normalization version;
- created time;
- supersedes contract ID nullable.

Immutable after creation.

#### `requirements`

- contract ID;
- stable requirement key;
- kind;
- statement;
- owner;
- required evidence categories;
- structured metadata.

### Actors, roles, policies, approvals

#### `actors`

- actor ID;
- kind;
- display name;
- provider reference nullable;
- status;
- entity version.

No secret credentials.

#### `actor_sessions`

- session ID;
- actor ID;
- adapter/provider identity;
- external session reference;
- start/end and state;
- capability observation reference.

#### `role_assignments`

- assignment ID;
- actor ID;
- role;
- project/work scope;
- source authority;
- valid interval;
- status.

#### `policy_profiles`

- profile ID;
- scope;
- policy version;
- structured rules artifact;
- status;
- created/superseded metadata.

#### `policy_evaluations`

- evaluation ID;
- operation request digest;
- policy source/version set;
- facts/revisions digest;
- decision;
- reason codes;
- validity interval;
- created time.

#### `approvals`

- approval ID;
- request digest;
- actor/approver/role;
- scope and revision digest;
- decision;
- expiry/reuse/consumed status;
- policy source;
- times.

### Workspaces, leases, and executions

#### `workspaces`

- workspace ID;
- repository ID;
- kind;
- canonical location reference;
- base/head observations;
- isolation capabilities;
- trust class;
- state;
- latest audit ID;
- entity version.

#### `workspace_audits`

- audit ID;
- workspace ID;
- repository/base/head/branch/dirty/operation observations;
- adapter version;
- result;
- time.

Immutable.

#### `writer_leases`

- lease ID;
- workspace/execution/actor/session/work item;
- expected revision;
- state;
- acquired/heartbeat/expiry/release times;
- release/conflict/recovery reason;
- entity version.

Partial unique constraint: one active/expiring lease per workspace.

#### `executions`

- execution ID;
- work item/contract/workspace/actor/session/role;
- policy snapshot/evaluation;
- expected base/head;
- state and terminal reason;
- budgets;
- start/end;
- retry-of execution ID;
- handoff ID nullable;
- entity version.

#### `activities`

- activity ID;
- execution ID;
- sequence number;
- kind;
- actor/role;
- operation/policy/approval references;
- start/end/outcome;
- structured redacted payload reference;
- sensitivity/retention.

Unique `(execution_id, sequence_number)`.

#### `handoffs`

- handoff ID;
- source execution;
- target actor/role nullable;
- authority/contract/workspace/head snapshot;
- summary/artifact/evidence/finding references;
- next action;
- created time.

### Artifacts, evidence, findings, reviews, acceptance

#### `artifacts`

- artifact ID;
- work/execution/activity;
- kind/classification;
- external reference or content-store reference;
- content digest/size/type;
- source revisions;
- sensitivity/retention;
- stale/supersession state;
- created time.

Large payloads are stored outside relational rows through an artifact store or file reference.

#### `claims`

- claim ID;
- contract/requirement/review dimension;
- statement;
- subject revision/environment;
- assessment/invalidation rules.

#### `evidence`

- evidence ID;
- claim/work/execution;
- category/result/independence;
- producer and source;
- exact subject revision/environment;
- procedure identity;
- artifact/diagnostic reference;
- limitations;
- freshness/invalidation state;
- time.

#### `findings`

- finding ID;
- source review/evidence;
- category/severity/status;
- statement;
- affected scope/revision;
- owner;
- resolution evidence/disposition;
- times;
- entity version.

#### `reviews`

- review ID;
- contract and exact subject revision set;
- reviewer/role;
- state/verdict;
- dimensions;
- evidence snapshot;
- created/completed times;
- entity version.

#### `review_findings`

- review ID;
- finding ID.

#### `acceptance_records`

- acceptance ID;
- work item/contract;
- acceptor or external authority;
- exact accepted artifact/revision;
- source reference;
- validation/review references;
- limitations/risk decisions;
- acceptance time;
- immutable.

#### `reconciliations`

- reconciliation ID;
- work item/acceptance/execution/workspace;
- state;
- required/completed operations;
- blockers/conflicts;
- created/completed times;
- entity version.

### Journal and local settings

#### `activity_journal`

- journal sequence integer primary key;
- event kind and schema version;
- entity kind/ID/version;
- project/execution correlation;
- redacted bounded payload;
- sensitivity/retention;
- timestamp.

#### `local_preferences`

- profile/project scope;
- namespaced key;
- structured value;
- version/update time.

No project-authoritative facts or secrets.

## Transaction boundaries

### Atomic domain transition

One write transaction should:

1. verify expected entity versions;
2. insert immutable records;
3. update current entity/projection rows;
4. append associated activity-journal events;
5. commit.

No external network or process call occurs inside the SQLite transaction.

### External operation pattern

```text
transaction A
    record pending operation and idempotency key
commit

external adapter call

transaction B
    record confirmed result or uncertain outcome
    apply domain transition if preconditions still match
    append journal
commit
```

On crash between call and confirmation, recovery observes the external source before retrying.

### Writer contention

- writes use short explicit transactions;
- `BEGIN IMMEDIATE` may be used for operations that must reserve the sole writer predictably;
- busy timeout/backoff is bounded;
- UI never blocks indefinitely;
- write contention is operational, not a project lease;
- long-running work and actor streams never hold a database transaction.

## Journal mode decision

W1 permits WAL for the local application database when supported and tested.

Required handling:

- confirm `PRAGMA journal_mode=WAL` actually returns `wal`;
- same-host local filesystem only;
- keep database, `-wal`, and `-shm` files together for backup/copy semantics;
- use bounded connections and short readers;
- retain default auto-checkpoint initially unless measurements justify change;
- expose checkpoint/large-WAL diagnostics;
- fall back to rollback journal where WAL is unsupported;
- do not let journal-mode differences change domain semantics.

W2 may use the library default journal mode because its workload is small and single-process. WAL adoption can be deferred to measured need, but schema/design must support concurrent UI reads and one writer later.

## Foreign keys and integrity

On every connection:

- explicitly enable foreign keys;
- verify the setting;
- use constraints for owned entity relations;
- avoid cascade deletes that can erase audit/evidence unexpectedly;
- use restrictive deletes plus explicit retention workflows;
- run `foreign_key_check` and `quick_check`/`integrity_check` during diagnostics and backup verification.

Unknown failure to enable foreign keys is fatal for writable operation.

## Application ID and schema version

- assign a registered/non-conflicting 32-bit `application_id` before implementation release;
- schema version starts at 1 in W2;
- `user_version` changes only after a migration commits successfully;
- application refuses writable open when application ID is wrong or schema is newer than supported;
- older supported schema migrates through the exact ordered chain;
- read-only diagnostic mode may open incompatible databases without mutation where safe.

## Migration policy

Each migration has:

- monotonically increasing integer version;
- stable name;
- SQL/operation checksum;
- minimum/maximum application compatibility;
- forward transformation;
- verification queries;
- documented rollback/recovery strategy;
- tests from every supported prior version;
- size/time risk classification.

Rules:

1. Backup recommendation or automatic backup policy is explicit before risky migration.
2. Migration obtains exclusive application-level database ownership.
3. Migration records intent before external file operations if any.
4. Schema changes and `schema_migrations` entry commit atomically where possible.
5. `user_version` updates last in the same successful transaction.
6. Failed migration leaves the prior version or a detectable recovery state.
7. Destructive data migrations require a separate accepted product decision.
8. Downgrade is not assumed; older binaries fail safely on newer schema.

W2 implements only initial schema creation and version verification, not upgrade history beyond test fixtures.

## Backup and recovery

Backup requirements:

- use SQLite backup API or a transactionally safe method rather than copying an active database file alone;
- include WAL state correctly when WAL is active;
- record application ID, schema version, application version, and backup time;
- verify integrity and foreign keys on restored copy;
- never treat local backup as backup of external Git/GitHub authority;
- artifact payload storage has a coordinated backup/retention policy.

Recovery sequence:

1. open database in diagnostic mode;
2. verify application ID and supported schema;
3. run integrity and foreign-key checks;
4. inspect last-clean-shutdown and pending operations;
5. repair or restore storage before running mutations;
6. refresh external authority and actual workspaces;
7. reconcile nonterminal executions, leases, approvals, and pending operations;
8. preserve uncertain history and require owner resolution where external effects are ambiguous.

## Data minimization

Do not store by default:

- repository file contents already addressable by Git revision;
- full issue/PR bodies beyond bounded observation payload needed for offline display;
- secrets or inherited environments;
- private chain-of-thought;
- unbounded command/model streams;
- binary artifacts inline in SQLite;
- duplicate roadmap/Project state as editable fields.

Store references, digests, normalized facts, bounded summaries, and retention-aware artifacts.

## W2 adapter and storage subset

Adapters:

- Git repository inspection;
- read-only GitHub issue import or repository-local work document;
- existing checkout registration/audit;
- named validation process invocation;
- editor/terminal command rendering or launching;
- local filesystem artifact output;
- SQLite storage;
- system clock and ID generator.

Storage:

- project/repository/work-source registration;
- authority observations and work snapshots/contracts;
- human workspace and execution records;
- activities, evidence, findings, review-packet artifact, and reconciliation checklist;
- schema metadata and local preferences.

Excluded from W2:

- actor runtime;
- approvals/policy enforcement beyond manual capability display and invariant checks;
- leases beyond human ownership assertion;
- GitHub mutations;
- CI dispatch;
- secret/network management;
- remote artifact store;
- WAL tuning;
- multi-process coordination.
