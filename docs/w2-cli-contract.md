# W2 Human-First CLI Contract

## Status

Proposed W1 authority for the W2 command-line product behavior. W2 implementation begins only after W1 is accepted.

## W2 mission

Prove that Werkstatt improves a real manual development workflow without any model, agent runtime, autonomous service, or graphical frontend.

W2 helps a human:

- register a repository and work source;
- understand accepted work and applicable authority;
- register an existing checkout;
- verify repository/base/head state;
- run the repository-owned validation command;
- capture evidence honestly;
- inspect changes against the work contract;
- generate a review packet;
- observe acceptance and reconcile local state.

The human continues to use their preferred editor, terminal, Git, and GitHub workflow.

## W2 non-goals

W2 does not:

- edit source files;
- create worktrees or branches;
- enforce writer leases;
- run arbitrary shell commands;
- manage credentials or secrets;
- perform network requests directly;
- run Codex or offline models;
- publish branches or pull requests;
- dispatch CI;
- update or close issues;
- merge changes;
- implement Runenwerk UI;
- select work autonomously;
- replace repository or GitHub authority.

## Installation and invocation

Binary name:

```text
werkstatt
```

General syntax:

```text
werkstatt [GLOBAL OPTIONS] <COMMAND> [OPTIONS]
```

Global options:

| Option | Meaning |
|---|---|
| `--data-dir <PATH>` | override local profile directory for testing/portable use |
| `--project <ID_OR_NAME>` | select project when command is project-scoped |
| `--format human|json` | output format; default `human` |
| `--no-color` | disable color |
| `--verbose` | include adapter diagnostics and source references |
| `--quiet` | emit only result/error essentials |
| `--yes` | accept non-sensitive confirmation prompts; never grants protected capabilities |
| `--version` | print application/schema compatibility information |
| `--help` | command help |

Exit codes:

| Code | Meaning |
|---:|---|
| 0 | operation completed |
| 1 | domain or validation result failure |
| 2 | CLI usage error |
| 3 | authority stale/conflicted/unavailable |
| 4 | repository/workspace mismatch |
| 5 | storage/integrity/migration failure |
| 6 | external adapter/tool unavailable or incompatible |
| 7 | operation denied or outside W2 capability |
| 8 | cancellation/interruption |

Structured JSON output always includes `ok`, `operation`, and either `result` or `error`.

## Ordinary workflow

```text
werkstatt project add
    -> werkstatt work import
        -> werkstatt work show
            -> werkstatt workspace register
                -> edit with normal tools
                    -> werkstatt validate run
                        -> werkstatt review prepare
                            -> publish/review manually
                                -> werkstatt reconcile
```

The CLI prints the next recommended command after successful ordinary-path operations.

## Project commands

### `project add`

Registers one repository and validation command.

```text
werkstatt project add <REPOSITORY_PATH>
    --name <NAME>
    [--remote <EXPECTED_REMOTE>]
    [--validator-program <PROGRAM>]
    [--validator-arg <ARG>]...
    [--work-source none|local|github-gh]
```

Behavior:

1. canonicalize and inspect the Git repository;
2. record repository/common-directory identity;
3. observe remotes, default branch, current branch/head, and dirty state;
4. configure one authoritative work-source adapter;
5. configure validation as an argument vector, never a shell string;
6. create local SQLite project state;
7. print project ID, repository identity, validator, and limitations.

Default detection:

- when `scripts/validate.py` exists, suggest `python scripts/validate.py` but require confirmation;
- otherwise require explicit validator arguments or permit no validator with a visible limitation;
- no command is inferred from issue or model text.

Errors:

- repository not found;
- path is not a Git worktree;
- expected remote mismatch;
- project already registered;
- validator program unavailable;
- database unavailable.

### `project list`

Lists local registrations with repository, work source, freshness, and last workspace/execution status.

### `project show`

Shows:

- project and repository identity;
- authoritative work source;
- current source freshness;
- registered validator argv;
- active work item/workspace/execution;
- local data location and schema version;
- configured capabilities: W2 manual/read-only only.

### `project remove`

Deletes local operational state after confirmation.

It never deletes repository files, issues, branches, PRs, or external authority.

Options:

- `--keep-artifacts`;
- `--export-summary <PATH>`;
- `--force` only bypasses local incomplete-execution prompt, not storage integrity checks.

## Work-source commands

### `work import`

Imports or refreshes one accepted work item.

```text
werkstatt work import --file <MARKDOWN_OR_JSON>
werkstatt work import --github <ISSUE_URL>
```

#### Local file mode

- reads a Markdown work item using the Dornglut `## Work contract` table and conventional sections, or a versioned Werkstatt JSON snapshot;
- records file path identity, digest, modified time, and import time;
- treats the source file as authoritative only when project configuration selected `local` work source;
- otherwise imports it as unverified supporting material.

#### GitHub-through-`gh` mode

