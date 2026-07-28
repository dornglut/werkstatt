# Human and Actor Workflows

## Purpose

Werkstatt uses one role-aware work model for manual, assisted, delegated, offline, and autonomous development. This document defines the target journeys and the points where authority, capabilities, approvals, validation, and acceptance differ.

## Shared lifecycle

Every operating mode follows the same logical lifecycle:

```text
understand authority
    -> prepare work contract
        -> select actor and policy
            -> prepare workspace
                -> execute
                    -> validate
                        -> review
                            -> accept or revise
                                -> reconcile
```

The number of visible steps and required artifacts scales with work risk.

## Manual human development

### Goal

Improve ordinary development without requiring a model.

### Journey

1. Select a project and accepted work item.
2. Refresh the authoritative issue, roadmap, architecture, branch, and PR state.
3. Review goal, scope, non-goals, dependencies, stop conditions, and exit gate.
4. Attach an existing checkout or create a task worktree from the accepted base.
5. Start a human execution attempt and acquire the workspace lease.
6. Launch the preferred editor and terminal.
7. Edit source using normal tools.
8. Run repository-defined focused and baseline commands.
9. Capture commands, revisions, outputs, and unavailable evidence.
10. Inspect the diff against the work contract and selected review dimensions.
11. Publish the task branch and create or update the pull request.
12. Process findings and rerun validation.
13. Request owner acceptance.
14. Observe merge, update authoritative work state, and release the workspace.

### Required product value

Werkstatt should reduce context hunting, repeated state transcription, evidence formatting, and missed authority while preserving the developer's existing editor, terminal, Git, and GitHub workflow.

## Interactive assistance

### Goal

Keep the human as active executor while delegating bounded analysis or transformations.

### Journey

1. Human starts the execution and owns the workspace.
2. Human selects a bounded operation such as explain, search consumers, propose plan, generate test, draft patch, or review diff.
3. Werkstatt creates a derived request packet from current authority and workspace state.
4. The assistant receives only permitted context and capabilities.
5. Proposed mutations remain suggestions or require explicit application approval.
6. Human reviews outputs, applies or rejects them, and continues the normal manual flow.

### Rule

The assistant does not acquire an independent writer lease while the human owns the same workspace. Applied changes occur through the human execution or a separately handed-off workspace.

## GPT web through GitHub

### Strengths

- source-grounded external research;
- architecture and workflow analysis;
- GitHub issue, PR, comment, and review coordination;
- bounded text changes through the connector;
- cross-repository planning and status review.

### Constraints

- no generic local checkout or command execution;
- connector operations may be file-size and API limited;
- native GitHub settings may require the web interface;
- current ChatGPT UI sessions are not a generic programmable actor runtime.

### Journey

1. Werkstatt or a human publishes a concise work contract in the authoritative issue.
2. A GPT web handoff references repository, parent issue, active issue, authority, accepted base, requested stage, output, and non-goals.
3. GPT web inspects GitHub and external sources through available tools.
4. It updates issues, bounded documentation, PRs, or review findings through ordinary GitHub surfaces.
5. Werkstatt refreshes authoritative GitHub state and imports produced evidence or findings.
6. Local implementation work is handed to a checked-out executor when required.

### Rule

GPT web is an actor adapter and coordination path, not a build or validation dependency of the product.

## Delegated local Codex execution

### Goal

Complete one decision-ready work item from accepted base to reviewable draft PR.

### Journey

1. Verify the work item is eligible for delegation.
2. Render a revision-bound work packet.
3. Select the Codex actor and capability policy.
4. Create an isolated worktree and acquire an expiring lease.
5. Start execution with exact repository, base, branch, authority, scope, commands, limits, and stop conditions.
6. Stream observable actions and approval requests.
7. Permit iterative edits and named validation commands within policy.
8. Stop on ambiguity, protected operations, unexpected scope, inherited baseline failure, moved authority, or resource limits.
9. Produce an execution receipt and review packet.
10. Publish a task branch and draft PR only if permitted.
11. Release the execution to human or independent review.

### Initial boundary

W4 stops at a reviewable draft PR. Codex does not merge or close the issue.

## Offline actor execution

### Goal

Prove the work model does not depend on one hosted model or vendor.

### Runtime requirements

An offline adapter requires more than a completion endpoint. It must provide:

