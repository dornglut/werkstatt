# W1 Work-Domain and System Design

## Status

Proposed W1 authority for [issue #6](https://github.com/dornglut/werkstatt/issues/6). It becomes accepted only through review and merge of its owning pull request.

## Purpose

W1 turns the accepted W0 product boundary into a decision-complete design for W2, the human-first headless CLI proof.

W1 defines the work domain, state transitions, authority synchronization, workspace and execution semantics, policy and security model, evidence and review model, adapter ports, storage boundary, CLI contract, implementation plan, and acceptance tests.

W1 contains no product implementation.

## Accepted foundation

- Dornglut ADR 0005 authorizes the bounded Werkstatt pilot.
- W0 defines Werkstatt as a human-first engineering workbench with optional policy-controlled execution.
- external project authority remains external;
- humans and automated actors use one role-aware work model;
- generated assistance and operational state cannot become parallel project authority;
- the headless core remains independent of Runenwerk, models, forges, and concrete storage;
- W2 proves manual value before agent integration;
- W3 proves execution safety before delegated execution.

## W1 decision set

| Decision area | Canonical detail | Binding result |
|---|---|---|
| Domain vocabulary, ownership, invariants, states, and errors | [Work domain](work-domain.md) | Each concept is owned once; work, execution, evidence, and acceptance remain distinct |
| Roles, capabilities, policy, approvals, denials, and threat controls | [Policy and security](policy-and-security.md) | Capabilities are deny-by-default and never granted by prompts or inspected content |
| External authority, freshness, workspaces, leases, cancellation, and recovery | [Authority and workspace](authority-workspace.md) | External authority is referenced rather than copied; one writer owns a workspace at a time |
| Activities, artifacts, receipts, findings, review, validation, acceptance, and reconciliation | [Evidence and review](evidence-review.md) | Execution reports, independent validation, review, and acceptance remain separate records |
| Adapter ports, protocol mappings, SQLite schema design, and migrations | [Ports and storage](ports-and-storage.md) | Concrete systems remain adapters; SQLite stores local operational state only |
| Human-facing W2 behavior and journeys | [W2 CLI contract](w2-cli-contract.md) | W2 is useful without any model and preserves existing editor, terminal, Git, and GitHub workflows |
| W2 package, file, dependency, test, and delivery plan | [W2 implementation specification](w2-implementation-spec.md) | A competent implementation actor can build W2 without inventing material behavior or architecture |

## Core architecture

```text
CLI and future Runenwerk frontend
    -> application use cases and projections
        -> work-domain types and rules
            <- ports implemented by concrete adapters

External authority
    Git, repository files, GitHub issues and pull requests, CI, merge records

Local operational state
    projects, authority observations, workspace registrations, executions,
    activities, evidence references, findings, approvals, and preferences
```

Dependency direction:

```text
domain
    depends on no adapter, database, forge, model, process, or graphical host

application
    depends on domain and port traits

adapters
    depend on domain/application contracts

frontends
    compose application services and adapters
```

## Stable domain versus adapter vocabulary

Werkstatt does not adopt provider concepts as internal authority:

| External concept | Adapter treatment | Werkstatt concept |
|---|---|---|
| Git branch, commit, worktree | repository/workspace observations | RepositoryRef, RevisionRef, Workspace |
| Git worktree lock | administrative Git protection | not the Werkstatt writer lease |
| GitHub issue or other tracker item | authoritative work-source record | WorkItemRef and imported WorkSnapshot |
| GitHub pull request | publication and review authority | ChangeProposalRef |
| CI workflow run | independent validation observation | ValidationReceiptRef |
| Codex thread and turn | actor-runtime session and interaction | ActorSession and Execution activity |
| MCP host/client/server session | tool and context adapter connection | AdapterSession |
| A2A task | remote actor-runtime task | remote Execution correlation |
| SQLite transaction | local persistence mechanism | not a work or execution state transition |

Provider-specific identifiers remain opaque adapter values with an explicit provider kind and versioned payload boundary.

## Human-first ordinary path

The ordinary W2 path exposes only:

1. project;
2. accepted work item;
3. authority summary;
4. registered workspace;
5. named validation command;
6. evidence summary;
7. review packet;
8. reconciliation status.

Actors, advanced policies, approvals, remote protocols, autonomous selection, retries, budgets, sandboxes, and multi-agent coordination do not burden the ordinary manual path.

## Cross-cutting invariants

1. One project binds at most one authoritative work source at a time.
2. External authority is stored as references and observations, never as an independently editable project copy.
3. A derived view identifies its sources, observed revisions, generation time, and staleness.
4. Work-item state, lifecycle stage, execution state, decision state, approval state, lease state, and synchronization state are distinct.
5. A workspace has at most one active writer lease.
6. An actor performs an operation under a role and an evaluated policy decision.
7. Technical capability never grants decision or acceptance authority.
8. Execution receipts cannot serve as independent validation receipts.
9. Validation and review evidence bind to an exact revision.
10. A moved revision invalidates evidence and approvals whose scope includes that revision.
11. Acceptance requires an authoritative acceptance observation, not local completion alone.
12. Local operational records may be deleted without deleting external project authority.
13. W2 performs no delegated actor execution.
14. W2 introduces no hidden agent-only workflow or authority.

## Structured error contract

Every user-visible failure contains:

- stable error code;
- failed operation;
- cause category;
- relevant project, work item, workspace, execution, authority, or revision identifiers;
- whether retry is safe;
- corrective next action;
- optional underlying adapter diagnostic;
- no secret or private-data leakage.

The CLI renders a concise message by default and structured JSON when requested.

## W1 conformance matrix

| W1 requirement | Design evidence | Verdict |
|---|---|---|
| One human/agent work model | Work domain and policy model | Bound |
| Human-only usefulness | W2 CLI contract and acceptance tests | Bound |
| External authority preserved | Authority-reference and synchronization rules | Bound |
| State meanings separated | Explicit state machines | Bound |
| Deny-by-default capability policy | Policy evaluation and approval contracts | Bound |
| Workspace conflict safety | Lease and revision rules | Bound |
| Independent validation | Receipt and acceptance separation | Bound |
| Replaceable adapters | Port contracts and provider mappings | Bound |
| SQLite is local operational storage only | Logical schema and migration policy | Bound |
| Runenwerk does not own the core | Dependency direction | Bound |
| No implementation in W1 | File and phase boundary | Bound |

## W2 authorization boundary

W1 authorizes W2 only after this design is accepted.

W2 may implement the manual headless proof specified here. W2 may not implement:

- Codex, offline-model, MCP, or A2A actor execution;
- arbitrary command execution;
- automatic work selection;
- workspace creation or lease enforcement beyond registering and validating one human-owned workspace;
- GitHub write automation;
- review correction loops;
- automatic merge or issue closure;
- Runenwerk frontend code;
- multi-agent coordination.

## References

The protocol and storage decisions use current primary sources listed in [W0 external references](references.md). Exact provider schemas remain version-bound adapter concerns and must be regenerated or rechecked during their implementation phase.