- requires the `gh` executable and existing user authentication;
- invokes a fixed read-only `gh issue view` argument vector;
- requests bounded JSON fields needed for normalization;
- performs no GitHub mutation;
- records repository/issue identity, update observation, raw bounded artifact, and import time;
- reports that authentication and network are owned by the user’s `gh` environment, not Werkstatt secret management;
- when `gh` is unavailable, the user can export JSON/Markdown and use file mode.

Normalization extracts:

- title and goal;
- owner;
- parent/program;
- work kind/class/stage/state;
- accepted base;
- authority links;
- included scope and non-goals;
- validation;
- stop conditions;
- exit gate;
- next transition.

Missing material fields produce an incomplete contract with exact diagnostics. W2 does not guess.

### `work refresh`

Reimports the configured source and compares revisions/digests.

Results:

- unchanged;
- advanced compatible;
- moved and contract regenerated;
- conflicted;
- unavailable;
- deleted/retired.

An active execution remains bound to its existing contract version. Source movement marks it stale and requires explicit restart or handoff.

### `work show`

Default human view:

```text
Work
  title
  owner
  stage / state / class
  source and observed revision
  freshness

Contract
  goal
  accepted base
  scope
  non-goals
  validation
  stop conditions
  exit gate

Readiness
  executable | incomplete | stale | conflicted
  missing or conflicting fields
  next valid action
```

`--format json` returns normalized contract data and provenance.

### `work packet`

Generates a derived Markdown or JSON work packet.

```text
werkstatt work packet --output <PATH> [--format markdown|json]
```

The packet contains source/revision/generation/staleness metadata and states that it is derived assistance, not authority.

## Workspace commands

### `workspace register`

Registers an existing human-owned checkout.

```text
werkstatt workspace register <PATH>
    [--name <NAME>]
    [--expect-base <REVISION>]
```

Audit output:

- repository/common-directory identity;
- workspace path (redacted in public output mode);
- branch/detached state;
- current head;
- accepted base relationship;
- dirty and untracked path summary;
- Git operation state;
- remote mismatch;
- administrative worktree state;
- trust statement: registered checkout, no sandbox claim.

Unexpected repository or base mismatch fails with correction guidance.

### `workspace status`

Refreshes and compares workspace snapshots.

Shows:

- expected versus observed head;
- changed-path counts and categories;
- stale evidence affected by head movement;
- human ownership assertion;
- active execution;
- next safe action.

### `workspace forget`

Removes local registration only after execution/handoff checks.

## Execution commands

### `execution start`

Starts a manual human execution.

```text
werkstatt execution start
    [--work <WORK_ITEM>]
    [--workspace <WORKSPACE>]
```

Preconditions:

- work contract executable and fresh;
- workspace repository matches project;
- accepted base relationship is valid;
- no unexpected Git operation or conflicting local execution;
- W2 manual policy selected.

Records:

- actor `current-human`;
- role `executor`;
- contract/base/head snapshot;
- workspace audit;
- W2 capability statement;
- start time.

W2 does not enforce an OS/process lease. It records a human ownership assertion and warns that concurrent external writers cannot be prevented.

### `execution status`

Shows contract, workspace/head movement, evidence, findings, and next action.

### `execution cancel`

Records manual cancellation after current workspace audit. It does not terminate editors or arbitrary user processes.

### `execution handoff`

Generates a handoff summary with current revisions, evidence, findings, commands not run, and next action. It may end the execution as succeeded, failed, cancelled, or conflicted according to explicit selection and domain rules.

## Validation commands

### `validate run`

Runs the configured repository-owned named validation command.

```text
werkstatt validate run [--timeout <DURATION>] [--output <PATH>]
```

Rules:

- executes the stored argv directly without shell interpretation;
- working directory is the registered workspace root;
- records head before and after;
- captures bounded stdout/stderr to a local artifact;
- displays output live unless `--quiet`;
- observes exit status, duration, cancellation, and changed paths;
- marks result `executor_observed`, not independent CI;
- if head or protected state changes unexpectedly, marks execution conflicted;
- does not retry automatically;
- does not treat unavailable command as pass.

Default limits are explicit in `project show`; user may narrow timeout per run.

### `validate record`

Records a manually supplied external validation reference without claiming verification.

```text
werkstatt validate record
    --kind exact-head-ci|accepted-main-ci|manual|runtime|security|performance|usability
    --subject <REVISION>
    --result pass|fail|warning|unavailable|inconclusive
    --reference <URI_OR_ID>
    [--notes <TEXT>]
```

The result is `unverified` until a future adapter verifies it. This supports human workflows without W5 CI integration.

### `validate show`

Shows evidence by exact revision and marks stale records.

## Review commands

### `review prepare`

Generates a review packet for the current contract and workspace head.

```text
werkstatt review prepare
    [--output <PATH>]
    [--format markdown|json]
```

Packet content:

