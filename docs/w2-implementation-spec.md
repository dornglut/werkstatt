# W2 Implementation Specification

## Status

Proposed W1 implementation contract for W2. This document authorizes no code until W1 is accepted and a separate W2 issue is created.

## Objective

Implement one Rust package and one synchronous CLI that proves the accepted human-first workflow without model or autonomous execution dependencies.

W2 must be small enough to review as a coherent product slice and complete enough to execute the comparative human pilot in [W2 CLI contract](w2-cli-contract.md).

## Accepted architecture

```text
src/cli
    parses commands and renders human/JSON output

src/application
    coordinates use cases and transactions

src/domain
    owns types, invariants, transitions, and errors

src/ports
    defines repository, work-source, workspace, command, storage,
    launcher, clock, and identifier contracts

src/adapters
    implements Git CLI, local/GitHub-through-gh work sources,
    process validation, SQLite, filesystem artifacts, system launchers,
    clock, and identifiers
```

Dependency direction:

```text
cli -> application -> domain
                 -> ports
adapters -> ports/domain/application contracts
composition root -> all concrete modules
```

No domain module imports an adapter or CLI type.

## Repository shape

```text
Cargo.toml
Cargo.lock
rust-toolchain.toml
src/
  lib.rs
  main.rs
  cli/
    mod.rs
    args.rs
    output.rs
  domain/
    mod.rs
    ids.rs
    authority.rs
    project.rs
    work.rs
    workspace.rs
    execution.rs
    evidence.rs
    policy.rs
    error.rs
  application/
    mod.rs
    project_service.rs
    work_service.rs
    workspace_service.rs
    execution_service.rs
    validation_service.rs
    review_service.rs
    reconciliation_service.rs
    doctor_service.rs
  ports/
    mod.rs
    repository.rs
    work_source.rs
    workspace.rs
    command.rs
    storage.rs
    launcher.rs
    clock.rs
    identifiers.rs
  adapters/
    mod.rs
    git_cli.rs
    local_work_source.rs
    github_gh.rs
    process_command.rs
    sqlite.rs
    filesystem_artifacts.rs
    system_launcher.rs
    system_clock.rs
    uuid_identifiers.rs
  storage/
    schema_v1.sql
    queries.rs
  work_source/
    markdown.rs
    json.rs
  review/
    markdown.rs
    json.rs
tests/
  cli_contract.rs
  domain_transitions.rs
  git_adapter.rs
  work_source_parser.rs
  sqlite_storage.rs
  acceptance_manual_fixture.rs
fixtures/
  repository/
  work-items/
```

This is a planning map. Implementation may combine very small files when doing so preserves ownership and the 131,072-byte limit. It must not split into multiple crates in W2.

## Dependency plan

The W2 dependency set should remain intentionally small.

### Runtime dependencies

| Dependency class | Proposed crate/approach | Reason | Constraint |
|---|---|---|---|
| CLI parsing | `clap` derive | mature command/help/error surface | no business logic in derive metadata |
| Serialization | `serde`, `serde_json` | versioned JSON output, artifacts, structured fields | provider payloads remain adapter types |
| SQLite | `rusqlite` with bundled SQLite | predictable cross-platform local DB and transaction API | domain does not expose SQL types |
| Errors | `thiserror` | stable typed error implementation | public CLI error codes owned separately |
| IDs | `uuid` with v7/random support | opaque typed local identities | wrap in typed IDs; no raw UUID domain mixing |
| Time | `time` | RFC3339 wall time and durations | clock port supplies values to domain |
| Data directories | `directories` or equivalent maintained crate | platform-correct local profile path | overridable with `--data-dir` |
| Digests | `sha2` | source/artifact/request digests | not used as authorization or signature |
| URLs | `url` | parse and normalize source/navigation locations | external IDs remain opaque |
| Shell-free argument parsing | no shell crate | argv stored/executed directly | never evaluate a command string |

### Development dependencies

| Dependency class | Proposed crate/approach | Purpose |
|---|---|---|
| Temporary repositories/data | `tempfile` | isolated fixtures |
| CLI integration | `assert_cmd` and `predicates` or direct process harness | command/output/exit-code tests |
| Property testing | optional `proptest` only where transition/parser invariants benefit | bounded generative tests |

### Explicit exclusions

W2 adds no:

- async runtime;
- HTTP client;
- Git library or libgit2 binding;
- GitHub API SDK;
- model/agent SDK;
- MCP/A2A implementation;
- templating engine;
- ORM;
- event-sourcing framework;
- logging/telemetry service dependency;
- GUI dependency;
- plugin framework.