- an agent loop;
- context assembly;
- file and command tools;
- workspace restrictions;
- structured events;
- approvals and cancellation;
- artifacts and handoff;
- stop and resource limits.

### Journey

The journey matches delegated Codex execution. Provider-specific details remain inside the actor adapter.

### Initial protocol

Use a generic subprocess protocol with structured messages and events before embedding a particular local model server into the core.

## Review-loop automation

### Goal

Process CI and reviewer findings without losing exact-head reviewability.

### Journey

1. Observe the current PR head and unresolved findings.
2. Verify the execution owns the branch and prior snapshots are current.
3. Classify each finding as actionable, decision-required, invalid, or already resolved.
4. Apply only accepted corrections within the work contract.
5. Rerun focused and canonical validation.
6. Publish the new head.
7. Invalidate validation and review evidence from prior heads.
8. Update the review packet and return to independent review.

### Rule

Review-loop automation cannot silently change the accepted design or expand scope. Material decisions return to the owner.

## Policy-controlled autonomous delivery

### Goal

Complete eligible work end to end without routine human intervention while preserving explicit policy and independent evidence.

### Eligibility conditions

- work source marks the item ready;
- contract is decision-complete;
- work class is allowed by policy;
- repository and path scopes are permitted;
- required validator and reviewer separation is available;
- resource and retry budgets are defined;
- merge and reconciliation authority are explicit;
- no unresolved security, architecture, or ownership decision remains.

### Journey

1. Select eligible work from the authoritative source.
2. Claim it with a bounded lease.
3. Prepare workspace and execution.
4. Implement, validate, and publish.
5. Process CI and review findings.
6. Obtain or satisfy required approvals.
7. Merge using the exact expected head if policy permits.
8. Observe accepted-main validation.
9. Reconcile issue, program, and local execution state.
10. Activate only explicitly dependent eligible work.

### Rule

Autonomous delivery is a policy profile, not the default behavior of Werkstatt.

## Failure, cancellation, and handoff

Every execution must support:

- safe cancellation;
- lease expiry;
- partial-result handoff;
- retry from a known base or current branch head;
- escalation with a bounded blocker report;
- explicit statement of commands not run and evidence unavailable;
- release of workspace and credentials;
- preservation of reviewable artifacts without claiming completion.

## Actor capability matrix

| Capability | Human | GPT web | Codex local | Offline actor | Autonomous service |
|---|---:|---:|---:|---:|---:|
| Inspect repository | Yes | Connector-limited | Yes | Yes | Yes |
| Investigate external sources | Yes | Strong | Tool-dependent | Tool-dependent | Policy-dependent |
| Make durable decisions | Owner role | Propose | Propose | Propose | Policy-controlled |
| Edit checked-out files | Yes | No | Yes | Yes | Yes |
| Perform bounded GitHub edits | Yes | Yes | Adapter-dependent | Adapter-dependent | Yes |
| Run local commands | Yes | No | Yes | Yes | Yes |
| Work fully offline | Yes | No | Environment-dependent | Yes | Deployment-dependent |
| Publish branch and PR | Yes | Yes | Policy-controlled | Policy-controlled | Policy-controlled |
| Review code | Yes | Strong | Strong | Variable | Separate reviewer preferred |
| Observe CI | Yes | Yes | Adapter-dependent | Adapter-dependent | Yes |
| Accept architecture | Owner role | No by default | No by default | No by default | Explicit policy only |
| Merge | Owner role | Explicit authorization | Explicit policy | Explicit policy | Explicit policy |
| Complete entire issue | Yes | Tool-limited | Yes with integration | Potentially | Yes for eligible classes |

## Human equivalence rule

Every agent operation must have a human equivalent exposed through the same work item, workspace, evidence, and review model.

The interface may optimize paths differently, but it must not create an agent-only hidden authority or completion path.

## Ergonomic requirements

- ordinary manual fields appear before advanced autonomy controls;
- stage, state, actor, role, and execution are visually distinct;
- every blocked or denied action explains the cause and corrective next step;
- generated packets are optional views, not required manual paperwork;
- a new contributor or agent can begin from repository, program issue, and active issue;
- common operations require few decisions while advanced policies remain inspectable;
- stale authority, moved heads, and expired evidence are visible rather than silently refreshed.
