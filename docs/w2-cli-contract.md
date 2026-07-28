# W2 Human-First CLI Contract

## Status

Proposed W1 authority for W2 command-line behavior. W2 implementation begins only after W1 is accepted and a separate W2 issue authorizes code.

## Mission

Prove that Werkstatt improves a real manual development workflow without any model, agent runtime, autonomous service, or graphical frontend.

Werkstatt coordinates context, workspace state, validation evidence, review, and reconciliation while the human continues to use their preferred editor, terminal, Git, and GitHub workflow.

## Product boundary

W2 supports:

- one local project and Git repository at a time in the ordinary path;
- a repository-local work file or optional read-only GitHub issue import through the user’s existing `gh` installation;
- one existing human-owned checkout;
- one manual execution;
- one repository-owned named validation command;
- evidence and findings;
- a generated review packet;
- manual reconciliation after an external outcome.

W2 does not:

- edit source files;
- create worktrees, branches, commits, or pull requests;
- enforce a writer lease or sandbox;
- run arbitrary shell commands;
- manage credentials, secrets, or network policy;
- execute models or agents;
- dispatch CI;
- update issues or reviews;
- merge or close work;
- implement Runenwerk UI;
- select work autonomously.

## Ordinary path

The default workflow is intentionally small:

```text
werkstatt start
    -> edit with normal tools
        -> werkstatt status
            -> werkstatt validate
                -> werkstatt review
                    -> publish/review manually
                        -> werkstatt reconcile
```

Every ordinary command prints:

- what was observed;
- source and exact revision where available;
- whether information is authoritative, derived, observed, unverified, stale, or conflicted;
- the next recommended command;
- limitations that prevent stronger claims.

Granular subcommands remain available for inspection and correction but are not required for the ordinary path.

## Invocation

```text
werkstatt [GLOBAL OPTIONS] <COMMAND> [OPTIONS]
```

Global options:

| Option | Meaning |
|---|---|
| `--data-dir <PATH>` | override local profile directory |
| `--project <ID_OR_NAME>` | select a registered project |
| `--format human|json` | output format; default `human` |
| `--no-color` | disable color |
| `--verbose` | show adapter/source detail |
| `--quiet` | show result/error essentials only |
| `--yes` | accept ordinary confirmations; never grants protected capability |
| `--version` | application, CLI-contract, and storage compatibility |

Exit codes:

| Code | Meaning |
|---:|---|
| 0 | command completed |
| 1 | domain or validation result failure |
| 2 | invalid CLI usage |
| 3 | authority stale, conflicted, or unavailable |
| 4 | repository/workspace/revision mismatch |
| 5 | storage or integrity failure |
| 6 | external executable/provider unavailable or incompatible |
| 7 | operation denied or outside W2 boundary |
| 8 | cancellation or interruption |

JSON output always contains `ok`, `operation`, and `result` or `error`.

## `start`

Guided setup and execution start.

```text
werkstatt start
    --repo <PATH>
    (--work-file <PATH> | --github <ISSUE_URL>)
    [--name <PROJECT_NAME>]
    [--expect-remote <REMOTE_URL>]
    [--validator-program <PROGRAM>]
    [--validator-arg <ARG>]...
```

If an applicable project/workspace already exists, `start` reuses it after refresh and confirmation.

Steps:

1. inspect and identify the Git repository;
2. verify expected remote when supplied;
3. import the work item;
4. derive a versioned WorkContract;
5. report missing, stale, or conflicting material fields;
6. register and audit the existing checkout;
7. verify accepted-base relationship and current head;
8. configure the named validation argv;
9. create a manual human Execution;
10. show the contract, workspace, limitations, and next action.

No shell string is accepted. Validation is stored as program plus argument vector.

When `scripts/validate.py` exists and no validator is supplied, the CLI proposes:

```text
python scripts/validate.py
```

The user must confirm it. Issue, repository, or model text cannot select a command automatically.

`start` fails safely when:

- path is not the expected repository;
- remote identity differs;
- work contract lacks a goal, owner, accepted base, scope, validation, stop condition, or exit gate required by its work class;
- source is stale or conflicted;
- workspace is in an unresolved Git operation;
- accepted base is not an ancestor where required;
- another nonterminal local execution claims the workspace;
- validator executable is unavailable.

## `status`

The primary overview command.

```text
werkstatt status [--refresh]
```

Human output sections:

```text
Work
  source, observed revision, freshness
  goal, owner, stage, state, work class
  scope, non-goals, stop conditions, exit gate

Workspace
  repository, branch, accepted base, current head
  changed/untracked paths
  Git operation state
  registered-checkout limitation

Execution
  manual execution state
  evidence and findings
  stale or conflicting records

Next
  recommended command or owner decision
```

`--refresh` rereads configured work source and repository state. It never mutates external authority.

Moved work source or workspace head:

- preserves prior observation;
- marks derived contract, diff, evidence, or review packet stale when affected;
- stops readiness claims;
- explains whether to refresh, hand off, cancel, or start a new execution.

## `validate`

Runs the configured repository-owned named validation command.

```text
werkstatt validate [--timeout <DURATION>] [--output <PATH>]
```

Rules:

- run stored argv directly without shell interpretation;
- working directory is the registered workspace root;
- record exact head before and after;
- stream bounded output and retain a bounded local artifact;
- record exit status, duration, interruption, and changed-path observation;
- classify evidence as `executor_observed`, not independent CI;
- never retry automatically;
- never treat unavailable or interrupted validation as pass;
- mark evidence stale/conflicted if head changes unexpectedly.

Result summary distinguishes:

- passed local validation;
- failed local validation;
- command unavailable;
- timed out/interrupted;
- head moved;
- output truncated;
- evidence limitations.

## `review`

