# Werkstatt Policy and Security Model

## Status

Proposed W1 authority for roles, capabilities, policy evaluation, approvals, denials, conflicts, and threat-control traceability. Enforcement implementation begins no earlier than W3.

## Security principles

1. Deny by default.
2. Grant operations, not trust labels.
3. Scope every grant by project, repository, work item, workspace, actor, role, operation, resource, revision, time, and budget where applicable.
4. Separate read, propose, apply, publish, validate, review, approve, and accept.
5. Prompts, issue text, repository content, external pages, tool output, and model output never grant capabilities.
6. Capability availability does not imply policy permission.
7. Approval cannot widen an operation beyond the request it reviews.
8. Sensitive approvals are single-use and revision-bound by default.
9. Stale authority, moved revisions, conflicting writers, and ambiguous identity fail closed.
10. A worktree is not a security sandbox.
11. Independent validation and acceptance remain outside executor self-report.
12. Security failures produce observable denial records without leaking secrets.

## Actor, role, capability, and policy separation

```text
Actor
    has identity and technical adapter capabilities

RoleAssignment
    states responsibility in a project/work scope

OperationRequest
    asks to use one capability on one resource

PolicyEvaluation
    permits, denies, or requires approval under current facts

Approval
    may satisfy one policy condition

OperationExecution
    occurs only after all conditions remain valid
```

No layer substitutes for another.

## Capability namespace

Capabilities use stable dotted identifiers. Initial families:

### Project and authority

```text
project.read
project.configure
authority.read
authority.refresh
authority.bind
authority.annotate_local
```

`authority.annotate_local` creates explicitly non-authoritative notes. It cannot modify source authority.

### Repository and workspace

```text
repository.read_metadata
repository.read_source
repository.inspect_history
workspace.register
workspace.create
workspace.write
workspace.remove
workspace.recover
lease.acquire
lease.release
lease.takeover
```

### Commands and processes

```text
command.run_named
command.run_arbitrary
process.signal
process.cancel
```

### Network and secrets

```text
network.read
network.write
secret.request
secret.use
```

Secret values are never domain data. The domain records only named grant references and redacted use observations.

### Dependencies and workflows

```text
dependency.inspect
dependency.modify
lockfile.modify
workflow.inspect
workflow.modify
```

### Git and publication

```text
branch.create
branch.commit
branch.publish
branch.force_update
pull_request.create
pull_request.update
review.respond
pull_request.mark_ready
pull_request.merge
issue.comment
issue.update
issue.close
dependent_work.activate
```

### Validation, review, and acceptance

```text
validation.request
validation.observe
review.create
review.resolve_finding
approval.decide
acceptance.decide
reconciliation.record
```

A repository adapter may technically expose an operation not permitted by the current product phase. Domain capability registration does not authorize its use.

## Capability scope

A `CapabilityScope` is an intersection of constraints, never a union created implicitly from separate grants.

Possible dimensions:

- project IDs;
- repository IDs and remote identities;
- work item and contract IDs;
- workspace IDs and canonical path roots;
- path allowlists and denylists;
- branch names or patterns;
- expected base/head revisions;
- named command IDs;
- executable and argument constraints;
- network destinations, protocols, methods, and direction;
- secret grant names;
- dependency classes;
- workflow paths;
- artifact sensitivity classes;
- work classes and lifecycle stages;
- time interval;
- resource and retry budget;
- required isolation class;
- required approval class.

An empty scope denies the operation. Unknown dimensions fail closed when they affect the request.

## Policy layers

Policies are evaluated from most general to most specific but use explicit composition rules.

| Layer | Owns |
|---|---|
| Product invariant | non-overridable Werkstatt safety and authority rules |
| Organization policy | Dornglut-wide constraints such as protected branches and read-only validation |
| Repository policy | repository commands, protected paths, work classes, validation and publication rules |
| Project policy | local source bindings, trust class, retention, default actor modes |
| Work-contract policy | scope, non-goals, stop conditions, exact base, allowed operations |
| Execution policy | actor, role, workspace, budgets, requested capability subset |
| Approval amendment | one reviewed exception permitted by upper layers |

Composition:

1. A deny from a non-overridable invariant is final.
2. An explicit deny from any applicable layer wins over an allow unless that layer declares a specific overridable approval condition.
3. Allows intersect across scopes.
4. Missing policy for a sensitive operation is a deny.
5. Approval may satisfy only a declared `requires_approval` condition.
6. Approval cannot override product invariants, stale authority, identity ambiguity, moved expected revision, or unsupported isolation.
7. Policy evaluation is repeated immediately before the operation.

## Policy decision

`PolicyDecision` variants:

- `Allow` — all conditions currently satisfied;
- `Deny` — operation forbidden with stable reason code;
- `RequireApproval` — operation may proceed after a matching valid approval;
- `RequireStrongerIsolation` — operation cannot run in the selected workspace class;
- `RequireRefresh` — authority or revision observation is too old;
- `Conflict` — policies or source facts disagree and owner resolution is required.

