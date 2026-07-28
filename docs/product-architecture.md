# Werkstatt Product and Architecture Design

## Status

Accepted W0 product boundary. Accepted as part of pull request #3 at revision `5a8a6c080a7cab0894e78d2d4dd28ee135b0808f`.

W1 refines this boundary through [W1 work-domain and system design](w1-design.md). The original complete W0 proposal is retained in [history](history/w0-product-architecture.md).

## Product statement

Dornglut Werkstatt is a human-first engineering workbench for understanding, performing, reviewing, and coordinating software work across human and automated actors.

It supports progressively stronger operating modes:

1. manual human development;
2. interactive assistance;
3. delegated bounded execution;
4. policy-controlled autonomous delivery.

All modes use one role-aware work model. Autonomy changes capabilities and approval requirements, not the meaning of work, evidence, review, or acceptance.

## Binding principles

- useful without a model;
- actor-neutral and role-aware;
- external authority remains external;
- generated material is revision-bound and derived;
- execution is observable and bounded;
- validation is independent from authorship;
- concrete first adapters sit behind narrow ports;
- the headless core remains independent of Runenwerk;
- ordinary manual work precedes agent integration;
- workspace and policy safety precede delegated execution.

## Product surfaces

Werkstatt may provide:

- project, program, phase, dependency, and work-item views;
- authority and freshness navigation;
- workspace and execution coordination;
- evidence and finding views;
- review packets and acceptance readiness;
- approval and policy views in later phases;
- a future Runenwerk graphical frontend over the same headless core.

Werkstatt does not replace Git, repository source, accepted architecture, issues, roadmaps, pull requests, validation, merge records, editors, terminals, or CI.

## Architecture

```text
CLI and future Runenwerk frontend
    -> application use cases and projections
        -> work domain and ports
            <- concrete adapters
```

The domain depends on no graphical host, model provider, forge, process runtime, or storage implementation.

Canonical W1 details:

- [Work domain](work-domain.md)
- [Policy and security](policy-and-security.md)
- [Authority and workspace](authority-workspace.md)
- [Evidence and review](evidence-review.md)
- [Ports and storage](ports-and-storage.md)
- [W2 CLI contract](w2-cli-contract.md)
- [W2 implementation specification](w2-implementation-spec.md)

## Phase boundary

W1 is documentation and design only. W2 is the first Rust implementation and must prove human-only value before any agent adapter or autonomous workflow is authorized.
