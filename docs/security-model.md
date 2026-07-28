# Werkstatt Security and Trust Model

## Status

Proposed W0 security baseline. W1 must turn these requirements into explicit domain contracts and W3 must prove their enforcement before delegated execution is accepted.

## Security objective

Werkstatt must support useful human and automated development without treating repository code, model output, commands, dependencies, workspaces, credentials, or external services as implicitly trusted.

The system should minimize authority, make sensitive actions observable, require explicit approval where policy demands it, and preserve independent validation.

## Trust domains

### Project authority

Issues, architecture documents, repositories, pull requests, validation, and merge records may be authoritative for project questions, but their content is not automatically safe to execute.

### Workbench process

Werkstatt owns local operational state and policy enforcement. A compromise here may affect all configured projects and credentials.

### Workspace

A checkout or worktree contains repository-controlled code and generated modifications. It is not a security sandbox.

### Actor runtime

A human, Codex, offline agent, script, or autonomous service may propose or execute actions. Actor output is untrusted until validated and reviewed according to policy.

### Command environment

Compilers, tests, package managers, hooks, build scripts, shell commands, and external tools may execute repository or dependency code.

### External systems

GitHub, model providers, package registries, remote MCP servers, A2A agents, and other network services have separate identities, permissions, availability, and data policies.

## Primary threats

### Authority confusion

A generated packet, cached view, model response, execution log, or local database row is mistaken for accepted issue, architecture, code, validation, or merge authority.

Controls:

- source identity and revision on every external or derived fact;
- explicit authoritative, derived, cached, generated, observed, unverified, and stale labels;
- no independent editing of derived project state;
- conflict handling that stops dependent work.

### Prompt or instruction injection

Repository files, issues, comments, external pages, or tool output attempt to grant commands, credentials, or scope beyond the accepted work contract.

Controls:

- issue and model text do not grant capabilities;
- policies are configured outside untrusted content;
- tool outputs are data, not executable authority;
- protected actions require explicit policy or approval;
- actor prompts distinguish authority from inspected content.

### Filesystem escape

Commands or actors modify files outside the intended workspace or protected paths.

Controls:

- workspace-root path enforcement;
- path allowlists and protected-path rules;
- separate handling for repository metadata and credentials;
- post-execution changed-path audit;
- stronger operating-system sandboxing for untrusted projects.

### Command abuse

An actor executes destructive, privileged, or unrelated commands.

Controls:

- named repository commands as the ordinary path;
- explicit capability for arbitrary commands;
- command preview and approval policy;
- time, process, output, and retry limits;
- denial of privilege escalation by default;
- complete command and result activity records.

### Network exfiltration

Repository code or actors send source, secrets, personal data, or generated artifacts to unauthorized services.

Controls:

- network disabled by default for high-risk execution;
- destination and protocol allowlists;
- separate read and write capabilities;
- clear disclosure of provider-bound data;
- secret redaction and data-classification policy;
- no ambient credentials in untrusted workspaces.

### Secret exposure

Tokens, SSH keys, environment variables, configuration files, or logs expose credentials.

Controls:

- named secret grants rather than inherited environments;
- least-privilege credentials scoped to project and operation;
- redaction before persistence or display;
- no secrets in prompts, packets, artifacts, or PRs unless explicitly authorized;
- revocation and rotation support;
- separate publisher and validator permissions where practical.

### Dependency and supply-chain changes

An actor adds malicious, unnecessary, or unreviewed dependencies or modifies lockfiles unexpectedly.

Controls:

- dependency modifications as a distinct capability;
- approval required for material dependency or source changes;
- locked validation;
- manifest and lockfile diff review;
- provenance and pinning requirements defined by the repository;
- package-manager scripts treated as executable code.

### Workflow modification

An actor changes CI or release workflows to weaken validation, gain credentials, or publish unreviewed output.

Controls:

- workflow modification capability separate from ordinary source writes;
- explicit owner approval;
- review of permissions, triggers, actions, and secrets;
- validation workflows remain read-only;
- no temporary source-authoring or transport workflow.

### Git and publication abuse

An actor force-pushes, writes to a protected branch, publishes unexpected paths, or merges a moved head.

Controls:

- task branches only by default;
- no force push unless a narrowly accepted recovery operation exists;
- expected base, branch, remote, and head checks;
- changed-path and commit review;
- merge pinned to the reviewed expected head;
- branch protection and independent CI.

### Duplicate writers

Humans or agents concurrently modify the same workspace or branch and invalidate evidence.

Controls:

- one writer per workspace and branch;
- bounded, visible, expiring leases;
- heartbeat and stale-lease recovery;
- explicit handoff;
- head movement invalidates prior snapshots and validation.

### Resource exhaustion

Actors consume unbounded time, tokens, disk, processes, network, or CI capacity.

Controls:

- per-execution budgets;
- command timeouts;
- bounded logs and artifacts;
- retry limits and backoff;
- cancellation;
- visibility into cost and resource use;
- no automatic infinite correction loop.

