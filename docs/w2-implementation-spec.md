# W2 Implementation Specification

## Status

Proposed W1 implementation contract for W2. It authorizes no code until W1 is accepted and a separate W2 issue is created.

## Objective

Implement one Rust package and one synchronous CLI that proves the accepted human-first workflow without model, autonomous, GitHub-write, or graphical dependencies.

The implementation must satisfy [W2 CLI contract](w2-cli-contract.md) while remaining small enough to review and revise after the comparative pilot.

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

## Build baseline

- Rust edition `2024`;
- minimum Rust `1.93.0`;
- stable toolchain, minimal profile, `clippy` and `rustfmt`;
- Cargo resolver `3`;
- one package containing a library and CLI binary;
- unsafe code forbidden;
- locked dependency resolution in validation and CI.

The W2 issue selects dependency versions compatible with this floor. Raising the floor requires an explicit decision.

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
| JSON | `serde`, `serde_json` | versioned public output and bounded artifacts |
| SQLite | `rusqlite` with bundled SQLite | SQL remains inside adapter |
| Errors | `thiserror` | stable public codes owned by domain/CLI contract |
| IDs | `uuid` | wrapped typed IDs; raw UUIDs do not interchange |
| Time | `time` | values supplied through clock port |
| Data directory | `directories` or maintained equivalent | overridable by `--data-dir` |
| Digests | `sha2` | identity/change detection, not authorization/signature |
| URLs | `url` | navigation/source parsing; external IDs remain opaque |

### Development

- `tempfile` for isolated repositories and databases;
- `assert_cmd` plus `predicates`, or equivalent direct CLI harness;
- optional `proptest` only for high-value parser or transition invariants.

### Excluded

No async runtime, HTTP client, Git library, GitHub SDK, model SDK, MCP/A2A implementation, ORM, event-sourcing framework, templating engine, GUI, plugin system, or telemetry service.

Any extra dependency requires explicit W2 issue/PR justification.

## Canonical validation

W2 retains and extends the existing repository-owned command:

```text
python scripts/validate.py
```

It remains the only canonical validation entrypoint for local and CI use.

W2 extends the script to run, in deterministic order:

1. existing authority, UTF-8, newline, whitespace, link, file-size, workflow-pin, and required-file checks;
2. `cargo fmt --all --check`;
3. `cargo test --workspace --locked`;
4. `cargo clippy --workspace --all-targets --locked -- -D warnings`;
5. any bounded fixture or generated-state check required by W2.

No competing `xtask` or second gate is added in W2. Focused Cargo commands may support iteration but do not replace the baseline.

## Git adapter

W2 uses the installed Git executable rather than libgit2.

Required read operations:

- repository and common-directory identity;
- exact HEAD and symbolic branch;
- remotes;
- ancestry and merge-base checks;
- status porcelain v2 with NUL separation;
- worktree list porcelain;
- ongoing merge, rebase, cherry-pick, or bisect state;
- diff statistics and bounded diff;
- changed and untracked path audit.

Rules:

- fixed argv, never shell;
- explicit working directory;
- bounded time and output;
- optional locks disabled for inspection where supported;
- external diff/text conversion disabled when deterministic output requires it;
- no authority decision from lossy path rendering;
- private absolute paths redacted from default/public output.

## Work-source adapters

### Local file

- bounded Markdown or versioned JSON;
- source digest and bounded raw artifact retained;
- known fields normalized;
- duplicate, ambiguous, or missing material values reported;
- no inferred decisions;
- active Execution binds to one immutable contract version.

### GitHub through `gh`

Optional and read-only:

- fixed `gh issue view <url> --json ...` argv;
- existing human authentication and network environment;
- no token storage or output;
- no GitHub mutation;
- bounded JSON response;
- source identity, observation time, digest, and limitations retained;
- mutable observation not described as immutable issue revision;
- exported-file fallback when `gh` is unavailable.

W5 owns richer GitHub lifecycle integration.

## Minimal SQLite schema v1

W2 deliberately avoids materializing the full future domain.

Required tables:

| Table | Purpose |
|---|---|
| `schema_migrations` | migration version, checksum, application version |
| `application_metadata` | database/profile identity and clean-shutdown marker |
| `projects` | local project registration and current bindings |
| `repository_bindings` | repository identity, remote, trust, current observation |
| `work_source_bindings` | one authoritative source per project and refresh state |
| `authority_observations` | immutable source/revision/freshness snapshots |
| `work_items` | local work identity and current immutable contract reference |
| `workspaces` | registered checkout and current audit reference |
| `executions` | manual attempt, snapshots, state, terminal reason |
| `activities` | ordered bounded execution observations and recovery timeline |
| `artifacts` | metadata, digest, relative path for packets/output |
| `evidence` | category, result, independence, exact subject, limitations |
| `findings` | category, severity, status, resolution reference |
| `reconciliations` | local external-outcome and cleanup checklist |
| `local_preferences` | non-authoritative local settings |

