# Werkstatt Documentation

## Current authority

- [W1 work-domain and system design](w1-design.md)
- [Work domain](work-domain.md)
- [Policy and security](policy-and-security.md)
- [Authority and workspace](authority-workspace.md)
- [Evidence, review, and acceptance](evidence-review.md)
- [Ports and storage](ports-and-storage.md)
- [W2 human-first CLI contract](w2-cli-contract.md)
- [W2 implementation specification](w2-implementation-spec.md)

## Accepted foundation

- [Accepted W0 product boundary](product-architecture.md)
- [Accepted W0 security baseline](security-model.md)
- [W0 investigation closeout](w0-investigation.md)
- [W1 authorization](w1-readiness.md)
- [Organization decision outcome](organization-decision-input.md)
- [Human and actor workflow target](actor-workflows.md)
- [External primary references](references.md)

## Historical evidence

The `history/` directory preserves the complete W0 proposals exactly as reviewed before acceptance:

- [W0 product architecture proposal](history/w0-product-architecture.md)
- [W0 security model proposal](history/w0-security-model.md)
- [W0 investigation](history/w0-investigation.md)
- [W0 organization decision input](history/w0-organization-decision-input.md)
- [W0-to-W1 readiness contract](history/w0-w1-readiness.md)

Historical files do not override accepted current architecture or organization authority.

## Authority rules

- root [ARCHITECTURE.md](../ARCHITECTURE.md) is the concise architecture map;
- this directory owns canonical long-form product architecture;
- GitHub issue #6 owns active W1 work;
- ADR 0005 owns the organization pilot decision;
- pull requests own exact proposed delivery and review evidence;
- code/tests will own implemented behavior beginning in W2;
- generated packets and history are derived or historical, not active authority.

## Current maturity

W1 is documentation-only. No Rust product, CLI, database, actor runtime, autonomous service, GitHub write integration, or Runenwerk frontend exists.
