# Werkstatt Architecture

## Purpose

Dornglut Werkstatt is a human-first engineering workbench with optional policy-controlled execution for human, assisted, delegated, and autonomous software development.

Its core responsibility is to make accepted work understandable, executable, reviewable, and observable without replacing the repositories and systems that already own source, architecture, work authorization, validation, and acceptance.

## Boundary

Werkstatt may own local operational state such as projects, workspace registrations, actor sessions, approvals, leases, execution attempts, generated packets, local evidence, cached projections, and user preferences.

Werkstatt must not replace:

- Git and repository source;
- code and executable tests as current-behavior authority;
- accepted ADRs and architecture documents;
- repository issues as Dornglut work authority;
- repository roadmaps as durable sequence;
- pull requests as delivery and review surfaces;
- repository-owned validation and exact-head CI;
- merge commits and issue closure as acceptance records.

## Four planes

### Authority plane

Contains durable and externally owned facts:

- repositories and revisions;
- code and tests;
- ADRs and accepted designs;
- issues and roadmaps;
- pull requests, review, CI, and merge records.

### Assistance plane

Contains derived guidance and views:

- work packets;
- current-state and ownership matrices;
- implementation checklists;
- review packets;
- dependency and program projections;
- generated summaries.

Every derived artifact identifies its source, source revision, generation time, and staleness. It cannot independently authorize work or acceptance.

### Execution plane

Contains temporary operational state:

- isolated workspaces;
- actor sessions;
- narrow leases;
- approvals;
- retries and resource limits;
- activity and logs;
- branch and workspace expectations.

Execution state does not become project planning or architecture authority.

### Validation plane

Contains independent evidence:

- repository-defined commands;
- focused checks;
- exact-head CI;
- review findings;
- manual, runtime, usability, security, and performance evidence when required.

An executor may report observed results but cannot certify or accept its own work merely by declaration.

## Target layers

```text
Runenwerk graphical frontend — later
    portfolio, work, execution, evidence, approvals, diff, review

Application services
    use cases, coordination, projections, synchronization

Work domain
    projects, programs, work items, contracts, roles, capabilities,
    requirements, executions, evidence, findings, review, acceptance

Ports and adapters
    Git, GitHub, local work sources, workspaces, processes, storage,
    Codex, offline actor runtimes, validation sources
```

## Dependency direction

```text
domain
    depends on no graphical host, model provider, forge, or storage implementation

application services
    depend on the domain and ports

adapters
    implement ports for concrete systems

CLI and Runenwerk frontend
    compose application services and adapters
```

The future Runenwerk frontend depends on the headless Werkstatt core. The core must not depend on Runenwerk.

## Initial implementation shape

The first implementation should be one Rust package containing a library and CLI binary with internal modules. Crates are split only after a real dependency, release, platform, security, or compilation boundary is demonstrated.

Initial concrete choices should be simple:

- Git repositories;
- GitHub issues and pull requests for Dornglut;
- SQLite for local operational state;
- repository-defined named validation commands;
- Codex as the first delegated actor adapter;
- a generic subprocess protocol for offline actor runtimes;
- a Runenwerk frontend only after the headless human workflow is proven.

These choices sit behind narrow interfaces without requiring a provider-neutral execution platform before the product is validated.

## State separation

Werkstatt keeps separate state models for:

- work-item lifecycle;
- lifecycle stage;
- execution attempt;
- decision artifact;
- external authority synchronization.

No single `status` field may represent all of these meanings.

## Security boundary

A Git worktree is source isolation, not a security sandbox. Command execution, filesystem access, network access, secrets, dependency changes, workflow changes, publication, destructive actions, and merge require explicit policies and observable approvals.

See [docs/security-model.md](docs/security-model.md).

## Current maturity

Only W0 investigation is authorized. This document describes the target boundary; it does not claim that an implementation exists.