W2 does **not** add a separate internal event/activity journal. The `activities` table is sufficient for the manual single-process proof. W3 may introduce a distinct recovery journal only if fault-testing proves it necessary; that requires a schema migration and explicit ownership.

Programs, phases, requirements, actors, roles, policies, approvals, leases, claims, reviews, acceptances, and handoffs remain domain or versioned serialized projection concepts in W2 unless a concrete W2 query/invariant requires normalization.

## SQLite file identity

W2 uses:

```text
PRAGMA application_id = 0x574B5354
PRAGMA user_version = 1
```

`0x574B5354` is the ASCII mnemonic `WKST` and is within SQLite’s signed 32-bit range. At W2 implementation pickup, the actor must recheck SQLite’s current official `magic.txt` registry. If the value has become assigned, stop and update accepted design rather than inventing another ID locally.

Every JSON CLI response separately uses `schemaVersion: 1`; database and CLI schema versions are independent.

## SQLite rules

### Open

1. resolve or create the local profile directory;
2. open SQLite;
3. verify or initialize `application_id`;
4. explicitly enable and verify foreign keys;
5. inspect `user_version`;
6. create schema v1 or reject unsupported future schema;
7. inspect unclean-shutdown marker and run diagnostics where needed;
8. avoid unredacted private paths in public errors.

### Transactions

- short explicit write transactions;
- current-state update and associated `activities` insert commit atomically;
- Git, `gh`, process, and artifact-file calls occur outside transactions;
- optimistic version checks prevent lost local updates;
- bounded busy timeout and retry;
- no transaction spans human interaction;
- uncertain external effects are observed before retry.

### Journal mode

W2 may use SQLite’s default rollback journal. WAL is not required for the single-process proof. The actual mode is recorded; later WAL adoption must not alter domain semantics.

### Artifacts

- bounded text/JSON files under profile directory;
- SQLite stores relative path, digest, size, type, sensitivity, and retention;
- atomic temporary write and rename;
- conservative orphan diagnostics;
- explicit output truncation;
- no secrets or private chain-of-thought.

### Migrations

- schema starts at version 1;
- `user_version` changes only after successful migration;
- migration checksum recorded;
- newer unsupported schema refuses writable open;
- W2 implements initial creation and future-version refusal, not downgrade;
- destructive migration requires a later accepted decision.

## Application use cases

### Guided start

1. inspect repository;
2. import work source;
3. derive immutable WorkContract;
4. audit workspace, base, and head;
5. configure validation argv;
6. persist project/source/work/workspace in short transitions;
7. create manual Execution;
8. render status and next action.

### Refresh and status

1. read source and repository outside the database transaction;
2. normalize observations;
3. compare prior revisions;
4. persist immutable observations and current pointers;
5. invalidate affected evidence and packet projections;
6. render authority, derivation, staleness, and conflicts separately.

### Validate

1. require compatible active execution and workspace;
2. insert pending activity;
3. run stored argv outside transaction;
4. capture bounded output and head before/after;
5. persist artifact, activity, and executor-observed evidence;
6. mark stale/conflicted after revision movement;
7. render result without claiming independent CI.

### Review

1. refresh workspace;
2. compute base/head/diff summary;
3. load contract, evidence, and findings;
4. generate Markdown/JSON packet;
5. persist artifact and source revisions;
6. supersede prior packet projection;
7. render readiness and missing evidence.

### Reconcile

1. refresh available sources;
2. receive explicit external outcome;
3. verify accepted revision where possible;
4. update execution and reconciliation state;
5. mark derived/evidence history truthfully;
6. list remaining manual actions;
7. mutate no external source.

## Error implementation

Every expected failure returns:

```text
code
operation
message
retryable
correction
safe context
optional redacted adapter diagnostic
```

Requirements:

- stable families from [Work domain](work-domain.md);
- no panic for expected input/provider/storage failures;
- concise human output and versioned JSON;
- no token, raw environment, secret, private home path, or unbounded provider payload;
- backtrace only in explicit developer diagnostics.

## Test strategy

### Domain

- typed IDs cannot interchange;
- allowed and forbidden state transitions;
- terminal retry creates new Execution;
- lifecycle stage and work state independent;
- contract completeness;
- evidence invalidation after revision movement;
- execution success cannot accept work;
- errors include correction and safe context.

### Work-source parser

- valid Dornglut Markdown;
- missing, duplicate, ambiguous fields;
- extra repository sections;
- malformed table;
- bounded size and non-UTF-8 failure;
- JSON schema round-trip;
- source digest movement.

