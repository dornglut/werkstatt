# Werkstatt Roadmap

This roadmap owns the durable outcome sequence. GitHub issues and pull requests own active branches, blockers, review state, and validation evidence.

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
| W0 | Product boundary, retrospective, authority model, actor journeys, security baseline, alternatives, success criteria, and implementation order | Organization reevaluation issue | W1 can be designed without inventing product, authority, security, or workflow decisions |
| W1 | Work-domain vocabulary, roles, capabilities, state machines, contracts, policies, evidence, adapter ports, persistence, and W2 specification | W0 accepted | Human and agent workflows use one role-aware model; W2 is implementation-ready |
| W2 | Model-free CLI proof for a real manual development task | W1 accepted and organization authorization | A human can understand, execute, validate, review, and reconcile real work more easily with Werkstatt |
| W3 | Git workspace, one-writer leases, command policy, approvals, SQLite operational state, activity journal, cancellation, and recovery | W2 accepted | Safe single-workspace execution substrate passes fault and policy proofs |
| W4 | One delegated Codex actor completes a bounded issue to a validated draft PR | W3 accepted | One issue, exact base, workspace, writer, branch, validation, receipt, and review handoff proven |
| W5 | GitHub issue, branch, PR, review, CI, merge, and reconciliation integration | W4 accepted | Connected lifecycle preserves external authority and exact-head evidence |
| W6 | Generic subprocess actor protocol proven with one offline model runtime | W5 accepted | Actor portability demonstrated without provider-specific domain types |
| W7 | Runenwerk graphical frontend over the headless core | Human and delegated workflows stable | Workbench UI is useful without a model and exposes authority, execution, evidence, approvals, and review clearly |
| W8 | Automated CI observation and review-correction loop | W5 and W7 accepted | Bounded corrections, republishing, revalidation, and escalation are reliable |
| W9 | Policy-controlled end-to-end delivery for selected work classes | W8 accepted and explicit owner policy | Eligible work can be claimed, executed, validated, reviewed, merged, and reconciled without exceeding authority |
| W10 | Multiple actors, remote execution, and interoperability | W9 reliability evidence | Coordinated work preserves workspace isolation, authority, reviewability, and recovery |

## Global gates

Every phase must preserve:

- human-only product value;
- one authoritative work source per project;
- separation of authority, assistance, execution, and validation;
- role-aware capabilities and approvals;
- exact repository and workspace identity;
- independent validation;
- security boundaries for commands, filesystem, network, secrets, dependencies, workflows, publication, and merge;
- comprehensible vocabulary, ordinary paths, actionable errors, and visible staleness;
- no generated or operational artifact as parallel planning or architecture authority;
- no product dependency on one model, connector, forge, or graphical host.

## Readiness rules

- W0 and W1 contain no product implementation.
- W2 proves manual usefulness before any agent integration.
- W3 proves execution safety before delegated code generation.
- W4 stops at a reviewable draft PR.
- Automatic review correction follows connected GitHub integration, not precedes it.
- Automatic merge requires a separately accepted policy and demonstrated reliability.
- Multi-agent operation follows dependable single-agent execution.
- The Runenwerk frontend consumes the headless core; it does not own the domain.

## Program authority

- [Program issue #1](https://github.com/dornglut/werkstatt/issues/1)
- [W0 issue #2](https://github.com/dornglut/werkstatt/issues/2)
- [Organization reevaluation dornglut/engineering#23](https://github.com/dornglut/engineering/issues/23)
