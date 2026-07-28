# W0 Product and Workflow Investigation

## Status

Proposed W0 investigation authority. This document becomes accepted only through review and merge of its owning pull request.

## Question

Can Dornglut build a human-first engineering workbench that also supports assisted, delegated, offline, and eventually autonomous development without recreating the churn, duplicate authority, and process concentration of the retired Runenwerk workflow platform?

## Scope

This investigation covers:

- the retired Runenwerk workflow system and organization-level ForgeOps proposal;
- human, GPT web, Codex, offline-agent, and autonomous workflows;
- authority, state, role, capability, security, and validation boundaries;
- product scope and explicit non-goals;
- a headless core and future Runenwerk frontend;
- the minimum evidence needed before implementation.

It does not authorize Rust implementation or persistent orchestration.

## Evidence inspected

Primary Dornglut evidence:

- `dornglut/runenwerk` issue #122 and pull requests #123 and #124;
- `dornglut/engineering` ADR 0002 and ADR 0003;
- Dornglut authority, repository, and validation standards;
- current Runenwerk and RunenUI work-tracking and agent contracts;
- current Werkstatt program issues #1 and #2;
- organization reevaluation issue `dornglut/engineering#23`.

External architectural references considered:

- OpenAI Codex local, cloud, SDK, and app-server execution patterns;
- OpenAI Symphony's issue-driven isolated-workspace orchestration model;
- GitHub Spec Kit's specification, planning, task, and consistency workflow;
- Model Context Protocol for tool and context interoperability;
- A2A for possible later remote-agent task interoperability;
- GitHub Actions concurrency and exact-head validation patterns;
- NIST secure-development guidance for integrating security into the normal lifecycle.

These systems are references, not imported authority.

## Retired-system retrospective

### What the old Runenwerk system tried to provide

The retired workflow platform attempted to make complex repository work repeatable and agent-friendly through:

- investigation and design gates;
- production tracks and phase contracts;
- execution locks;
- machine-readable contract packs;
- generated worker prompts;
- batch and worktree orchestration;
- truth conformance specifications and certificates;
- structured roadmap and production databases;
- generated diagrams and status material;
- multiple validation gate profiles;
- closeout and evidence generation.

Several goals were valid:

- reduce repeated prompt writing;
- preserve accepted scope and non-goals;
- make long programs understandable;
- prevent conflicting writers;
- provide explicit implementation handoffs;
- record validation and completion evidence;
- support automated or semi-automated execution;
- avoid losing architectural context across sessions.

### What failed

The failure was not simply that files were generated. It was concentration and authority confusion.

The system accumulated overlapping control planes:

- GitHub issues and generated track state both represented work;
- roadmaps and structured production databases both represented sequence and status;
- specifications, contract packs, prompts, and execution manifests restated the same scope;
- locks and ledgers became workflow prerequisites rather than narrow concurrency mechanisms;
- truth certificates overstated what execution evidence could prove;
- multiple gate vocabularies obscured the one baseline required for merge;
- repository process policy leaked into product tests;
- generated artifacts required synchronization and cleanup;
- the workflow platform became a large product inside Runenwerk.

The deletion in PR #124 removed hundreds of files and more than one hundred thousand lines. That scale is evidence that the workflow implementation had become disproportionate to the product work it was intended to assist.

### What should remain retired

Do not restore:

- a workflow platform inside Runenwerk;
- duplicate roadmap or issue databases;
- truth-certificate authority;
- process-only activation branches or commits;
- temporary source-export or source-writing validation workflows;
- multiple mandatory validation gate taxonomies;
- global execution locks;
- generated diagrams that require manual synchronization;
- arbitrary commands derived directly from issue or model text;
- a provider-neutral publisher protocol before repeated need is proven.

### What should be reconsidered

The following concepts remain useful when properly separated:

- generated work packets derived from accepted authority;
- current-state, ownership, and conformance matrices;
- isolated workspaces;
- one-writer leases scoped to a workspace or branch;
- explicit actor capabilities and approvals;
- execution attempts and activity history;
- generated execution and review summaries;
- automatic stale-state detection;
- review and reconciliation assistance;
- later bounded orchestration of decision-complete work.

The revised rule is not "no generated state." It is:

> Generated and operational artifacts must remain derived, bounded, inspectable, replaceable, and unable to become parallel project authority.

## Product problem

Software work is spread across repositories, architecture documents, issues, branches, pull requests, CI, terminals, editors, review comments, model conversations, and human memory.

The recurring problems are:

- difficulty understanding which authority applies;
- loss of context between sessions and actors;
- manually rebuilding prompts and implementation plans;
- unclear phase, stage, state, and next action;
- weak handoffs between research, implementation, review, and acceptance;
- duplicated or stale planning status;
- poor visibility into what an agent changed or proved;
- no shared model for manual and autonomous execution;
- unsafe expansion from "agent can run commands" to "agent is authorized to decide or merge";
- no integrated human-readable view of work, architecture, evidence, and execution.

## Product decision

Werkstatt should be a human-first engineering workbench with an optional policy-controlled execution harness.