### Self-validation and false completion

The executor declares its own work correct or completed without independent evidence.

Controls:

- execution receipt separated from validation receipt and acceptance record;
- repository-defined validation;
- exact-head CI;
- review roles and findings;
- explicit acceptance transition;
- unavailable evidence reported honestly.

### Stale authority or confused deputy

An actor operates on an old issue body, design, base revision, branch head, or approval.

Controls:

- source revision and observation time;
- refresh before write, publish, review, and merge;
- moved authority invalidates derived packets and approvals where relevant;
- stop and reconcile conflicts rather than silently rebasing decisions.

## Policy dimensions

Werkstatt policies should cover at least:

| Area | Example policy values |
|---|---|
| Repository read | denied, metadata only, full source |
| Workspace | existing only, create worktree, isolated sandbox |
| Filesystem write | none, workspace only, selected paths |
| Named commands | allowlisted command identifiers |
| Arbitrary commands | denied, approval required, allowed within sandbox |
| Network read | none, allowlisted destinations, unrestricted |
| Network write | none, approval required, allowlisted destinations |
| Secrets | none, named grants, scoped credentials |
| Dependencies | unchanged, approval required, permitted classes |
| Workflows | read only, approval required |
| Git branches | local only, task branch publish, protected branch denied |
| Issues and PRs | observe, comment, update, create |
| Merge | denied, owner approval, allowed work classes |
| Issue closure | denied, after accepted merge, policy-controlled |
| Dependent work | no activation, owner approval, eligible-only activation |
| Resources | time, tokens, cost, disk, output, process and retry budgets |

## Capability principles

- deny by default;
- grant the smallest capability and scope needed;
- separate read, propose, apply, publish, validate, and accept;
- make capability origin and active policy visible;
- require reauthorization when work class, repository, actor, or scope changes;
- do not infer permission from actor identity alone;
- do not permit model or issue content to modify policy.

## Approval model

An approval should record:

- requested operation;
- actor and role;
- project, work item, execution, workspace, and revision;
- capability and scope;
- reason and risk;
- relevant diff or command preview;
- approver identity and role;
- expiry and reuse rules;
- decision and time.

Approvals should be single-use by default for destructive, secret, workflow, dependency, publication, and merge operations.

## Workspace isolation levels

### Registered checkout

Suitable for trusted manual work. Werkstatt observes but does not claim strong isolation.

### Git worktree

Provides source and branch separation. Suitable for trusted repository code under bounded command policy. Not a security sandbox.

### Operating-system sandbox or container

Required for stronger isolation of untrusted repositories or autonomous arbitrary command execution. Exact implementation is deferred to W1/W3.

### Remote isolated executor

May provide stronger process and credential separation. It remains an adapter and must return observable evidence.

## Logging and retention

Activity records should be useful for review and recovery without becoming an uncontrolled data store.

Requirements:

- redact secrets before persistence;
- bound stdout, stderr, model streams, and artifacts;
- classify sensitive project data;
- configure retention by project or execution class;
- permit deletion of local operational records without deleting project authority;
- retain immutable external evidence references where required;
- avoid storing private chain-of-thought; store actions, structured reasons, summaries, artifacts, and evidence instead.

## Validation and acceptance separation

### Execution receipt

Records what an actor attempted and observed.

### Validation receipt

Records independent checks against an exact revision.

### Review record

Records findings and conformance assessment.

### Acceptance record

Records the authorized transition into accepted project state.

No single receipt is a universal truth certificate.

## Phase security gates

| Phase | Required security result |
|---|---|
| W0 | Threats, trust domains, policy dimensions, and non-goals accepted |
| W1 | Typed capabilities, policies, approvals, state transitions, errors, and adapter security contracts designed |
| W2 | Manual workflow does not require secrets beyond normal developer tooling and cannot silently exceed repository authority |
| W3 | Workspace, command, lease, approval, resource, cancellation, and recovery enforcement tested |
| W4 | Codex execution constrained to one issue, workspace, branch, policy, and draft-PR handoff |
| W5 | GitHub permissions, exact-head observation, publication, review, and reconciliation proven |
| W6 | Offline runtime demonstrates adapter isolation and equivalent policy enforcement |
| W7 | UI clearly exposes policy, actor, role, approval, authority, and staleness |
| W8 | Review loops cannot expand scope, bypass decisions, or reuse stale evidence |
| W9 | Merge and issue closure limited to accepted work classes and exact reviewed heads |
| W10 | Multi-actor scheduling preserves isolation, least privilege, and conflict recovery |

## Stop conditions

Persistent delegated or autonomous execution must stop if:

- the sandbox or command policy is undefined;
- required secrets cannot be scoped and redacted;
- validation independence is unavailable;
- branch and head identity cannot be proven;
- actor or policy identity is ambiguous;
- work requires a material unaccepted design decision;
- the execution observes unexpected protected-path, dependency, workflow, or permission changes;
- a lease conflict or stale authority cannot be reconciled safely.
