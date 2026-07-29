# Werkstatt Roadmap

This roadmap owns the durable outcome sequence. GitHub issues and pull requests own active branches, blockers, review state, exact revisions, and validation evidence.

## Product progression

```text
W0 product investigation
    -> W1 work-domain and system design
        -> W2 human-first headless proof
            -> W3 safe workspace and policy runtime
                -> W4 delegated Codex proof
                    -> W5 GitHub lifecycle integration
                        -> W6 offline actor proof
                            -> W7 Runenwerk frontend
                                -> W8 review-loop automation
                                    -> W9 policy-controlled autonomous delivery
                                        -> W10 multi-agent and remote execution
```

## Phase matrix

| Phase | Durable outcome | Depends on | Exit gate |
|---|---|---|---|
| W0 | Product boundary, retrospective, authority model, actor journeys, security baseline, alternatives, success criteria, and sequence | organization authorization | accepted at `5a8a6c080a7cab0894e78d2d4dd28ee135b0808f` |
| W1 | Domain vocabulary, invariants, states, policy, authority sync, workspace/recovery, evidence/review, ports/storage, and W2 contract | W0 | accepted at `a87d8fbc5ac1e4e31d5a3f6b601237cf8e3b8cd9` |
| W2 | Model-free CLI proof for a comparative real manual task | W1 | human workflow shows net value without duplicate authority |
| W3 | Managed workspace, writer lease, command policy, approvals, persistence recovery, cancellation, and budgets | W2 | safe single-writer execution substrate passes fault/security proofs |
| W4 | One delegated Codex actor completes bounded work to validated draft PR | W3 | issue, base, workspace, actor, branch, validation, receipt, and handoff proven |
| W5 | GitHub issue, branch, PR, review, CI, merge, and reconciliation integration | W4 | connected lifecycle preserves external authority and exact-head evidence |
| W6 | Generic process/offline actor adapter | W5 | provider portability with equivalent policy enforcement |
| W7 | Runenwerk graphical frontend over headless core | stable human/delegated workflows | UI useful without a model and exposes authority, policy, evidence, and review clearly |
| W8 | Automated CI observation and review-correction loop | W5 and W7 | bounded correction, republishing, revalidation, and escalation reliable |
| W9 | Policy-controlled end-to-end delivery for selected work classes | W8 and explicit policy | eligible work accepted without exceeding authority |
| W10 | Multiple actors, remote execution, and interoperability | W9 reliability evidence | coordination preserves isolation, least privilege, reviewability, and recovery |

## Current W2 delivery sequence

| Delivery | State | Boundary | Next gate |
|---|---|---|---|
| W2A | Accepted | package, typed domain foundation, and SQLite schema/diagnostics | accepted-main validation proven by run `30441529835` |
| W2B | Active | read-only Git, local Markdown/JSON, and optional read-only `gh` observations | independent review, merge, and accepted-main validation |
| W2C | Blocked | guided start and status | W2B acceptance |
| W2D | Blocked | validation, evidence, review, and reconciliation | W2C acceptance |
| W2E | Blocked | fixture, counterbalanced pilot, and closeout | W2D acceptance |

W2B does not implement the ordinary command path, run validators, edit source, mutate Git or GitHub, or introduce W3 enforcement.

## Global gates

Every phase preserves:

- human-only product value;
- one authoritative work source per project;
- separation of authority, assistance, execution, validation, and acceptance;
- role-aware capabilities and approvals;
- exact repository/workspace/revision identity;
- independent validation;
- explicit command, filesystem, network, secret, dependency, workflow, publication, merge, resource, cancellation, and recovery boundaries;
- comprehensible vocabulary, ordinary paths, actionable errors, and visible staleness;
- no generated or operational artifact as parallel planning or architecture authority;
- no product dependency on one model, connector, forge, protocol, database API, or graphical host.

## Readiness rules

- W2 proves manual usefulness before agent integration.
- W3 proves execution safety before delegated code generation.
- W4 stops at a reviewable draft PR.
- GitHub review correction follows connected lifecycle integration.
- Automatic merge requires a separately accepted policy and reliability evidence.
- Multi-agent operation follows dependable single-agent execution.
- Runenwerk consumes the headless core; it does not own the domain.
- Each delivery becomes active only after its predecessor is accepted and accepted-main validation is proven.

## Accepted foundation

- [ADR 0005](https://github.com/dornglut/engineering/blob/main/adrs/0005-authorize-werkstatt-pilot.md)
- [W0 accepted architecture](docs/w0-investigation.md)
- [W1 design](docs/w1-design.md)
- W2A accepted at `bd4f12f770fa82d25657f41fd8cff5e3a299a8a1`

## Program authority

- [Program issue #1](https://github.com/dornglut/werkstatt/issues/1)
- [W2 umbrella issue #8](https://github.com/dornglut/werkstatt/issues/8)
- [Active W2B issue #10](https://github.com/dornglut/werkstatt/issues/10)
