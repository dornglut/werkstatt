# W2 Implementation Specification

## Status

Proposed W1 implementation contract for W2. It authorizes no code until W1 is accepted and a separate W2 issue is created.

## Objective

Implement one Rust package and one synchronous CLI that proves the accepted human-first workflow without model, autonomous, GitHub-write, or graphical dependencies.

The implementation must satisfy [W2 CLI contract](w2-cli-contract.md) while remaining small enough to review and change after the comparative pilot.

## Architectural shape

```text
CLI
    -> application use cases
        -> domain and ports
            <- concrete adapters
```

Dependency direction:

```text
cli -> application -> domain
                 -> ports
adapters -> port/domain contracts
composition root -> concrete application
```

Domain code imports no adapter, SQL, Git, process, provider, or CLI types.

## One-package repository shape

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
    work.rs
    workspace.rs
    execution.rs
    evidence.rs
    error.rs
  application/
    mod.rs
    service.rs
    projections.rs
  ports/
    mod.rs
    repository.rs
    work_source.rs
    command.rs
    storage.rs
    launcher.rs
    clock.rs
  adapters/
    mod.rs
    git_cli.rs
    local_work_source.rs
    github_gh.rs
    process_command.rs
    sqlite.rs
    filesystem_artifacts.rs
    system_launcher.rs
  storage/
    schema_v1.sql
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
  acceptance_fixture.rs
fixtures/
  repository/
  work-items/
