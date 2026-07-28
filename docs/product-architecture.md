# Werkstatt Product and Architecture Design

## Status

Proposed W0 target design. This document becomes accepted only through review and merge of its owning pull request.

## Product statement

Dornglut Werkstatt is a human-first engineering workbench for understanding, performing, reviewing, and coordinating software work across human and automated actors.

It supports four progressively stronger operating modes:

1. manual human development;
2. interactive assistance;
3. delegated bounded execution;
4. policy-controlled autonomous delivery.

All modes use one work model. Autonomy changes allowed capabilities and approval requirements, not the meaning of work, evidence, review, or acceptance.

## Product principles

### Human useful without a model

The ordinary manual workflow must be complete and valuable when no model, agent, API key, or network service is configured.

### Actor-neutral and role-aware

Humans, agents, scripts, and services are actors. Roles express responsibility; capabilities and policies express permitted operations. Technical capability never implies decision or acceptance authority.

### External authority remains external

Werkstatt consumes and projects repositories, issues, architecture, roadmaps, pull requests, validation, and merge state. It does not silently replace them.

### Generated material is derived

Generated packets, matrices, summaries, and views identify source authority, source revision, generation time, and staleness. They cannot independently authorize or accept work.

### Execution is observable and bounded

Each execution records actor, role, workspace, base, policy, actions, approvals, artifacts, evidence, limits, and final handoff.

### Validation is independent

An executor may report observations. Repository-defined validation, exact-head CI, review, and explicit acceptance remain separate.

### Concrete first, replaceable later

Use real Git, GitHub, SQLite, Codex, and process adapters first. Preserve narrow ports without building a universal provider-neutral platform before demand is proven.

## Primary product surfaces

### Portfolio and program overview

Shows projects, programs, phases, dependencies, current lifecycle stage, work-item state, blockers, and next valid work.

It is a projection over authoritative sources. It must show freshness and source identity.

### Work item

Shows:

- goal and owner;
- authoritative source;
- scope and non-goals;
- requirements;
- dependencies and blockers;
- lifecycle stage and work state;
- workspaces and executions;
- evidence and findings;
- approvals and stop conditions;
- next valid transition.

### Workspace

Shows:

- repository and exact base revision;
- checkout or worktree path;
- branch and remote state;
- current actor and lease;
- changed files;
- editor and terminal launch points;
- running commands and activity;
- policy and resource limits.

### Execution

Shows one attempt:

- actor and role;
- selected capabilities;
- start and end state;
- commands, approvals, outputs, and artifacts;
- validation observations;
- failure, cancellation, retry, or handoff.

### Evidence and findings

Separates:

- source inspection;
- local validation;
- exact-head CI;
- manual verification;
- runtime proof;
- security evidence;
- performance evidence;
- usability evidence.

Findings retain category, severity, source, status, owner, and resolution evidence.

### Review and acceptance

Shows:

- accepted contract;
- requirement-to-diff mapping;
- public API and dependency changes;
- migration and deletion evidence;
- validation and exact-head state;
- unresolved findings;
- owner decision and reconciliation.

### Approval inbox

Prioritizes actions requiring human or policy approval, such as:

- material design choices;
- arbitrary command execution;
- network or secret access;
- dependency or workflow changes;
- destructive operations;
- branch publication;
- policy exceptions;
- merge and issue closure.

## Core domain

### Project

A repository or related work context. A project binds one authoritative work source and one or more repository sources.

### Program

A multi-phase outcome with durable sequence and closure criteria.

### Work item

An authorized unit of investigation, decision, delivery, verification, maintenance, or reconciliation.

### Work contract

The goal, owner, authority, scope, non-goals, requirements, validation, stop conditions, and exit gate for a work item.

### Actor

A human, agent, script, or service capable of participating in work.

### Role

The responsibility under which an actor performs an action.

### Capability

An operation an actor or adapter can technically perform.

### Policy

The conditions under which a capability may be used, including approvals and limits.

### Workspace

An isolated or registered working environment bound to a repository and exact base revision.

### Execution

One attempt by an actor to perform work in a workspace under a policy.

### Artifact

A produced plan, patch, report, document, image, binary, log, or other result.

### Evidence

An observation supporting a claim about source, validation, security, behavior, usability, or performance.

### Finding

A problem, risk, uncertainty, or nonconformance discovered during investigation, validation, or review.

### Review

An assessment of an artifact or execution against an accepted contract and selected review dimensions.

### Approval

Explicit permission for a gated operation.

### Acceptance

A decision that a result becomes accepted project state.

## Roles

