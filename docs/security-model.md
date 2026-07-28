# Werkstatt Security and Trust Model

## Status

Accepted W0 security baseline. Accepted as part of pull request #3 at revision `5a8a6c080a7cab0894e78d2d4dd28ee135b0808f`.

W1 converts this baseline into explicit capability, policy, approval, authority, workspace, adapter, storage, evidence, and error contracts. See [Policy and security](policy-and-security.md), [Authority and workspace](authority-workspace.md), and [Ports and storage](ports-and-storage.md).

The complete reviewed W0 threat analysis remains available at its [immutable accepted revision](https://github.com/dornglut/werkstatt/blob/5a8a6c080a7cab0894e78d2d4dd28ee135b0808f/docs/security-model.md).

## Security objective

Werkstatt must support useful human and automated development without treating repository code, model output, commands, dependencies, workspaces, credentials, or external services as implicitly trusted.

## Binding trust boundaries

- external project authority may be authoritative for project questions but is not automatically safe to execute;
- the Werkstatt process owns local operational state and policy enforcement;
- a checkout or Git worktree is not a security sandbox;
- actor output is untrusted until reviewed and validated under policy;
- compilers, tests, package managers, hooks, and build scripts may execute untrusted code;
- external providers have separate identity, permission, availability, and data boundaries.

## Binding controls

- deny by default;
- capabilities and policies are configured outside untrusted issue, repository, tool, and model content;
- read, propose, apply, publish, validate, approve, and accept are separate operations;
- filesystem, commands, network, secrets, dependencies, workflows, publication, merge, resources, cancellation, and recovery have explicit policy dimensions;
- one active writer owns a workspace at a time in managed modes;
- expected revisions and moved-head detection are first-class;
- execution receipts, validation receipts, review records, and acceptance records remain separate;
- secrets are referenced through opaque grants and redacted before persistence;
- logs and artifacts are bounded and retention-aware;
- private chain-of-thought is not stored;
- stronger sandbox adapters may be added without changing domain semantics.

## Phase gates

| Phase | Security result |
|---|---|
| W1 | capability, policy, approval, state, error, adapter, and storage contracts accepted |
| W2 | manual workflow makes no false sandbox or authority claim and requires no Werkstatt-managed secrets |
| W3 | workspace, command, lease, approval, resource, cancellation, and recovery enforcement proven |
| W4 | one Codex execution constrained to one issue, workspace, branch, policy, and draft-PR handoff |
| W5+ | publication, review-loop, offline-runtime, UI, merge, and multi-actor controls proven in sequence |

## Stop conditions

Delegated or autonomous execution remains unauthorized when isolation, command policy, secret scope, validation independence, revision identity, actor/policy identity, decision completeness, or conflict recovery is undefined.