Every decision records:

- request digest;
- policy sources and versions;
- evaluated facts and revisions;
- result;
- reason codes;
- missing conditions;
- evaluation time;
- validity window.

## Operation request

An `OperationRequest` contains:

- actor and actor session;
- active role assignment;
- project, work item, contract, execution, and workspace;
- capability;
- target and requested scope;
- exact expected revisions;
- command, diff, network destination, or operation preview where relevant;
- stated purpose;
- resource estimate;
- idempotency key;
- sensitivity classification.

Untrusted content may inform the purpose or preview but cannot alter capability or policy fields.

## Roles and default conflicts

Roles are not inherently exclusive, but some combinations require separation for a record to count as independent.

| Record or action | Allowed producer | Independence rule |
|---|---|---|
| Execution receipt | executor | may be same actor that performed work |
| Local check observation | executor or validator | marked non-independent if executor produced it |
| Exact-head validation receipt | validator | validator must be independent from modifying the validated revision |
| Review record | reviewer | policy may require reviewer distinct from executor |
| Approval | approver | approver cannot approve their own request where separation is required |
| Acceptance | acceptor or external authority | must follow repository/work-class authority |
| Operator recovery | operator | cannot silently alter project authority or acceptance |

W2 records these distinctions but does not implement automated actor separation.

## Approval model

### Approval request

Required fields:

- operation request digest;
- actor and role;
- project, work item, execution, workspace;
- capability and exact scope;
- source authority and expected revisions;
- preview or diff;
- reason approval is required;
- risk and sensitivity;
- requested duration/reuse;
- requested resource limits;
- policy source requiring approval.

### Approval decision

Variants:

- grant once;
- grant for bounded session;
- grant a narrower scope;
- deny;
- cancel because request is obsolete;
- require owner decision;
- require stronger isolation.

### Approval validity

An approval is valid only when all bound facts still match:

- actor identity and role;
- capability;
- project/work item/contract;
- workspace and canonical root;
- expected base/head;
- command or operation digest;
- destination or target;
- policy versions;
- expiry;
- prior consumption rules.

Revision-bound approvals expire when the expected revision moves.

### Reuse policy

Single-use by default for:

- arbitrary commands;
- network write;
- secret use;
- dependency or lockfile changes;
- workflow changes;
- destructive operations;
- force branch updates;
- publication outside task branches;
- merge and issue closure;
- lease takeover.

Session reuse is permitted only for explicitly homogeneous low-risk requests and remains narrower than a persistent policy grant.

## Denial model

A denial is a first-class outcome, not an adapter error.

A `DenialRecord` includes:

- request digest;
- denying policy and rule;
- stable reason code;
- safe explanation;
- whether narrowing is possible;
- whether owner review is required;
- no secret values or sensitive raw payload.

Common denial codes:

```text
policy.default_deny
policy.protected_branch
policy.path_outside_scope
policy.command_not_named
policy.arbitrary_command_forbidden
policy.network_destination_forbidden
policy.secret_not_granted
policy.dependency_change_forbidden
policy.workflow_change_requires_owner
policy.revision_moved
policy.authority_stale
policy.workspace_conflict
policy.isolation_insufficient
policy.resource_budget_exceeded
policy.role_conflict
policy.acceptance_authority_missing
```

## Conflict model

Conflicts differ from denials because accepted sources disagree or ownership is ambiguous.

Examples:

- issue contract base differs from repository `main` after an unreviewed update;
- local branch head moved outside the active execution;
- two authoritative documents claim the same responsibility;
- repository policy allows an operation while organization policy forbids it;
- an approval references a superseded contract;
- two actors claim the same workspace.

Conflict handling:

1. stop dependent operations;
2. preserve all observations;
3. identify owning authority for each disputed fact;
4. produce a bounded conflict report;
5. require explicit resolution;
6. refresh and reevaluate policy;
7. do not auto-merge or silently prefer local state.

## Resource and retry policy

`ExecutionBudget` dimensions:

- wall-clock duration;
- CPU time where observable;
- process count;
- memory advisory/limit where supported;
- disk and artifact size;
- stdout/stderr/event volume;
- network transfer;
- model token or monetary budget;
- number of command attempts;
- number of execution retries;
- number of review-loop iterations.

Budget exhaustion transitions execution to handoff, cancellation, expiry, or failure according to the contract. It never silently increases the limit.

Retry rules:

- transient adapter overload may retry with bounded exponential backoff and jitter;
- domain invariant, policy denial, moved revision, and material ambiguity are not automatic-retry conditions;
- terminal execution retry creates a new `ExecutionId`;
- prior activities and evidence remain linked and immutable;
- a retry must revalidate authority, policy, workspace, and approvals.

## Isolation classes