- source and contract revisions;
- accepted base/current head;
- scope/non-goals;
- changed-file/diff summary;
- requirement-to-change mapping entered by the human where not derivable;
- validation/evidence matrix;
- findings;
- unavailable evidence;
- security/compatibility/workflow/dependency change flags;
- acceptance readiness projection;
- staleness metadata.

W2 does not analyze arbitrary code semantically or claim architectural conformance automatically.

### `finding add`

```text
werkstatt finding add
    --category <CATEGORY>
    --severity blocking|major|minor|advisory
    --summary <TEXT>
    [--requirement <KEY>]
    [--path <PATH>]
```

### `finding resolve`

Requires disposition and resolution evidence/reference. Blocking finding resolution does not retroactively make stale validation current.

### `review show`

Shows findings, claims, evidence, readiness, and stale state.

## Reconciliation commands

### `reconcile inspect`

Refreshes read-only work/repository state and reports:

- observed PR/merge/issue references supplied/imported;
- current workspace/branch/head;
- local execution state;
- evidence invalidation;
- external state changes;
- cleanup and authoritative corrections still required.

### `reconcile complete`

Records local reconciliation after the human confirms external actions.

Required confirmation includes:

- accepted or rejected external outcome;
- exact accepted revision when applicable;
- issue/source state observation or explicit unverified note;
- workspace/branch disposition;
- unresolved follow-up.

W2 changes no external source.

## Diagnostic commands

### `doctor`

Checks:

- data directory access;
- SQLite application/schema/integrity/foreign keys;
- Git availability/version;
- optional `gh` availability/auth status without exposing credentials;
- repository registration validity;
- validator program availability;
- stale nonterminal executions;
- missing artifact references;
- unsupported database version.

`doctor` is read-only unless `--repair-local` is supplied. W2 repair is limited to rebuilding derived projections and marking missing local artifacts; it cannot modify Git or external authority.

### `export`

Exports bounded human-readable or JSON project summary without secrets, raw credentials, or private paths unless explicitly requested.

## Human output conventions

- most important result first;
- source and revision near every authority statement;
- `authoritative`, `derived`, `observed`, `unverified`, `stale`, and `conflicted` labels use consistent vocabulary;
- stage, work state, execution state, validation result, and readiness are never merged into one status;
- errors state cause, blocked operation, and corrective command;
- ordinary output avoids provider payloads and internal IDs unless needed;
- `--verbose` reveals adapter and correlation details;
- paths are relative to project/workspace where safe;
- public packet output redacts home directory and user-specific paths.

## JSON output contract

Example shape:

```json
{
  "ok": false,
  "operation": "validate.run",
  "error": {
    "code": "revision.head_moved",
    "message": "Workspace head changed while validation was running.",
    "retryable": false,
    "correction": "Run `werkstatt workspace status`, reconcile the new head, then start a new validation run.",
    "context": {
      "expectedRevision": "...",
      "observedRevision": "..."
    }
  }
}
```

Stable fields are camelCase in JSON. New optional fields may be added within a schema version; breaking changes require a CLI contract version change.

## Configuration and data locations

W2 writes no repository configuration by default.

Local profile contains:

- SQLite database;
- bounded artifacts and logs;
- generated packets/review output when no explicit output path is supplied;
- local preferences.

Platform data directory is selected by an adapter. `--data-dir` provides deterministic testing and portable use.

No secret store exists in W2.

## Work-source Markdown convention

W2 recognizes, but does not require every project to use, this structure:

```markdown
## Work contract

| Field | Value |
|---|---|
| Parent | ... |
| Kind | ... |
| Lifecycle stage | ... |
| Work class | ... |
| Owner | ... |
| Accepted base | ... |
| Exit gate | ... |
| Next transition | ... |

## Goal
...

## Included scope
...

## Explicit non-goals
...

## Validation
...

## Stop conditions
...
```

Parser behavior:

- preserve raw source artifact;
- normalize known fields;
- report duplicates and ambiguous values;
- do not infer missing material decisions;
- permit repository-specific additional sections;
- version the normalization rules.

## Human pilot

W2 acceptance uses one explicitly authorized comparative task:

1. choose a bounded real Dornglut documentation change with an issue, exact base, one or few files, and canonical validator;
2. perform the task once using ordinary tools from a clean isolated checkout while recording baseline metrics;
3. reset to the same accepted base in a second isolated checkout;
4. perform the same task with Werkstatt coordinating context, workspace audit, validation evidence, review packet, and reconciliation;
5. publish at most one resulting delivery; the comparison run is disposable evidence;
6. compare results and retain limitations.

Metrics:

- time to locate authority and understand the task;
- number of distinct sources manually inspected;
- manual state/revision copying;
- missed or stale authority findings;
- time to prepare validation/review evidence;
- corrections caused by wrong base/head or scope;
- subjective clarity and friction;
- total additional Werkstatt steps;
- result correctness and review quality.

W2 passes only if the manual workflow shows net value without hiding extra process cost.
