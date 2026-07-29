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

The complete W0 documents remain available at immutable Git revision `5a8a6c080a7cab0894e78d2d4dd28ee135b0808f`. Current summary documents link directly to their accepted historical versions.

Do not duplicate accepted proposals into a second active or historical file tree merely for convenience. Git history is the evidence store unless a later issue requires a maintained historical document for a distinct purpose.

## Authority rules

- root [ARCHITECTURE.md](../ARCHITECTURE.md) is the concise architecture map;
- this directory owns canonical long-form product architecture;
- GitHub issue #6 owns active W1 work;
- ADR 0005 owns the organization pilot decision;
- pull requests own exact proposed delivery and review evidence;
- code/tests will own implemented behavior beginning in W2;
- generated packets and immutable historical revisions are derived or historical, not active authority.

## Current maturity

W1 is documentation-only. No Rust product, CLI, database, actor runtime, autonomous service, GitHub write integration, or Runenwerk frontend exists.