| Class | Intended use | Guarantees |
|---|---|---|
| Registered checkout | trusted manual W2 work | path and repository observation only; no sandbox claim |
| Git worktree | trusted bounded human/agent work | branch/source separation; not process isolation |
| Local sandbox/container | repository code with stronger command isolation | adapter-defined filesystem/process/network controls |
| Remote isolated executor | stronger process, credential, and host separation | remote adapter reports enforceable controls and evidence |

Policy requests an isolation capability, not a concrete container technology.

## Command policy

### Named command

A repository-owned command identity includes:

- stable command ID;
- display name and purpose;
- exact executable/arguments or repository script entrypoint;
- allowed working-directory rule;
- environment allowlist;
- timeout and output limit;
- network requirement;
- expected artifact classes;
- sensitivity and trust notes.

W2 supports only the repository canonical validator as a named command and invokes it through ordinary local process execution initiated by the human.

### Arbitrary command

Requires a separate capability, explicit command preview, suitable isolation, and approval unless a future accepted repository policy permits a tightly scoped class. It is outside W2.

### Environment

No ambient environment inheritance is assumed safe. Future execution adapters construct environment variables from:

- minimal platform baseline;
- repository command declaration;
- named secret grants;
- explicit approved additions.

## Network policy

Network read and write are independent.

A network scope contains:

- destination host or service identity;
- protocol and port;
- operation/method class;
- data classification permitted to leave the workspace;
- authentication grant reference;
- rate and transfer limits;
- audit/redaction rules.

W2 requires no Werkstatt-managed network capability. A human may use their existing Git/GitHub tooling outside Werkstatt’s process policy.

## Secret policy

Werkstatt domain stores:

- secret grant ID;
- provider and purpose;
- scope;
- expiry;
- redaction rule;
- usage observation.

It never stores secret values in domain events, packets, logs, evidence, errors, or review artifacts.

Secret access is outside W2.

## Threat-control traceability

| Threat | Domain control | Adapter/enforcement control | Independent evidence | Earliest proof phase |
|---|---|---|---|---|
| Authority confusion | typed ExternalRef, RevisionRef, fact classification, sync states | source-specific refresh and conflict mapping | source/revision displayed and validation tests | W2 observation; W3 enforcement |
| Prompt/instruction injection | capability and policy fields cannot originate from untrusted content | actor adapter separates instructions, data, and tools | adversarial fixture tests | W3/W4 |
| Filesystem escape | workspace root and path scope | canonical path checks and sandbox policy | changed-path audit and sandbox tests | W3 |
| Command abuse | named/arbitrary capability separation | command preview, process limits, deny privilege escalation | command-policy tests | W3 |
| Network exfiltration | independent network scopes and data classification | destination allowlist and network sandbox | denied/allowed integration tests | W3/W4 |
| Secret exposure | named opaque grants and redacted observations | secret broker and environment construction | redaction and no-leak tests | W3/W5 |
| Dependency changes | distinct capabilities and approval | manifest/lockfile changed-path inspection | diff and locked validation | W3/W4 |
| Workflow weakening | protected-path capability and owner approval | Git changed-path and workflow review adapter | exact-head CI and policy tests | W3/W5 |
| Git/publication abuse | expected revisions and publication capabilities | protected branch, expected-head, no-force defaults | branch/PR integration tests | W3/W5 |
| Duplicate writers | WriterLease invariant | workspace audit and heartbeat | conflict/recovery tests | W3 |
| Resource exhaustion | ExecutionBudget | process, output, network, model, retry limits | budget and cancellation tests | W3/W4 |
| Self-validation | distinct receipt types and role independence | CI/validator adapter identity | exact-head validation | W2 observation; W4 separation |
| Stale authority | freshness and conflict transitions | source refresh before sensitive operations | moved-head fixtures | W2 observation; W3 enforcement |
| Unsafe recovery | immutable terminal executions and new retry IDs | workspace audit before takeover/resume | crash/recovery tests | W3 |
| Protocol drift | opaque provider refs and adapter schema version | generated schemas/version checks | compatibility tests | adapter implementation phase |

## W2 policy subset

W2 implements a non-enforcing manual subset:

- display actor as human and role as executor;
- register one trusted checkout;
- record the repository canonical validator as a named command;
- show that no model, secret, network, arbitrary-command, publication, or merge capability is configured;
- reject project/workspace/revision mismatches;
- record local command observations and their non-independent status;
- render policy/security limitations honestly.

W2 does not claim sandboxing, lease enforcement, secret brokering, network enforcement, approval automation, or autonomous authority.

## Later-phase boundary

- W3 implements workspace, lease, command, approval, budget, cancellation, and recovery enforcement.
- W4 maps Codex approval requests and execution events through these domain contracts.
- W5 implements GitHub publication and reconciliation capabilities.
- W6 proves equivalent enforcement for an offline actor adapter.
- W9 requires a separate accepted merge/closure policy for eligible work classes.