| Role | Owns |
|---|---|
| Requester | Desired outcome or reported problem |
| Owner | Product or architectural decision |
| Investigator | Current reality, evidence, uncertainty |
| Designer | Target behavior and boundaries |
| Executor | Workspace changes and produced artifacts |
| Validator | Independent checks and observed results |
| Reviewer | Correctness and conformance assessment |
| Approver | Permission for gated operations |
| Acceptor | Accepted repository or project transition |
| Operator | Execution environment and service health |

An actor may hold multiple roles. Role conflicts are handled by policy; for example, an executor may run local checks but does not automatically act as independent validator or acceptor.

## Capability model

Initial capability families:

```text
project.read
repository.read
workspace.register
workspace.create
workspace.write
command.run_named
command.run_arbitrary
network.read
network.write
secret.read
dependency.modify
workflow.modify
branch.create
branch.publish
issue.comment
issue.update
pull_request.create
review.respond
pull_request.merge
issue.close
dependent_work.activate
```

Capabilities are scoped by project, repository, path, command, destination, time, resource limit, work class, and required approval.

## Operating modes

| Mode | Behavior |
|---|---|
| Manual | Human performs mutations; Werkstatt provides context and evidence capture |
| Observe | Actor reads and analyzes only |
| Suggest | Actor proposes plans, findings, or patches without applying them |
| Assist | Actor applies operations approved by the human |
| Delegate | Actor completes bounded work and stops at draft-PR handoff |
| Review loop | Actor may process CI and review findings under policy |
| Autonomous delivery | Actor may publish and accept selected work classes under explicit policy |
| Program automation | Actor may select and sequence already approved eligible work |

Modes are policy presets, not domain states.

## State models

### Work-item lifecycle

```text
captured -> ready -> active -> in_review -> accepted -> completed
                  \-> blocked
captured/ready/active/blocked/in_review -> cancelled
```

### Lifecycle stage

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

Stage and state are independent. A delivery work item may be blocked; a verification work item may be active.

### Execution attempt

```text
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
```

### Decision artifact

```text
draft
proposed
accepted
superseded
rejected
```

W1 will define exact transition rules and failure semantics.

## Authority references

Every external fact is represented with:

- authority kind;
- stable source identity;
- source location;
- observed revision or version;
- observation time;
- optional freshness policy;
- current synchronization state.

Werkstatt displays whether a fact is:

- authoritative;
- derived;
- cached;
- generated;
- observed;
- unverified;
- stale.

## Generated artifacts

Supported generated projections may include:

- work packet;
- execution packet;
- review packet;
- program matrix;
- current-state inventory;
- ownership and disposition matrix;
- requirement-to-diff matrix;
- validation summary.

Generation never changes the source authority. Regeneration replaces the derived view rather than creating a new planning authority.

## Application services

Initial use cases:

- register project and repository;
- bind authoritative work source;
- import or refresh work item;
- render work contract and packets;
- register or create workspace;
- start human execution;
- run named validation;
- record evidence and finding;
- inspect diff against contract;
- produce review packet;
- publish or observe branch and PR;
- reconcile accepted work.

Agent execution services follow after W2 and W3.

## Ports

Initial ports:

- repository inspection and mutation;
- work-source query and update;
- workspace lifecycle;
- command execution;
- actor runtime;
- validation evidence source;
- storage;
- clock and identifiers;
- editor and terminal launch;
- publication and review integration.

## Persistence

Use SQLite for local current state and a bounded append-only activity journal for observability.

Do not make the journal the sole reconstruction mechanism.

External authority is synchronized by references and observed revisions, not copied as independently editable project truth.

## Frontends

### CLI

The first product frontend. It proves the headless work model and manual development flow.

### Runenwerk frontend

A later graphical workbench using the same application services and domain. It must remain usable when no model is configured.

### Future integrations

MCP and A2A may be supported at adapter boundaries. They do not define the internal work lifecycle or authority model.

## Implementation constraints

- one Rust package initially;
- no premature crate family;
- no agent integration before human-only proof;
- no delegated actor before workspace and policy runtime;
- no automatic merge before connected review-loop reliability;
- no multi-agent execution before dependable single-agent operation;
- no direct protected-branch writes;
- no source-writing validation workflow;
- no file above 131,072 raw bytes without explicit exception and suitable execution path.

## Open W1 design work

W1 must specify:

- typed identifiers and ownership;
- exact state transitions and errors;
- role conflict and approval semantics;
- capability scopes and policy composition;
- authority refresh and conflict behavior;
- workspace lease and recovery behavior;
- evidence, finding, review, and acceptance schemas;
- storage boundaries and migrations;
- adapter contracts;
- security enforcement;
- W2 public CLI contract and acceptance tests.