Its primary value is:

1. make accepted work understandable;
2. connect work to its real authority;
3. create or attach safe workspaces;
4. support humans and agents through one work contract;
5. collect observable execution and validation evidence;
6. make review, handoff, and reconciliation explicit;
7. automate only the capabilities permitted by policy.

The product must be useful without a model.

## Product users and actors

### Human developer

Needs a concise view of goal, authority, dependencies, workspace, commands, diff, validation, findings, and next action while retaining their editor and terminal.

### Human reviewer or owner

Needs requirement-to-diff traceability, architecture and security findings, exact-head validation, unresolved uncertainty, and explicit acceptance controls.

### GPT web

Strong for research, architecture, GitHub issue and PR coordination, bounded documentation edits, and review through the connector. It is not a generic local command executor.

### Local Codex

Strong first delegated actor for checked-out source editing, command execution, validation, and iterative correction. It requires explicit workspace and capability policy.

### Offline actor runtime

Provides local or private execution through a generic runtime adapter. A model endpoint alone is insufficient; the adapter also needs an agent loop, tools, context, workspace, event reporting, and stop conditions.

### Autonomous service

A later actor that may select eligible work, execute, validate, publish, process review, merge, and reconcile only under an accepted policy and after single-task reliability is proven.

## Human-first requirement

Werkstatt fails as a product if it is useful only as an agent dashboard.

The first implementation proof must allow a human to:

- register a repository;
- select accepted work;
- see authority, scope, non-goals, and dependencies;
- create or attach a checkout;
- launch their preferred editor and terminal;
- run repository-defined commands;
- collect evidence without rewriting it manually;
- inspect the diff against the contract;
- publish through normal Git and GitHub mechanisms;
- review and reconcile accepted work;
- complete the flow without configuring a model.

W2 must compare the same real task with and without Werkstatt using simple measures such as time to understand, number of sources inspected, manual state copying, missed authority, stale-state findings, and evidence-formatting effort.

## Actor-neutral but role-aware model

Humans and automated systems share the same work model, but capabilities do not imply authority.

Initial roles:

| Role | Responsibility |
|---|---|
| Requester | States a problem or desired outcome |
| Owner | Owns product or architecture decisions |
| Investigator | Establishes current reality and uncertainty |
| Designer | Proposes target behavior and boundaries |
| Executor | Changes files or produces artifacts |
| Validator | Runs independent checks |
| Reviewer | Assesses correctness and conformance |
| Approver | Authorizes gated operations |
| Acceptor | Makes a result accepted project state |
| Operator | Maintains the execution environment |

One actor may hold several roles, but every action records the active role.

## Authority, assistance, execution, and validation

### Authority

Externally or durably owned facts such as source, tests, ADRs, issues, roadmaps, PRs, CI, and merge records.

### Assistance

Derived work packets, matrices, summaries, checklists, projections, and review packets. Each records source, source revision, generation time, and staleness.

### Execution

Temporary workspaces, actor sessions, leases, approvals, retries, activity, and resource limits.

### Validation

Independent commands, CI, review findings, and manual or operational proof.

No plane silently overrides another.

## State model decision

Werkstatt must not use one overloaded status.

### Work-item lifecycle

`captured`, `ready`, `active`, `blocked`, `in_review`, `accepted`, `completed`, `cancelled`.

### Lifecycle stage

`intake`, `investigation`, `decision`, `delivery`, `verification`, `reconciliation`, `observation`, `reevaluation`.

### Execution attempt

`queued`, `claimed`, `preparing`, `running`, `waiting_for_input`, `waiting_for_approval`, `validating`, `publishing`, `handing_off`, `succeeded`, `failed`, `cancelled`, `expired`.

### Decision artifact

`draft`, `proposed`, `accepted`, `superseded`, `rejected`.

Exact names may be refined in W1, but the separation is binding.

## Work contracts and specification policy

Werkstatt should be contract-gated, not universally spec-driven.

Every significant task needs enough information to avoid inventing a material product, architecture, safety, or compatibility decision. The contract can range from a PR description for a small fix to an accepted design and implementation specification for architecture or extraction work.

A separate specification is required only when the issue and accepted design are insufficient for a competent actor to implement safely.

Generated plans and task breakdowns may derive from the contract. They do not become current-behavior authority.

## Product boundary

Werkstatt initially owns four capability groups:

### Work comprehension

- authority navigation;
- program and phase overview;
- dependency and blocker views;
- work contracts and matrices;
- next valid action;
- staleness detection.

### Workspace execution

- checkout or worktree attachment;
- exact base and branch identity;
- editor and terminal integration;
- named commands;
- evidence capture;
- diff inspection;
- branch and PR publication.

### Delegation and orchestration

- work-packet rendering;
- actor selection;
- capability policy;
- isolated execution;
- activity streaming;
- approvals, retries, cancellation, and handoff.

### Review and acceptance

- requirement-to-diff views;
- tests and exact-head evidence;
- architecture, security, migration, usability, and documentation findings;
- acceptance and reconciliation controls.