Any dependency addition beyond this plan requires justification in the W2 issue/PR.

## Git adapter decision

W2 uses the installed Git executable rather than a Git library.

Reasons:

- matches the human’s actual repository tooling;
- supports worktree and repository behavior without libgit2 compatibility gaps;
- avoids native library dependency and duplicate Git semantics;
- stable porcelain forms exist for status and worktree listing;
- W2 is inspection-only.

Required invocations use fixed argv and disabled optional user behaviors where relevant.

Examples of information required:

- repository/common directory;
- exact `HEAD`;
- symbolic branch;
- status in porcelain v2 with NUL separation;
- remotes;
- ancestry/merge-base checks;
- worktree list in porcelain format;
- diff statistics and optional bounded diff output;
- ongoing Git operation state.

Rules:

- no shell;
- set working directory explicitly;
- bound output/time;
- use `--no-optional-locks` for read-only inspection where supported;
- disable external diff/textconv where deterministic inspection requires it;
- preserve non-UTF-8 path bytes internally or report unsupported rendering without lossy authority decisions;
- redact private absolute paths in public output.

## Work-source adapters

### Local Markdown/JSON

- parse bounded files only;
- preserve raw source as an artifact reference/digest;
- normalize known contract fields;
- reject duplicate material fields;
- report incomplete contract rather than infer;
- detect digest/mtime movement;
- permit JSON round-trip with explicit schema version.

### GitHub through `gh`

- optional adapter;
- detect executable and version;
- invoke a fixed `gh issue view <url> --json ...` argv;
- no shell or mutation command;
- no credential storage or token output;
- bounded JSON size;
- map authentication/network/provider errors distinctly;
- store source reference, observed fields, digest, and time;
- do not claim immutable issue revision when provider supplies only mutable observations;
- allow export-to-file fallback.

W5 replaces this minimal subprocess adapter with richer lifecycle integration where justified.

## SQLite implementation contract

### Open sequence

1. resolve/create local profile directory with restrictive platform-appropriate permissions where possible;
2. open SQLite connection;
3. set/check application ID;
4. explicitly enable and verify foreign keys;
5. inspect `user_version`;
6. initialize schema v1 or reject unsupported future schema;
7. run lightweight integrity diagnostics after unclean shutdown;
8. record open/session metadata;
9. never expose raw database errors containing private paths in public output.

### Schema v1

Implement the W2 subset of [Ports and storage](ports-and-storage.md):

- metadata/migrations;
- projects and repository/work-source bindings;
- authority observations;
- work items/snapshots/contracts/requirements;
- actors/human session/role assignment;
- workspaces/audits;
- executions/activities/handoffs;
- artifacts/claims/evidence/findings/reviews;
- reconciliation;
- activity journal;
- local preferences.

Tables for later policy, automated leases, approvals, and actor runtimes may be omitted from schema v1 if domain serialization and future migration ownership remain explicit. Do not create empty speculative tables solely for symmetry.

### Transactions

- use explicit short transactions for domain state and journal append;
- external Git/`gh`/process calls occur outside transactions;
- use optimistic entity version checks;
- use bounded busy timeout/retry;
- rollback on any invariant or persistence failure;
- surface uncertain external outcomes separately;
- tests inject failures before and after commit boundaries.

### Journal mode

W2 may use SQLite default rollback journal. WAL is not required for the single-process proof. The implementation records the actual journal mode and remains compatible with later migration to WAL.

### Artifacts

- bounded text/JSON artifacts may use filesystem files under the profile directory;
- SQLite stores metadata, digest, relative path, sensitivity, and retention;
- atomic temp-write/rename for artifact files;
- database transaction records artifact only after file creation succeeds;
- orphan cleanup is diagnostic and conservative;
- large command output is truncated with explicit marker and original-size estimate where available.

## Application use cases

### Register project

Inputs:

- repository path;
- name;
- expected remote optional;
- validator argv optional;
- work-source kind.

Process:

1. inspect Git repository;
2. validate expected identity;
3. validate configured executable without running it;
4. persist project/bindings/observation atomically;
5. render limitations and next action.

### Import/refresh work

1. call selected read-only work-source adapter;
2. record raw bounded artifact/digest;
3. normalize WorkSnapshot;
4. derive immutable WorkContract;
5. classify executable/incomplete/stale/conflicted;
6. persist observation/snapshot/contract atomically;
7. leave active executions bound to prior contracts;
8. render changes and next action.

### Register/audit workspace

