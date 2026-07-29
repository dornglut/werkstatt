# Werkstatt Architecture

## Purpose

Dornglut Werkstatt is a human-first engineering workbench with optional policy-controlled execution for human, assisted, delegated, and autonomous software development.

It makes accepted work understandable, executable, reviewable, and observable without replacing the repositories and systems that own source, architecture, work authorization, validation, and acceptance.

## Authority boundary

Werkstatt may own local operational state such as project registrations, authority observations, workspaces, actor sessions, policies, approvals, leases, executions, activities, artifacts, local evidence, findings, and preferences.

Werkstatt must not replace:

- Git and repository source;
- code and executable tests as behavior authority;
- accepted ADRs and designs;
- repository issues as Dornglut work authority;
- repository roadmaps as durable sequence;
- pull requests as delivery and review surfaces;
- repository-owned validation and exact-head CI;
- merge commits and issue closure as acceptance records.

## Four planes

| Plane | Owns |
|---|---|
| Authority | external repositories, decisions, work, sequence, PR, CI, and acceptance facts |
| Assistance | revision-bound packets, matrices, plans, summaries, and projections |
| Execution | local workspaces, sessions, policies, approvals, leases, attempts, activities, and recovery |
| Validation | independent evidence, findings, review, acceptance, and reconciliation |

Generated or operational material cannot silently move into the authority plane.

## Layers and dependency direction

```text
CLI and future Runenwerk frontend
    -> application use cases and projections
        -> work domain and ports
            <- concrete adapters
```

```text
domain
    depends on no graphical host, model provider, forge, process runtime,
    protocol, filesystem layout, or storage implementation

application
    depends on domain and port contracts

ports
    express use-case-owned read or execution boundaries

adapters
    implement repository, work-source, workspace, command, actor,
    validation, storage, launcher, clock, and publication ports

frontends
    compose application services and concrete adapters
```

The future Runenwerk frontend depends on the headless core. The core must not depend on Runenwerk.

## W2B observation boundary

W2B adds no application workflow and no general command port. It adds only repository and work-source read boundaries:

```text
repository/work-source observation domain values
    <- repository and work-source read ports
        <- fixed-argv Git CLI, local-file, and optional read-only gh adapters
```

The concrete adapters own process, filesystem, Git, `gh`, parsing, timeout, output-bound, and redaction behavior. Domain observations contain provider-neutral identities, exact revisions, normalized facts, synchronization state, limitations, and safe display material only.

Repository identity is distinct from checkout location and Git common-directory identity. Authority decisions use opaque fingerprints and exact revisions, never lossy path rendering. `RepositoryIdentity::observation_fingerprint` is a heuristic derived from observed root revisions and credential-safe remotes; it can change when those observations change and must not be treated as a durable repository identifier. The common-directory key identifies the observed local Git administration location separately. Remote identities remove credentials, query strings, and fragments before entering retained observations.

Git and `gh` are invoked directly with fixed argument vectors, explicit working directories, bounded timeout and output, and no shell. Every Git command overrides `core.fsmonitor=false`, so repository-local fsmonitor programs cannot execute during observation. Git inspection also disables optional locks, prompts, external diff helpers, and text conversion where deterministic output requires it. Bounded diff observations retain separately labelled staged and unstaged statistics and patch sections under one aggregate output limit. No read adapter fetches, writes configuration, updates refs, changes the index/worktree, or mutates GitHub.

A work-source read returns one bounded `ObservedWorkSource`: the exact payload plus a normalized source observation. That observation reuses W2A `AuthorityObservation`, immutable SHA-256 `RevisionRef`, and `SynchronizationState`; it does not create a parallel authority or freshness family. Markdown accepts either canonical leading `Field: value` issue metadata or an explicit `## Work contract` table, rejects duplicate values across forms, and preserves unrecognized metadata and sections as supplemental context. Presentation-only surrounding backticks are removed from scalar metadata such as accepted revisions.

The private JSON helper accepts at most 131,072 bytes, rejects duplicate keys, and limits nesting to 64 levels. The W2B owning issue records the bounded-helper decision and the conditions that require revisiting accepted dependencies.

A mutable GitHub issue is represented as a mutable external source with an immutable exact-payload observation revision, provider update information, and limitations. It is never described as an immutable issue revision. Exported bounded Markdown or schema-version-1 JSON remains the offline fallback.

## Core distinctions

Werkstatt keeps separate:

- Project and external authority source;
- Program/Phase projection and WorkItem;
- WorkItem state and lifecycle stage;
- WorkContract and derived Plan;
- Actor, Role, Capability, Policy, and Approval;
- Workspace, Git administrative state, and WriterLease;
- Execution state and WorkItem acceptance;
- Activity, Artifact, Evidence, Finding, Review, ValidationReceipt, AcceptanceRecord, and Reconciliation;
- provider session/task identifiers and Werkstatt domain identifiers.

No single `status` field or receipt may collapse these meanings.

## W1 canonical detail

- [W1 design index](docs/w1-design.md)
- [Work domain](docs/work-domain.md)
- [Policy and security](docs/policy-and-security.md)
- [Authority and workspace](docs/authority-workspace.md)
- [Evidence and review](docs/evidence-review.md)
- [Ports and storage](docs/ports-and-storage.md)
- [W2 CLI contract](docs/w2-cli-contract.md)
- [W2 implementation specification](docs/w2-implementation-spec.md)

## Persistence boundary

SQLite stores local current state and bounded activities. External authority is represented through source references and revision observations.

- short explicit transactions;
- one writer at a time;
- no network, Git, `gh`, process, or artifact-file call inside a database transaction;
- foreign keys enabled explicitly;
- application-owned schema version and migrations;
- rollback journal retained for the single-process W2 proof;
- local database deletion cannot delete external project authority.

W2B does not change schema v1 or begin W2C project-registration workflows.

## Security boundary

A checkout or Git worktree is not a sandbox. Filesystem, commands, network, secrets, dependencies, workflows, publication, destructive actions, resources, cancellation, recovery, review, and merge require explicit policies in phases that execute them.

W2 records observations and limitations. W3 must prove enforcement before delegated execution.

## Current maturity

W0 and W1 are accepted. W2A is accepted at `bd4f12f770fa82d25657f41fd8cff5e3a299a8a1`. W2B is the active read-only repository and work-source observation delivery. Guided commands, validator execution, agents, lease enforcement, GitHub mutation, autonomy, and Runenwerk remain absent.