## Explicit product non-goals

The initial product is not:

- a replacement for Git, GitHub, CI, or repository roadmaps;
- a general business workflow engine;
- a full code editor or IDE;
- a model hosting platform;
- a distributed scheduler;
- a multi-agent swarm system;
- an architecture authority for other repositories;
- a universal provider-neutral publisher;
- an autonomous merge service by default.

## Architecture decision

Build a headless core independent of Runenwerk, model providers, forges, and concrete storage.

Use concrete first adapters behind narrow interfaces:

- Git;
- GitHub for Dornglut work and delivery authority;
- SQLite for local operational state;
- named repository commands for validation;
- Codex as the first delegated actor;
- a generic subprocess protocol for offline actor runtimes;
- Runenwerk as the later graphical frontend host.

Do not split into many crates at bootstrap.

## Persistence decision

Use a relational operational database for current local state and a separate append-only activity journal for observability.

Do not require full event sourcing or event replay to reconstruct every current projection.

Werkstatt-owned state may include projects, authority bindings, workspaces, executions, actors, policies, approvals, findings, generated artifacts, and preferences.

Project authority remains external.

## Security finding

A Git worktree is not a sandbox.

Autonomous execution may run untrusted repository code, build scripts, package hooks, tests, downloaded dependencies, or model-proposed commands. W1 must bind explicit controls for:

- filesystem scope;
- named versus arbitrary commands;
- network access;
- secret access and log redaction;
- dependency and workflow changes;
- destructive operations;
- branch publication;
- protected branches;
- validation independence;
- merge and issue closure;
- resource limits, cancellation, retry, and recovery.

No autonomous implementation may proceed without this policy model.

## Runenwerk frontend decision

The future graphical frontend should use Runenwerk and RunenUI as a major dogfooding application.

The frontend may provide:

- portfolio and program matrices;
- work-item and authority views;
- workspace and execution timelines;
- activity streams;
- evidence and approval inboxes;
- diff and conformance review;
- architecture exploration;
- resource and cost views.

The headless core remains usable through a CLI and external adapters.

## Alternatives

### Restore the deleted Runenwerk workflow platform

Rejected. It would restore the same ownership and churn problems and couple workflow infrastructure to the integration product.

### Reopen ForgeOps as designed in ADR 0002

Rejected for the pilot. It generalizes model, forge, executor, publisher, and conformance protocols before one real product workflow is proven.

### Adopt Spec Kit unchanged

Rejected. Its structured refinement is useful, but mandatory specification bundles and specification-primary authority conflict with Dornglut's code, issue, and ADR authority model.

### Clone Symphony

Rejected. Symphony is useful evidence for issue-driven orchestration and isolated workspaces, but Werkstatt also requires first-class manual development, review, architecture comprehension, and future Runenwerk UI.

### Build the Runenwerk GUI first

Rejected. The UI would harden an unproven domain model and delay evidence about manual workflow value.

### Build only an agent orchestrator

Rejected. It would not satisfy the human-first product requirement and would be less useful when no model is configured.

### Keep using ad hoc prompts and tools indefinitely

Rejected as the long-term target. It preserves repeated context rebuilding, weak handoffs, and poor execution visibility. It remains the comparison baseline for the pilot.

## Success criteria

| Criterion | W0/W1 target | Later proof |
|---|---|---|
| Human-only usefulness | Binding product requirement | W2 real-task comparison |
| Authority integrity | Explicit four-plane model | No duplicated editable authority |
| Actor neutrality | Domain contains roles and capabilities, not model-specific types | Codex and offline adapters share ports |
| Security | Threats and policy dimensions identified | W3 fault and policy tests |
| Delegated execution | Bounded architecture and stop conditions | W4 draft-PR proof |
| GitHub integration | External authority rules defined | W5 connected lifecycle |
| Runenwerk viability | Frontend depends on headless core | W7 usable application |
| Complete autonomy | Delayed and policy-controlled | W9 selected work class |
| Maintenance value | Pilot metrics defined | Repeated use exceeds upkeep |
| Churn resistance | Minimal artifacts and no duplicate databases | Adoption audit |

## Organization decision required

Before persistent execution-platform implementation, `dornglut/engineering#23` should decide:

- whether ADR 0003 is narrowed or superseded;
- whether `application` becomes a repository profile;
- which derived artifacts and execution state are permitted;
- security and acceptance constraints;
- evidence required before autonomous delivery;
- how the general Dornglut change lifecycle applies to Werkstatt.

## W0 conclusion

The product direction is coherent and worth continuing.

The correct sequence is:

1. accept W0 product and authority findings;
2. complete W1 domain, policy, security, and adapter design;
3. obtain the organization decision before persistent orchestration work;
4. implement W2 as a model-free human workflow proof;
5. add workspace safety before delegated actors;
6. prove one Codex task to draft PR;
7. integrate GitHub and one offline actor;
8. build the Runenwerk frontend;
9. add review loops and autonomy only after measured reliability.

W1 is ready to be designed after W0 acceptance. Rust implementation is not yet authorized.
