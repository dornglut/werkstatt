# W0 Product and Workflow Investigation

## Status

Completed and accepted. W0 was accepted through pull request #3 at revision `5a8a6c080a7cab0894e78d2d4dd28ee135b0808f`.

The complete investigation as reviewed before acceptance is retained in [history](history/w0-investigation.md). Current architecture is indexed by [W1 design](w1-design.md).

## Question

Could Dornglut recover useful generated guidance, execution state, and orchestration without recreating the retired Runenwerk workflow platform or the disproportionate ForgeOps proposal?

## Findings

The failed model combined too many concerns:

- product and architecture authority;
- generated prompts and task state;
- roadmap and execution databases;
- locks and certificates;
- validation gates;
- workspace and batch orchestration;
- repository policy and diagrams.

The underlying needs remained valid:

- better overview;
- repeatable human and agent handoff;
- isolated execution;
- observable activity;
- easier review and reconciliation;
- later autonomous operation.

The correct separation is:

1. external project authority;
2. derived assistance;
3. bounded local execution state;
4. independent validation and acceptance.

## Accepted product decision

Werkstatt is a human-first engineering workbench with optional policy-controlled execution.

- humans and automated actors use one role-aware work model;
- the product is useful without a model;
- generated material is derived rather than authority;
- the headless core remains independent of Runenwerk;
- concrete adapters precede provider-neutral infrastructure;
- human-only value is proven before agent integration;
- safety and recovery are proven before delegated execution;
- automatic merge and multi-agent operation remain later decisions.

## Organization outcome

Dornglut accepted ADR 0005, which authorizes the bounded Werkstatt pilot while preserving ADR 0003’s rejection of a mandatory provider-neutral publisher and product-delivery dependency.

## Next transition

W1 owns the work-domain and W2 implementation contract. No Rust implementation is authorized until W1 is accepted.
