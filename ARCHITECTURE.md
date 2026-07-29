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

adapters
    implement repository, work-source, workspace, command, actor,
    validation, storage, launcher, clock, and publication ports

frontends
    compose application services and concrete adapters
```

The future Runenwerk frontend depends on the headless core. The core must not depend on Runenwerk.

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

## Initial concrete choices

W2 is designed as:

- one Rust package with a library and synchronous CLI;
- Git CLI inspection rather than a Git library;
- local Markdown/JSON plus optional read-only `gh` issue import;
- SQLite for local operational state;
- shell-free named validation command execution;
- existing editor, terminal, Git, and GitHub tooling;
- no model, agent, network client, GitHub mutation, or Runenwerk dependency.

Later adapters may support Codex App Server, offline runtimes, MCP tools/context, A2A agents, richer GitHub integration, sandboxes, and the Runenwerk frontend. Their provider state remains adapter state.

## Persistence boundary

SQLite stores local current state and a bounded activity journal. External authority is represented through source references and revision observations.

- short explicit transactions;
- one writer at a time;
- no network/process call inside a database transaction;
- foreign keys enabled explicitly;
- application-owned schema version and migrations;
- WAL optional for later measured concurrency, never distributed coordination;
- local database deletion cannot delete external project authority.

## Security boundary

A checkout or Git worktree is not a sandbox. Filesystem, commands, network, secrets, dependencies, workflows, publication, destructive actions, resources, cancellation, recovery, review, and merge require explicit policies in phases that execute them.

W2 records manual observations and limitations. W3 must prove enforcement before delegated execution.

## Current maturity

W0 and W1 are accepted. W2 is active at the W2A package, domain, and local SQLite foundation only. Git/forge observation, workflow execution, agents, lease enforcement, GitHub mutation, autonomy, and Runenwerk remain absent.