Creates or displays a revision-bound review packet.

```text
werkstatt review [--output <PATH>] [--format markdown|json]
```

Packet includes:

- work source and contract revisions;
- accepted base and current head;
- scope and non-goals;
- changed-file and diff summary;
- requirement-to-change mapping supplied by the human when not derivable;
- evidence and validation matrix;
- open and resolved findings;
- unavailable evidence;
- dependency, workflow, public-contract, and protected-path change flags;
- readiness projection and reasons;
- generation time and staleness.

W2 does not claim semantic code review or architecture conformance automatically.

A blocking finding prevents ready-for-acceptance projection. A moved head makes the packet stale.

## `reconcile`

Records local reconciliation after the human performs or observes external review and acceptance.

```text
werkstatt reconcile
    --outcome accepted|rejected|superseded|cancelled
    [--accepted-revision <REVISION>]
    [--reference <URI_OR_ID>]
    [--notes <TEXT>]
```

Behavior:

1. refresh repository and configured work source where possible;
2. compare external outcome input with observed state;
3. record verification as verified or unverified;
4. close or hand off the local Execution;
5. mark derived packets/evidence historical or stale;
6. list workspace/branch/issue/roadmap cleanup still requiring human action;
7. never modify external issues, branches, PRs, or roadmaps.

Acceptance is not inferred from execution success. It requires an explicit external outcome observation or unverified manual record.

## `doctor`

Read-only diagnostics:

```text
werkstatt doctor [--repair-local]
```

Checks:

- data directory access;
- SQLite application ID, schema version, integrity, and foreign keys;
- Git availability and repository registrations;
- optional `gh` availability/auth status without exposing credentials;
- validator executable availability;
- stale nonterminal executions;
- missing artifact references;
- unsupported future schema.

`--repair-local` may rebuild derived projections, mark missing local artifacts, or close abandoned local sessions after confirmation. It cannot mutate Git or external authority.

## Granular commands

These commands expose the same use cases separately for debugging, scripting, and future GUI composition.

### Project

```text
werkstatt project add <REPO> ...
werkstatt project list
werkstatt project show
werkstatt project remove
```

`project remove` deletes local operational state only.

### Work

```text
werkstatt work import --file <PATH>
werkstatt work import --github <ISSUE_URL>
werkstatt work refresh
werkstatt work show
werkstatt work packet --output <PATH>
```

#### Local work source

Recognizes a versioned JSON snapshot or Markdown with a `## Work contract` table and conventional sections. It preserves a digest and reports duplicate/ambiguous/missing fields without guessing.

#### Read-only GitHub source

Uses fixed `gh issue view ... --json ...` arguments under the user’s existing `gh` authentication. It:

- performs no mutation;
- stores no token;
- records a bounded source artifact and observation time;
- reports mutable observation rather than claiming immutable issue revision;
- falls back to exported file input when `gh` is unavailable.

### Workspace

```text
werkstatt workspace register <PATH>
werkstatt workspace status
werkstatt workspace forget
```

Audit includes repository identity, branch/head, accepted-base relationship, dirty/untracked paths, Git operation state, remote mismatch, and worktree administrative state.

The output always states:

> Registered checkout; Werkstatt does not provide process, filesystem, network, or credential sandboxing in W2.

### Execution

```text
werkstatt execution start
werkstatt execution status
werkstatt execution cancel
werkstatt execution handoff
```

W2 records human ownership but cannot prevent external writers or terminate arbitrary editor processes.

### Evidence and findings

```text
werkstatt evidence record ...
werkstatt evidence show
werkstatt finding add ...
werkstatt finding resolve ...
```

Manual external evidence records remain `unverified` until a future adapter verifies them.

### Export

```text
werkstatt export --output <PATH> --format markdown|json
```

Exports bounded local summary without secrets or private absolute paths unless explicitly requested.

## Work-contract normalization

W2 recognizes this Dornglut-compatible structure:

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

Rules:

- preserve source identity/digest and raw bounded artifact;
- normalize known fields through versioned rules;
- allow repository-specific extra sections;
- report duplicate or ambiguous material values;
- never invent missing decisions;
- bind active Execution to an immutable contract version.

## Human and JSON output

Human output:

- result and next action first;
- source and revision near authority claims;
- consistent vocabulary for authoritative, derived, observed, unverified, stale, and conflicted;
- stage, work state, execution state, validation result, and readiness shown separately;
- default path redaction relative to project/workspace;
- concise provider details unless `--verbose`.

Error JSON example:

```json
{
  "ok": false,
  "operation": "validate",
  "error": {
    "code": "revision.head_moved",
    "message": "Workspace head changed while validation was running.",
    "retryable": false,
    "correction": "Run `werkstatt status --refresh`, reconcile the new head, then validate again.",
    "context": {
      "expectedRevision": "...",
      "observedRevision": "..."
    }
  }
}
```

Stable JSON fields use camelCase. Breaking output changes require a CLI-contract version change.

## Local data

W2 writes no repository configuration by default.

Local profile contains:

- SQLite operational database;
- bounded artifacts and command outputs;
- generated work/review packets;
- local preferences.

No secret store exists in W2.

## Comparative human pilot

W2 acceptance uses one authorized bounded Dornglut task performed twice from the same accepted base in separate disposable checkouts:

1. normal repository/editor/terminal/Git workflow;
2. the same task with Werkstatt coordinating context, audit, evidence, review, and reconciliation.

Only one result may be published. Compare:

- time to find authority and understand work;
- number of sources opened;
- manual revision/state copying;
- stale/wrong-base/scope errors;
- validation and review-packet effort;
- added Werkstatt steps;
- correctness and review quality;
- subjective clarity and friction.

W2 passes only when the product shows net manual value without hidden duplicate authority or unacceptable bookkeeping.