1. inspect Git identity/head/branch/status/operation state;
2. compare project repository and contract accepted base;
3. classify expected/unexpected changes;
4. persist workspace and immutable audit;
5. invalidate affected derived evidence after head movement;
6. render no-sandbox limitation.

### Start manual execution

1. require fresh executable contract;
2. require compatible workspace audit;
3. reject conflicting nonterminal local execution;
4. create human actor/session/role if absent;
5. create execution and initial activities;
6. record human ownership assertion;
7. render next action.

### Run validation

1. require active manual execution and compatible workspace/head;
2. resolve stored argv;
3. record pending activity;
4. run process outside DB transaction;
5. capture bounded output and head-before/head-after;
6. write artifact/evidence/activity in transaction;
7. classify result and invalidate on head movement;
8. render exact limitations.

### Prepare review

1. refresh workspace;
2. compute base/head/diff summary;
3. load contract/requirements/evidence/findings;
4. calculate readiness without semantic code claims;
5. render Markdown/JSON packet;
6. persist generated artifact and source revisions;
7. supersede prior packet projection.

### Reconcile

1. refresh work source and repository observations where configured;
2. accept explicit manual external outcome input;
3. validate exact accepted revision when supplied;
4. mark execution/handoff/reconciliation local state;
5. invalidate obsolete evidence/approvals;
6. never mutate external source;
7. render remaining authoritative/manual actions.

## Structured error implementation

Public error object:

```text
code
operation
message
retryable
correction
context (safe fields)
source (optional adapter code)
```

Requirements:

- stable domain codes from [Work domain](work-domain.md);
- adapter error codes mapped without losing diagnostics;
- human output concise;
- JSON output machine-readable;
- no panics for expected input/provider/storage errors;
- no secret, token, raw environment, or unredacted home-path leakage;
- backtrace only under explicit developer diagnostics and never default public output.

## Validation command

W2 updates the repository canonical validator to one command, likely:

```text
cargo run --locked --bin xtask -- validate
```

or a similarly explicit repository-owned entrypoint selected during implementation planning.

Because W2 begins as one package, a lightweight Rust `xtask` module/binary is allowed only if it provides durable validation value and does not become workflow machinery. An alternative checked-in Python validation wrapper may continue to orchestrate documentation plus Cargo commands. The W2 issue must choose exactly one canonical command before code lands.

Required implementation validation categories:

- formatting;
- locked build/tests;
- strict linting;
- documentation/link/file-size/current-authority checks;
- CLI contract tests;
- SQLite integrity/foreign-key/migration tests;
- exact-head CI;
- clean generated-state/diff proof where applicable.

## Test strategy

### Domain unit tests

- typed IDs do not interchange;
- all allowed/forbidden state transitions;
- terminal execution retry creates new ID;
- stage/state independence;
- contract completeness/readiness;
- evidence invalidation on revision movement;
- acceptance cannot derive from execution success;
- approval/policy types are representable but not enforced in W2;
- structured errors contain correction and safe context.

### Parser tests

- valid Dornglut Markdown contract;
- missing/duplicate/ambiguous fields;
- additional repository-specific sections;
- malformed tables;
- bounded file size;
- JSON schema/version round-trip;
- non-UTF-8 source failure;
- source digest movement.

### Git adapter tests

Using temporary real Git repositories:

- repository identity and remote observation;
- branch/detached HEAD;
- clean/modified/untracked/renamed paths;
- nontrivial ancestry and wrong base;
- worktree listing;
- ongoing merge/rebase/cherry-pick state;
- head movement between audits;
- external diff disabled;
- path redaction;
- Git unavailable/incompatible diagnostics.

### SQLite tests

- new database initialization/application ID/user version;
- foreign keys enabled;
- transaction rollback on injected failure;
- state+journal atomicity;
- optimistic version conflict;
- busy/timeout behavior;
- integrity/foreign-key diagnostics;
- unclean-shutdown marker;
- unsupported future schema fails writable open;
- artifact orphan diagnostics;
- local project removal does not touch repository/external source.

### CLI contract tests

- help and command hierarchy;
- global human/JSON formats;
- exit-code mapping;
- project add/show/list/remove;
- work import/show/refresh/packet;
- workspace register/status/forget;
- execution start/status/cancel/handoff;
- validate run/record/show;
- finding add/resolve;
- review prepare/show;
- reconcile inspect/complete;
- doctor/export;
- error correction text;
- no ANSI under `--no-color`/JSON;
- bounded output and path redaction.

### Acceptance fixtures

Fixture repository contains:

- canonical validation command;
- accepted base and task branch;
- representative Markdown work contract;
- clean and changed states;
- passing/failing validator modes;
- simulated moved head;
- simulated external acceptance snapshot.

