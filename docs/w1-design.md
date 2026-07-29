# W1 Work-Domain and System Design

## Status

Proposed W1 authority for [issue #6](https://github.com/dornglut/werkstatt/issues/6). It becomes accepted only through review and merge of its owning pull request.

## Purpose

W1 turns the accepted W0 product boundary into a decision-complete design for W2, the human-first headless CLI proof.

W1 defines domain vocabulary, states, policy, authority synchronization, workspace and execution semantics, evidence/review/acceptance, adapter ports, storage, CLI behavior, implementation structure, dependencies, validation, and acceptance tests.

W1 contains no product implementation.

## Accepted foundation

- ADR 0005 authorizes the bounded Werkstatt pilot.
- Werkstatt is useful without a model.
- humans and automated actors use one role-aware work model;
- external project authority remains external;
- generated assistance and local execution state cannot become parallel authority;
- exact revisions, independent validation, review, acceptance, and reconciliation remain distinct;
- the headless core remains independent of Runenwerk and concrete providers;
- W2 proves manual value before agent integration;
- W3 proves execution safety before delegated execution.

## Canonical W1 decisions

| Area | Authority | Binding result |
|---|---|---|
| Domain vocabulary, ownership, invariants, states, errors | [Work domain](work-domain.md) | each concept owned once; retries create new terminal attempts |
| Roles, capabilities, policy, approvals, denials, threats | [Policy and security](policy-and-security.md) | deny by default; prompts/content never grant capability |
| External authority, freshness, workspace, lease, recovery | [Authority and workspace](authority-workspace.md) | one work source; one writer per managed workspace; Git lock is not writer lease |
| Activities, artifacts, evidence, findings, review, acceptance | [Evidence and review](evidence-review.md) | executor report, validation, review, acceptance, reconciliation remain distinct |
| Ports, protocol mappings, SQLite, migrations | [Ports and storage](ports-and-storage.md) | provider sessions/tasks are adapter state; SQLite stores local operations only |
| Human-facing W2 behavior | [W2 CLI contract](w2-cli-contract.md) | guided five-command path; existing tools preserved |
| W2 package, dependencies, schema, tests, pilot | [W2 implementation specification](w2-implementation-spec.md) | one small Rust package and explicit acceptance matrix |

## Architecture

```text
CLI and future Runenwerk frontend
    -> application use cases and projections
        -> work domain and ports
            <- concrete adapters
```

Dependency direction:

```text
domain
    depends on no adapter, database, forge, model, process, protocol,
    filesystem layout, or graphical host

application
    depends on domain and port contracts

adapters
    implement ports

frontends
    compose application services and adapters
```

## Stable domain versus provider vocabulary

| Provider concept | Adapter meaning | Werkstatt concept |
|---|---|---|
| Git branch/commit/worktree | repository/workspace observation | RepositoryRef, RevisionRef, Workspace |
| Git worktree lock | administrative protection | not WriterLease |
| GitHub issue | authoritative work-source record | WorkItemRef and WorkSnapshot |
| GitHub pull request | delivery/review authority | ChangeProposalRef |
| CI run | independent validation observation | ValidationReceiptRef |
| Codex thread/turn | actor-runtime session/interaction | ActorSession and Execution activities |
| MCP connection/tool/resource | context/tool adapter | AdapterSession, operation, artifact reference |
| A2A task | remote runtime correlation | remote Execution correlation |
| SQLite transaction | persistence mechanism | not work/execution transition |

Provider schemas and identifiers remain opaque, versioned adapter concerns.

## W2 ordinary path

```text
werkstatt start
    -> edit with normal tools
        -> werkstatt status
            -> werkstatt validate
                -> werkstatt review
                    -> publish/review manually
                        -> werkstatt reconcile
```

Advanced project/work/workspace/execution/evidence commands compose the same application services but do not burden ordinary manual use.

## W2 build baseline

- Rust edition: `2024`;
- minimum supported Rust: `1.93.0`;
- toolchain declaration: stable, minimal profile, `clippy` and `rustfmt` components;
- Cargo resolver: `3`;
- one package with library and CLI binary;
- unsafe code forbidden at workspace/package lint level;
- canonical validation remains `python scripts/validate.py` and is extended to Cargo format, test, and strict Clippy checks;
- locked dependency resolution is required in validation and CI.

Dependency versions are selected and locked in the W2 implementation issue against this compatibility floor. The implementer may not silently raise the minimum Rust version or add an async/model/GitHub SDK stack.

## Cross-cutting invariants

1. One project has at most one authoritative work source.
2. External authority is referenced/observed, never copied as independently editable truth.
3. Derived views identify inputs, revisions, generation time, and staleness.
4. Work state, lifecycle stage, execution state, decision state, approval state, lease state, and sync state remain separate.
5. A managed workspace has at most one active writer lease.
6. Actor role, technical capability, policy permission, approval, review, and acceptance are independent.
7. Execution receipts cannot serve as independent validation.
8. Validation/review evidence binds to exact revision and becomes stale when the subject moves.
9. Acceptance requires authoritative observation or decision, not local success.
10. Local state may be deleted without deleting external project authority.
11. W2 executes no model/agent and performs no GitHub mutation.

## Structured errors

Every public failure contains:

- stable code;
- failed operation;
- cause category;
- safe relevant identifiers/revisions;
- retryability;
- corrective next action;
- optional redacted adapter diagnostic.

Human output is concise; JSON output is stable and machine-readable.

## W1 conformance

| Requirement | Evidence |
|---|---|
| One human/agent model | shared entities, roles, capabilities, executions, evidence |
| Human-only usefulness | guided CLI and comparative pilot |
| External authority preserved | typed authority references and refresh/conflict rules |
| States separated | explicit transition models |
| Deny-by-default policy | policy composition and approval validity |
| Workspace safety | identity, audit, lease, cancellation, recovery |
| Independent validation | receipt/review/acceptance separation |
| Replaceable adapters | narrow ports and protocol mappings |
| Local storage only | reduced W2 schema and transaction/migration rules |
| Runenwerk independence | dependency direction |
| No W3 leakage | explicit W2 exclusions |

## W2 authorization boundary

After W1 acceptance, W2 may implement the manual headless proof specified here.

W2 may not implement:

- Codex, offline-model, MCP, or A2A execution;
- arbitrary command execution;
- automated work selection;
- managed worktree/lease enforcement;
- GitHub mutation or CI dispatch;
- review correction loops;
- automatic merge/issue closure;
- Runenwerk frontend code;
- multi-agent coordination.

## Exit gate

W1 completes only when exact-head validation passes and critical review confirms that a competent implementation actor can build W2 without inventing material product, domain, security, storage, CLI, validation, dependency, or acceptance behavior.
