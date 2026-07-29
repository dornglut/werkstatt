# W1 Readiness and Authorization

## Status

W0-to-W1 gate satisfied.

W0 was accepted at `5a8a6c080a7cab0894e78d2d4dd28ee135b0808f`. Accepted ADR 0005 authorizes W1 design. [Issue #6](https://github.com/dornglut/werkstatt/issues/6) owns the active work.

The complete pre-W1 readiness contract remains available at its [immutable accepted W0 revision](https://github.com/dornglut/werkstatt/blob/5a8a6c080a7cab0894e78d2d4dd28ee135b0808f/docs/w1-readiness.md).

## Accepted W0 decisions

- Werkstatt is a human-first engineering workbench and remains useful without a model.
- humans, agents, scripts, and services use one actor-neutral but role-aware work model;
- external authority remains external;
- generated assistance and operational state cannot become parallel authority;
- the domain and headless runtime remain independent of Runenwerk and concrete providers;
- Werkstatt is contract-gated rather than universally spec-driven;
- manual value precedes agent integration;
- workspace and policy safety precede delegated execution;
- a worktree is not a sandbox;
- exact-head validation, review, and acceptance remain separate.

## Active W1 authority

- [W1 design index](w1-design.md)
- [Work domain](work-domain.md)
- [Policy and security](policy-and-security.md)
- [Authority and workspace](authority-workspace.md)
- [Evidence and review](evidence-review.md)
- [Ports and storage](ports-and-storage.md)
- [W2 CLI contract](w2-cli-contract.md)
- [W2 implementation specification](w2-implementation-spec.md)

## W1 exit gate

W1 is complete only when a competent implementation actor can build W2 without inventing:

- domain vocabulary and ownership;
- state transitions and structured errors;
- public CLI behavior;
- authority and synchronization semantics;
- workspace and revision rules;
- security and policy boundaries;
- storage and transaction behavior;
- evidence, review, validation, and acceptance rules;
- implementation dependencies and file ownership;
- tests and human-pilot acceptance criteria;
- later-phase exclusions.

## Implementation prohibition

Until W1 is accepted, the repository must not add Rust source, Cargo metadata, SQLite migrations, command execution, actor adapters, GitHub write integration, autonomous operations, or Runenwerk frontend code.