Fixture tests prove the full ordinary path without GitHub/network/model dependencies.

## W2 acceptance matrix

| ID | Requirement | Proof | Pass condition |
|---|---|---|---|
| W2-H01 | useful without a model | full CLI fixture and human pilot | no model/API key/runtime configured |
| W2-H02 | existing tools preserved | editor/terminal launch/render and manual Git flow | user can use preferred tools; Werkstatt does not edit source |
| W2-A01 | external authority visible | work import/show output | source, observation, revision/freshness always shown |
| W2-A02 | no duplicate editable work state | storage/schema/API review | imported work facts cannot be edited as authority |
| W2-A03 | stale source fails safely | refresh/moved-source tests | execution or packet marked stale/conflicted |
| W2-W01 | repository identity proven | Git adapter tests | wrong repo/remote/base rejected with correction |
| W2-W02 | workspace has no false sandbox claim | CLI output test | registered checkout limitation always visible |
| W2-W03 | head movement invalidates evidence | integration test | old diff/validation/review packet marked stale |
| W2-E01 | manual execution distinct from work state | domain/CLI tests | succeeded execution does not accept work item |
| W2-E02 | command evidence honest | pass/fail/unavailable tests | local result labeled executor-observed; unavailable never passes |
| W2-E03 | independent evidence representable | validate record tests | exact subject and verification state retained |
| W2-R01 | review packet revision-bound | packet tests | contract/base/head/source revisions present |
| W2-R02 | blocking findings block readiness | finding/review tests | readiness false until valid resolution/new review |
| W2-R03 | generated packets remain derived | regeneration tests | source unchanged; prior packet superseded/stale |
| W2-C01 | no shell command injection | argv/process tests | command stored/executed as argv only |
| W2-C02 | sensitive data minimized | redaction/export tests | no tokens/env/home paths in default public output |
| W2-S01 | storage transactional | failure-injection tests | no partial entity/journal transition |
| W2-S02 | storage identifiable/versioned | open/migration tests | application ID/schema rules enforced |
| W2-S03 | external authority survives local deletion | project remove test | repository/source untouched |
| W2-U01 | errors actionable | CLI/error review | cause, blocked operation, and correction present |
| W2-U02 | ordinary path concise | usability observation | common task requires no actor-policy/autonomy configuration |
| W2-P01 | comparative pilot shows net value | dual-run pilot report | context/evidence/review benefit exceeds added steps |
| W2-P02 | no product-scope creep | PR and dependency audit | no agents, GitHub writes, arbitrary commands, GUI, scheduler, or merge |

## Human pilot protocol

### Preparation

- create an accepted issue authorizing the comparison;
- choose a bounded documentation change;
- record repository/base/validator;
- prepare two isolated checkouts from the same base;
- define identical completion and review criteria;
- decide which result may be published.

### Baseline run

Human uses normal repository docs, Git, editor, terminal, and GitHub without Werkstatt.

Record:

- start/end times by step;
- sources opened;
- copied revisions/state;
- mistakes/rework;
- evidence preparation;
- subjective friction.

### Werkstatt run

Human uses Werkstatt for project/work/workspace/validation/review/reconciliation coordination while still editing and using Git manually.

Record the same metrics plus Werkstatt operations.

### Evaluation

Pass requires:

- no reduction in correctness/reviewability;
- fewer or easier authority/state reconstruction steps;
- materially easier validation/review handoff;
- no hidden duplicate authority;
- no requirement for a model;
- no unacceptable setup or bookkeeping burden;
- documented weaknesses and changes required before W3.

A neutral or negative result may still complete W2 technically but blocks W3 until product direction is revised.

## Delivery decomposition

W2 should be one umbrella/owning issue with bounded implementation PRs only when each is independently reviewable and the sequence does not create parallel incomplete APIs.

Recommended slices:

1. package/domain/storage foundation;
2. Git and work-source read adapters;
3. project/work/workspace CLI ordinary path;
4. execution/validation/evidence/review/reconciliation;
5. acceptance fixtures and human pilot;
6. documentation/closeout reconciliation.

A single PR is acceptable if the resulting diff remains reviewable. Do not create process-only activation PRs.

## Completion and next phase

W2 completes when:

- canonical validation and exact-head CI pass;
- all W2 matrix rows have evidence/verdict;
- the human comparative pilot is published;
- accepted-main validation passes;
- roadmap/issue/product maturity are reconciled;
- no W3 command-policy/lease/orchestration implementation entered scope;
- the owner explicitly decides whether evidence authorizes W3.