```

Small modules may be combined when ownership remains clear. W2 must not split into multiple crates.

## Dependency plan

### Runtime

| Need | Choice | Constraint |
|---|---|---|
| CLI | `clap` derive | parsing/help only; no domain decisions in attributes |
| JSON | `serde`, `serde_json` | versioned public output and local artifacts |
| SQLite | `rusqlite` with bundled SQLite | SQL remains inside adapter |
| Errors | `thiserror` | stable public codes owned by domain/CLI contract |
| IDs | `uuid` | wrapped typed IDs; raw UUIDs do not interchange |
| Time | `time` | values supplied through clock port |
| Data directory | `directories` or maintained equivalent | overridable by `--data-dir` |
| Digests | `sha2` | identity/change detection, not authorization/signature |
| URLs | `url` | navigation/source parsing; external IDs remain opaque |

### Development

- `tempfile` for isolated repositories and databases;
- `assert_cmd` plus `predicates`, or an equivalent direct CLI harness;
- optional `proptest` only for high-value parser/transition invariants.

### Excluded

No async runtime, HTTP client, Git library, GitHub SDK, model SDK, MCP/A2A implementation, ORM, event-sourcing framework, templating engine, GUI, plugin system, or telemetry service.

Any extra dependency requires explicit W2 issue/PR justification.

## Exact canonical validation decision

W2 retains and extends the existing repository-owned command:

```text
python scripts/validate.py
```

It remains the only canonical validation entrypoint for local and CI use.

W2 extends the script to run, in a deterministic order:

1. existing authority, UTF-8, newline, whitespace, link, file-size, workflow-pin, and required-file checks;
2. `cargo fmt --all --check`;
3. `cargo test --workspace --locked`;
4. `cargo clippy --workspace --all-targets --locked -- -D warnings`;
5. any bounded fixture/generated-state check required by the implementation.

Rules:

- no parallel competing gate or `xtask` is added in W2;
- CI invokes only `python scripts/validate.py` through the existing pinned reusable workflow;
- focused Cargo commands may support iteration but do not replace the baseline;
- diagnostics remain bounded and actionable;
- validation is read-only.

## Git adapter

W2 uses the installed Git executable rather than libgit2.

Reasons:

- matches actual human repository behavior;
- supports worktrees and modern Git semantics directly;
- avoids native library/version divergence;
- stable porcelain formats exist;
- W2 is inspection-only.

Required read operations:

- repository/common directory identity;
- exact HEAD and symbolic branch;
- remotes;
- ancestry/merge-base checks;
- status porcelain v2 with NUL separation;
- worktree list porcelain;
- ongoing merge/rebase/cherry-pick/bisect state;
- diff statistics and bounded diff;
- changed/untracked path audit.

Rules:

- fixed argv, never shell;
- explicit working directory;
- bounded time/output;
- disable optional locks for inspection where supported;
- disable external diff/text conversion when deterministic output requires it;
- do not make authority decisions from lossy path rendering;
- redact private absolute paths in default/public output.

## Work-source adapters

### Local file

- bounded Markdown or versioned JSON;
- preserve digest and raw bounded artifact;
- normalize known contract fields;
- report duplicates, ambiguity, and missing material fields;
- never infer missing decisions;
- active Execution binds to immutable contract version.

### GitHub through `gh`

Optional, read-only adapter:

- fixed `gh issue view <url> --json ...` argv;
- existing human authentication and network environment;
- no token storage or output;
- no GitHub mutation;
- bounded JSON response;
- source identity, observation time, digest, and limitations retained;
- mutable observation is not described as immutable issue revision;
- exported file fallback when `gh` is unavailable.

W5 owns richer GitHub lifecycle integration.

## Minimal SQLite schema v1

W2 deliberately avoids materializing the entire future domain as tables.

Required tables:

| Table | Purpose |
|---|---|
| `schema_migrations` | migration version/checksum/application version |
| `application_metadata` | database ID, profile ID, clean-shutdown marker |
| `projects` | local project registration and current bindings |
| `repository_bindings` | repository identity, remote, trust, current observation |
| `work_source_bindings` | one authoritative source per project and refresh state |
| `authority_observations` | immutable source/revision/freshness/normalized snapshot records |
| `work_items` | local work identity, current observation, immutable contract JSON reference |
| `workspaces` | registered checkout and current audit JSON/reference |
| `executions` | manual attempt, contract/workspace snapshot, state, terminal reason |
| `activities` | ordered bounded execution observations |
| `artifacts` | metadata/digest/path for packets and command output |
| `evidence` | category/result/independence/exact subject/limitations |
| `findings` | category/severity/status/resolution reference |
| `reconciliations` | local external-outcome and cleanup checklist |
| `activity_journal` | append-only domain/application event summary |
| `local_preferences` | non-authoritative local settings |

Programs, phases, requirements, actors, roles, policies, approvals, leases, claims, reviews, acceptances, and handoffs remain domain/serialized projection concepts in W2 unless a concrete W2 use case requires a column or JSON field. W3+ migrations create normalized tables only when enforcement/query needs justify them.

This prevents the first proof from becoming a workflow database before it proves value.

## SQLite rules

### Open

1. resolve/create local profile directory;
2. open database;
3. set/check Werkstatt `application_id`;
4. explicitly enable and verify foreign keys;
5. inspect `user_version`;
6. create schema v1 or reject unsupported future schema;
7. inspect unclean-shutdown marker and run diagnostics when needed;
8. never expose unredacted private paths in default errors.

### Transactions

- short explicit write transactions;
- state update and associated activity-journal append commit atomically;
- Git, `gh`, process, and filesystem calls occur outside database transactions;
- optimistic entity/version checks prevent lost local updates;
- bounded busy timeout/retry;
- no long-running read cursor across human interaction;
- uncertain external effects are observed before retry.

### Journal mode

W2 may use SQLite’s default rollback journal. WAL is not required for the single-process proof. The actual mode is recorded and later change to WAL must not alter domain semantics.

### Artifacts

- bounded text/JSON artifacts stored under profile directory;
- SQLite stores relative path, digest, size, type, sensitivity, and retention;
- atomic temporary write/rename;
- orphan diagnostics conservative;
- command output truncation explicit;
- no secrets or private chain-of-thought.

### Migrations

- schema version starts at 1;
- `user_version` changes only after successful migration;
- migration checksum recorded;
- newer unsupported schema refuses writable open;
- initial W2 implements creation plus future-version refusal, not speculative downgrade;
- destructive migration requires later accepted decision.

## Application use cases

### Guided start

1. inspect repository;
2. import work source;
3. derive immutable WorkContract;
4. audit workspace/base/head;
5. configure validator argv;
6. persist project/source/work/workspace atomically in short transitions;
7. create manual Execution;
8. render status and next action.

### Refresh/status

1. read external source and repository outside DB transaction;
2. normalize observations;
3. compare prior revisions;
4. persist new immutable observations/current pointers;
5. invalidate affected evidence/packet projections;
6. render authoritative/derived/stale/conflict distinctions.

### Validate

1. require compatible active manual execution/workspace;
2. record pending activity;
3. run stored argv outside transaction;
4. capture bounded output and head before/after;
5. persist artifact, activity, and executor-observed evidence atomically;
6. mark stale/conflicted when revision moved;
7. render result without claiming independent CI.

### Review

1. refresh workspace;
2. compute base/head/diff summary;
3. load contract/evidence/findings;
4. generate Markdown/JSON packet;
5. persist artifact/source revisions;
6. supersede prior packet projection;
7. render readiness and missing evidence.

### Reconcile

1. refresh sources when available;
2. receive explicit external outcome;
3. verify accepted revision when possible;
4. update execution/reconciliation local state;
5. mark derived/evidence history truthfully;
6. list remaining manual external actions;
7. mutate no external source.

## Public error contract

Every expected failure returns:

```text
code
operation
message
retryable
correction
safe context
optional adapter diagnostic
```

Requirements:

- stable families from [Work domain](work-domain.md);
- no panic for expected input/provider/storage failures;
- concise human output and machine-readable JSON;
- no token, raw environment, secret, private home path, or unbounded provider payload;
- backtrace only in explicit developer diagnostics.

## Test strategy

### Domain

- typed IDs cannot interchange;
- allowed/forbidden state transitions;
- terminal retry creates new Execution;
- stage/state independence;
- contract completeness;
- evidence invalidation on revision movement;
- execution success cannot accept work;
- errors include correction and safe context.

### Work-source parser

- valid Dornglut Markdown;
- missing/duplicate/ambiguous fields;
- extra repository sections;
- malformed table;
- bounded size/non-UTF-8;
- JSON schema round-trip;
- source digest movement.

### Git adapter

Temporary real repositories cover:

- identity/remotes;
- branch/detached head;
- clean/modified/untracked/renamed paths;
- ancestry/wrong base;
- linked worktree listing;
- merge/rebase/cherry-pick state;
- moved head;
- disabled external diff;
- path redaction;
- unavailable Git.

### SQLite

- initial application ID/user version;
- foreign keys enabled;
- rollback on injected failure;
- state+journal atomicity;
- optimistic conflict;
- busy timeout;
- integrity/foreign-key diagnostics;
- unclean-shutdown handling;
- unsupported future schema;
- artifact orphan diagnostics;
- project removal leaves repository/source untouched.

### CLI

- help and ordinary path;
- human/JSON output and exit codes;
- guided `start` reuse/error cases;
- status refresh/staleness;
- validation pass/fail/unavailable/timeout/head movement;
- review packet binding;
- findings/readiness;
- reconciliation outcomes;
- doctor/export;
- no ANSI in JSON/no-color;
- bounded output/path redaction.

### Acceptance fixture

A temporary repository fixture includes:

- canonical validator;
- accepted base and task branch;
- representative work contract;
- passing/failing validation;
- clean and changed states;
- moved head;
- simulated external acceptance.

The fixture proves the ordinary path without network, GitHub, or model dependency.

## W2 acceptance matrix

| ID | Requirement | Pass condition |
|---|---|---|
| H01 | useful without model | ordinary path works with no model/API key/runtime |
| H02 | existing tools preserved | Werkstatt edits no source and launches/renders preferred editor/terminal path |
| H03 | ordinary path concise | guided start plus status/validate/review/reconcile requires no autonomy configuration |
| A01 | authority visible | source, observation, revision, and freshness shown |
| A02 | no editable authority copy | imported project facts cannot be edited as external truth |
| A03 | stale/conflict safe | moved source blocks readiness and explains correction |
| W01 | repository identity | wrong repo/remote/base rejected |
| W02 | no false sandbox | registered-checkout limitation always visible |
| W03 | moved head invalidation | prior diff/evidence/packet becomes stale |
| E01 | execution separate | succeeded Execution does not accept WorkItem |
| E02 | evidence honest | local result executor-observed; unavailable never pass |
| E03 | revision-bound | all validation/review artifacts identify exact subject |
| R01 | packet complete | contract/base/head/source/evidence/findings present |
| R02 | findings gate | blocking finding prevents readiness |
| R03 | generated state derived | regeneration supersedes packet, not contract/source |
| C01 | shell-free | stored/invoked argv only |
| C02 | data minimized | no token/environment/home path in default output/export |
| S01 | transactional | injected failure leaves no partial state/journal transition |
| S02 | identifiable/versioned DB | application ID/schema/foreign-key rules enforced |
| S03 | local deletion safe | external repository/source untouched |
| U01 | actionable errors | cause, blocked operation, correction present |
| P01 | human pilot net value | benefits exceed added setup/bookkeeping without quality loss |
| P02 | no scope creep | no agents, GitHub writes, arbitrary command, GUI, scheduler, or merge |

## Comparative human pilot

One accepted issue authorizes a bounded documentation change performed twice from the same base in separate disposable checkouts:

- baseline with ordinary tools;
- Werkstatt-coordinated run.

Record identical metrics:

- authority discovery time and sources;
- manual state/revision copying;
- base/scope/staleness errors;
- validation/review preparation;
- extra product steps;
- correctness/review quality;
- subjective clarity/friction.

Only one result may be published.

A neutral or negative pilot may complete technical implementation but blocks W3 until the product is revised and re-evaluated.

## Delivery order

W2 may use bounded PRs only when each leaves a coherent reviewable state:

1. package/domain/minimal storage;
2. Git and work-source read adapters;
3. guided start/status;
4. validation/evidence/review/reconciliation;
5. fixtures and comparative pilot;
6. documentation and closeout.

One PR is acceptable if reviewable. Process-only activation PRs are forbidden.

## Completion

W2 completes only when:

- canonical `python scripts/validate.py` and exact-head CI pass;
- acceptance rows have evidence/verdicts;
- comparative pilot is published;
- accepted-main validation passes;
- maturity/roadmap/issues are reconciled;
- no W3 enforcement/orchestration entered scope;
- owner explicitly decides whether evidence authorizes W3.
