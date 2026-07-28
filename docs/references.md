# W0 External References

These sources informed the W0 comparison. They are supporting evidence, not Werkstatt authority. Their current versions and claims must be rechecked before implementation decisions that depend on them.

## Agent-oriented repository design

- [OpenAI — Harness engineering: leveraging Codex in an agent-first world](https://openai.com/index/harness-engineering/)
  - Relevant for concise `AGENTS.md` navigation, repository-local knowledge, structured documentation, execution plans, mechanical checks, and continuous entropy cleanup.
  - Werkstatt adopts the legibility and progressive-disclosure lessons without adopting one repository's high-throughput merge policy as a universal default.

## Codex integration

- [OpenAI — Unlocking the Codex harness: how we built the App Server](https://openai.com/index/unlocking-the-codex-harness/)
  - Relevant for a future bidirectional Codex adapter and the distinction between the Codex harness, App Server protocol, and application-specific orchestration.
  - Werkstatt keeps Codex protocol objects outside the core work domain.

## Issue-driven orchestration

- [OpenAI Symphony — service specification](https://github.com/openai/symphony/blob/main/SPEC.md)
  - Relevant for issue-driven dispatch, isolated per-issue workspaces, bounded concurrency, retries, reconciliation, repository-owned workflow configuration, and handoff states that stop before completion.
  - Werkstatt differs by making manual human development, review, architecture comprehension, and future graphical interaction first-class product responsibilities.

## Specification-driven development

- [GitHub Spec Kit](https://github.com/github/spec-kit/blob/main/README.md)
  - Relevant for progressive refinement from specification through planning, tasks, implementation, and consistency analysis.
  - Werkstatt adopts contract completeness and consistency checks but does not make generated specification bundles the universal primary authority.

## Tool and context interoperability

- [Model Context Protocol — architecture overview](https://modelcontextprotocol.io/docs/learn/architecture)
  - Relevant for tools, resources, prompts, capability negotiation, local and remote transports, and model-independent context exchange.
  - MCP is an adapter boundary, not Werkstatt's lifecycle or authority model.

## Remote agent interoperability

- [A2A Protocol — latest specification](https://a2a-protocol.org/dev/specification/)
  - Relevant for agent discovery, declared capabilities, messages, tasks, artifacts, streaming, cancellation, long-running work, and human-in-the-loop operation.
  - A2A is deferred until single-actor local execution is reliable.

## Secure development lifecycle

- [NIST Secure Software Development Framework](https://csrc.nist.gov/projects/ssdf)
- [NIST SP 800-218 Version 1.1](https://csrc.nist.gov/pubs/sp/800/218/final)
  - Relevant for integrating security practices into the normal development lifecycle rather than creating an unrelated security-only process.
  - Werkstatt's security requirements remain risk-scaled but mandatory before delegated command execution.

## Concurrency control

- [GitHub Actions concurrency](https://docs.github.com/en/actions/concepts/workflows-and-actions/concurrency)
  - Relevant as a narrow example of grouping and limiting conflicting executions rather than creating global project locks.
  - Werkstatt uses scoped, visible, expiring workspace or branch leases instead of workflow-wide authority locks.

## Dornglut primary evidence

The most important evidence remains inside Dornglut:

- [Runenwerk issue #122 — Simplify workflow authority and validation](https://github.com/dornglut/runenwerk/issues/122)
- [Runenwerk PR #123 — Simplify workflow authority and add canonical CI](https://github.com/dornglut/runenwerk/pull/123)
- [Runenwerk PR #124 — Retire legacy workflow orchestration and generated state](https://github.com/dornglut/runenwerk/pull/124)
- [Engineering ADR 0002 — Provider-neutral repository automation](https://github.com/dornglut/engineering/blob/main/adrs/0002-provider-neutral-repository-automation.md)
- [Engineering ADR 0003 — Retire provider-neutral repository automation](https://github.com/dornglut/engineering/blob/main/adrs/0003-retire-provider-neutral-repository-automation.md)
- [Engineering issue #23 — Reevaluate engineering work automation and authorize the Werkstatt pilot](https://github.com/dornglut/engineering/issues/23)

The W0 investigation should be revised when new primary evidence materially changes the product boundary, risk model, or implementation sequence.