### Git adapter

Temporary real repositories cover identity/remotes, branch/detached head, clean/modified/untracked/renamed paths, ancestry/wrong base, linked worktrees, active Git operations, moved head, disabled external diff, path redaction, and unavailable Git.

### SQLite

- exact application ID and schema version;
- foreign keys enabled;
- rollback on injected failure;
- state and activity atomicity;
- optimistic conflict;
- busy timeout;
- integrity/foreign-key diagnostics;
- unclean-shutdown handling;
- unsupported future schema;
- artifact orphan diagnostics;
- local project removal leaves repository/source untouched.

### CLI

- help and ordinary path;
- human and versioned JSON output;
- exit codes;
- guided start reuse/error cases;
- refresh and staleness;
- validation pass/fail/unavailable/timeout/head movement;
- review packet binding;
- findings and readiness;
- reconciliation outcomes;
- doctor and export;
- no ANSI in JSON/no-color;
- bounded output and path redaction.

### Acceptance fixture

A temporary repository contains a canonical validator, accepted base and task branch, representative work contract, passing/failing validation, clean/changed state, moved head, and simulated external acceptance. It proves the ordinary path without network, GitHub, or model dependencies.

## W2 acceptance matrix

| ID | Requirement | Pass condition |
|---|---|---|
| H01 | useful without model | ordinary path works with no model/API key/runtime |
| H02 | existing tools preserved | Werkstatt edits no source and preserves editor/terminal/Git flow |
| H03 | ordinary path concise | five guided commands require no autonomy configuration |
| A01 | authority visible | source, observation, revision, freshness shown |
| A02 | no editable authority copy | imported facts cannot be edited as external truth |
| A03 | stale/conflict safe | moved source blocks readiness and explains correction |
| W01 | repository identity | wrong repository, remote, or base rejected |
| W02 | no false sandbox | registered-checkout limitation always visible |
| W03 | moved head invalidation | prior diff, evidence, and packet become stale |
| E01 | execution separate | succeeded Execution does not accept WorkItem |
| E02 | evidence honest | local result executor-observed; unavailable never pass |
| E03 | revision-bound | validation and review artifacts identify exact subject |
| R01 | packet complete | contract, base, head, source, evidence, findings present |
| R02 | findings gate | blocking finding prevents readiness |
| R03 | generated state derived | regeneration supersedes packet, not contract/source |
| C01 | shell-free | stored and invoked argv only |
| C02 | data minimized | no token, environment, or home path in default output/export |
| S01 | transactional | injected failure leaves no partial state/activity transition |
| S02 | identifiable/versioned DB | application ID, schema, foreign-key rules enforced |
| S03 | local deletion safe | external repository/source untouched |
| U01 | actionable errors | cause, blocked operation, correction present |
| P01 | pilot net value | benefit exceeds setup/bookkeeping without quality loss |
| P02 | no scope creep | no agents, GitHub writes, arbitrary command, GUI, scheduler, or merge |

## Counterbalanced human pilot

A single same-task baseline-first test would credit learning and repetition effects to Werkstatt. W2 therefore uses two small comparable documentation tasks, each performed under both modes in separate disposable checkouts:

| Task | First run | Second run |
|---|---|---|
| A | ordinary workflow | Werkstatt workflow |
| B | Werkstatt workflow | ordinary workflow |

Rules:

- both tasks have accepted issues, comparable scope, exact bases, canonical validation, and predefined completion/review criteria;
- each run starts from its task’s same accepted base and uses a fresh checkout;
- only one result per task may be published; comparison runs are disposable evidence;
- task identity, order, prior familiarity, interruptions, and anomalies are recorded;
- results are directional product evidence, not a statistical performance claim.

Record:

- authority-discovery time and sources;
- manual revision/state copying;
- base, scope, and staleness errors;
- validation/review preparation effort;
- extra product steps;
- correctness and review quality;
- subjective clarity and friction.

A neutral or negative pilot may complete W2 technically but blocks W3 until the product is revised and reevaluated.

## Delivery order

W2 may use bounded PRs only when each leaves a coherent reviewable state:

1. package, domain, and minimal storage;
2. Git and work-source read adapters;
3. guided start and status;
4. validation, evidence, review, and reconciliation;
5. fixtures and counterbalanced pilot;
6. documentation and closeout.

One PR is acceptable if reviewable. Process-only activation PRs remain forbidden.

## Completion

W2 completes only when:

- canonical validation and exact-head CI pass;
- acceptance rows have evidence and verdicts;
- the counterbalanced pilot is published;
- accepted-main validation passes;
- maturity, roadmap, and issues are reconciled;
- no W3 enforcement or orchestration entered scope;
- the owner explicitly decides whether evidence authorizes W3.
