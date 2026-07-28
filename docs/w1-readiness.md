# W1 Readiness and Authorization

## Purpose

This document defines what W0 accepts, what W1 may design, what remains undecided, and what must not enter implementation before the required gates are accepted.

## W0 decisions proposed for acceptance

### Product

Werkstatt is a human-first engineering workbench with optional policy-controlled execution for human, assisted, delegated, offline, and autonomous development.

The product must remain useful without a model.

### Work model

Humans, agents, scripts, and services use one actor-neutral work model. Responsibilities remain role-aware, and technical capability never implies decision or acceptance authority.

### Authority

Werkstatt consumes and projects source, architecture, issues, roadmaps, pull requests, validation, and merge records. It does not replace their authority.

Generated assistance and operational execution state remain derived or local and cannot become parallel planning, architecture, validation, or acceptance authority.

### Architecture

The domain and headless runtime remain independent of Runenwerk, model providers, forges, and concrete storage. A future Runenwerk frontend consumes the headless core.

### Implementation order

Manual human usefulness is proven before agent integration. Workspace and policy safety are proven before delegated execution. Draft-PR delegation is proven before review-loop automation, automatic merge, or multi-agent execution.

### Security

A worktree is not a sandbox. Persistent delegated execution requires explicit policies for filesystem, commands, network, secrets, dependencies, workflows, publication, destructive operations, validation, merge, resources, cancellation, and recovery.

### Specification policy

Werkstatt is contract-gated rather than universally spec-driven. Formal specifications are used when an actor would otherwise need to invent a material product, architecture, safety, compatibility, migration, or proof decision.

## W1 authorized scope

W1 may design, without implementing product code:

- domain vocabulary and typed identifiers;
- entity ownership and invariants;
- role and actor relationships;
- capabilities, scopes, policy composition, approvals, and denials;
- work-item, lifecycle-stage, execution, and decision-artifact state transitions;
- work contracts, requirements, stop conditions, and exit gates;
- authority references, refresh, staleness, conflicts, and synchronization;
- projects, programs, phases, dependencies, and work-source bindings;
- workspace identity, leases, ownership transfer, recovery, and cancellation;
- execution attempts, activity, artifacts, receipts, and handoffs;
- evidence categories, findings, review, validation, acceptance, and reconciliation;
- storage and migration boundaries;
- repository, work-source, workspace, command, actor-runtime, validation, editor, terminal, publication, and clock ports;
- security enforcement and threat-control mapping;
- W2 CLI use cases, public contract, validation, and proof matrix;
- initial repository structure and one-package implementation boundary.

## W1 required outputs

1. accepted domain glossary;
2. ownership and invariant matrix;
3. state-transition specifications with structured errors;
4. role, capability, policy, approval, and conflict model;
5. authority-reference and synchronization model;
6. workspace, lease, cancellation, and recovery model;
7. evidence, finding, review, validation, and acceptance model;
8. adapter port contracts;
9. SQLite operational schema design and migration policy, without implementation;
10. security-control matrix mapping threats to domain and adapter enforcement;
11. W2 human-first CLI architecture and user journeys;
12. W2 implementation specification and acceptance tests;
13. implementation file and dependency plan;
14. explicit non-goals and later-phase ownership;
15. organization decision status and remaining constraints from `dornglut/engineering#23`.

## W1 review questions

### Domain quality

- Is each concept necessary and owned once?
- Are work item, contract, plan, execution, artifact, evidence, review, and acceptance distinct?
- Can the model represent manual and automated work without parallel entity families?
- Are external authority and local operational state distinguishable in types and APIs?

### Human ergonomics

- Can ordinary manual work avoid advanced agent and policy fields?
- Are names understandable to developers without workflow jargon?
- Do errors explain cause, blocked operation, and correction?
- Can a new contributor begin from repository, program issue, and active issue?

### Agent execution

- Can an actor receive a complete bounded contract?
- Are capabilities scoped independently from prompts and model output?
- Are stop, escalation, cancellation, retry, and handoff first-class?
- Can an execution report actions without claiming independent validation?

### Security

- Is deny-by-default enforceable?
- Are command, network, secret, dependency, workflow, publication, and merge permissions independent?
- Can stale authority, moved heads, conflicting writers, and expired approvals fail closed?
- Can stronger sandbox adapters be added without changing domain semantics?

### Architecture

- Does the headless core remain independent of Runenwerk and concrete providers?
- Are initial concrete adapters possible without a generic ForgeOps platform?
- Does one package remain coherent for W2?
- Are future crate splits based on observed boundaries?

## W1 stop conditions

Stop and return to W0 or the organization owner if W1 discovers that:

- human and agent workflows require fundamentally incompatible domain models;
- the product cannot remain useful without a model;
- external authority cannot be consumed without creating duplicate editable state;
- capability policy cannot safely constrain delegated command execution;
- the Runenwerk frontend would need to own the core domain;
- W2 cannot prove manual value without implementing W3 orchestration infrastructure;
- the organization decision rejects or materially narrows the pilot;
- the proposed scope requires a general provider-neutral publication platform before one concrete workflow is proven.

## Implementation prohibition

W1 does not authorize:

- Rust product source;
- Cargo manifests or dependencies;
- SQLite migrations;
- command execution;
- Codex or offline-agent adapters;
- GitHub credentials or write integrations;
- CI authoring beyond documentation validation required for the planning repository;
- automatic issue, branch, PR, review, merge, or reconciliation operations;
- Runenwerk frontend code.

## W1 exit gate

W1 is complete when a competent implementation actor can build W2 without inventing:

- the domain model;
- public CLI behavior;
- ownership and dependency direction;
- security and policy semantics;
- storage boundaries;
- error and recovery behavior;
- evidence and validation requirements;
- human workflow acceptance criteria;
- later-phase exclusions.

Only then may W2 Rust implementation be authorized.
