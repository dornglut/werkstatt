# W2 Human-First CLI Contract

## Status

Proposed W1 authority for W2 command-line behavior. W2 implementation begins only after W1 is accepted and a separate W2 issue authorizes code.

## Mission

Prove that Werkstatt improves a real manual development workflow without any model, agent runtime, autonomous service, or graphical frontend.

Werkstatt coordinates context, workspace state, validation evidence, review, and reconciliation while the human continues to use their preferred editor, terminal, Git, and GitHub workflow.

## W2 boundary

W2 supports:

- one local project and Git repository in the ordinary path;
- one repository-local work file or optional read-only GitHub issue import through the user’s existing `gh` installation;
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

```text
werkstatt start
    -> edit with normal tools
        -> werkstatt status
            -> werkstatt validate
                -> werkstatt review
                    -> publish and review manually
                        -> werkstatt reconcile
```

Every ordinary command reports:

- what was observed;
- source and exact revision where available;
- whether information is authoritative, derived, observed, unverified, stale, or conflicted;
- the next recommended command;
- limitations that prevent stronger claims.

Granular subcommands expose the same use cases for inspection and scripting but are not required in the ordinary path.

## Invocation

```text
werkstatt [GLOBAL OPTIONS] <COMMAND> [OPTIONS]
```

Global options:

| Option | Meaning |
|---|---|
| `--data-dir <PATH>` | override the local profile directory |
| `--project <ID_OR_NAME>` | select a registered project |
| `--format human|json` | output format; default `human` |
| `--no-color` | disable color |
| `--verbose` | show adapter and source detail |
| `--quiet` | show result or error essentials only |
| `--yes` | accept ordinary confirmations; never grants protected capability |
| `--version` | application, CLI-contract, and storage compatibility |

Exit codes:

| Code | Meaning |
|---:|---|
| 0 | command completed |
| 1 | domain or validation-result failure |
| 2 | invalid CLI usage |
| 3 | authority stale, conflicted, or unavailable |
| 4 | repository, workspace, or revision mismatch |
| 5 | storage or integrity failure |
| 6 | external executable or provider unavailable/incompatible |
| 7 | operation denied or outside W2 boundary |
| 8 | cancellation or interruption |

## JSON contract

Every JSON response contains:

```json
{
  "schemaVersion": 1,
  "ok": true,
  "operation": "status",
  "result": {}
}
```

Failure shape:

```json
{
  "schemaVersion": 1,
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

Rules:

- stable fields use camelCase;
- unknown additive fields must be ignored by compatible consumers;
- breaking changes require a new `schemaVersion` and documented compatibility behavior;
- human prose never appears where a stable enum/code is required;
- default output contains no secret, token, raw environment, or unredacted home path.

## `start`

Guided setup and execution start:

```text
werkstatt start
    --repo <PATH>
    (--work-file <PATH> | --github <ISSUE_URL>)
    [--name <PROJECT_NAME>]
    [--expect-remote <REMOTE_URL>]
    [--validator-program <PROGRAM>]
    [--validator-arg <ARG>]...
```

If a compatible project/workspace already exists, `start` refreshes and reuses it after confirmation.

Steps:

1. identify the Git repository and remote;
2. import the work source;
3. derive an immutable WorkContract;
4. report missing, stale, or conflicting fields;
5. register and audit the existing checkout;
6. verify accepted-base relationship and current head;
7. configure the named validation argv;
8. create a manual human Execution;
9. show the contract, workspace, limitations, and next action.

No shell string is accepted. Validation is stored as program plus argument vector.

When `scripts/validate.py` exists and no validator is supplied, propose:

```text
python scripts/validate.py
```

The user confirms it. Issue, repository, tool, or model text cannot select a command automatically.

`start` fails safely for wrong repository/remote/base, incomplete contract, stale/conflicting source, unresolved Git operation, conflicting local execution, or unavailable validator.

## `status`

```text
werkstatt status [--refresh]
```

Human output:

```text
Work
  source, observed revision, freshness
  goal, owner, stage, state, work class
  scope, non-goals, stop conditions, exit gate

Workspace
  repository, branch, accepted base, current head
  changed and untracked paths
  Git operation state
  registered-checkout limitation

Execution
  manual execution state
  evidence and findings
  stale or conflicting records

Next
  recommended command or owner decision
```

`--refresh` rereads configured work and repository sources without mutation.

Moved source or head preserves prior observations, marks affected contract/diff/evidence/review projections stale, stops readiness claims, and explains the safe correction.

## `validate`

```text
werkstatt validate [--timeout <DURATION>] [--output <PATH>]
```

Rules:

- invoke stored argv directly without shell interpretation;
- use the registered workspace root;
- record exact head before and after;
- stream bounded output and retain a bounded artifact;
- record exit status, duration, interruption, and changed paths;
- classify result as `executorObserved`, not independent CI;
- do not retry automatically;
- do not treat unavailable, interrupted, or timed-out validation as pass;
- mark evidence stale/conflicted after unexpected head movement.

## `review`

```text
werkstatt review [--output <PATH>] [--format markdown|json]
```

The revision-bound packet contains:

- source and contract revisions;
- accepted base and current head;
- scope and non-goals;
- changed-file and diff summary;
- human-supplied requirement mapping when not derivable;
- evidence and validation matrix;
- findings;
- unavailable evidence;
- dependency, workflow, public-contract, and protected-path flags;
- readiness projection and reasons;
- generation time and staleness.

W2 does not claim semantic code review or architecture conformance automatically. A blocking finding prevents readiness; a moved head makes the packet stale.

## `reconcile`

```text
werkstatt reconcile
    --outcome accepted|rejected|superseded|cancelled
    [--accepted-revision <REVISION>]
    [--reference <URI_OR_ID>]
    [--notes <TEXT>]
```

Behavior:

1. refresh available sources;
2. compare supplied external outcome with observations;
3. record verification as verified or unverified;
4. close or hand off the local Execution;
5. mark derived packets/evidence historical or stale;
6. list workspace, branch, issue, and roadmap cleanup still requiring a human;
7. mutate no external source.

Acceptance is never inferred from execution success.

## `doctor`

```text
werkstatt doctor [--repair-local]
```

Read-only checks:

- data-directory access;
- SQLite application ID, schema version, integrity, and foreign keys;
- Git and repository registrations;
- optional `gh` availability/auth status without exposing credentials;
- validator availability;
- stale nonterminal executions;
- missing artifact references;
- unsupported future schema.

`--repair-local` may rebuild derived projections or close abandoned local sessions after confirmation. It cannot modify Git or external authority.

## Granular commands

```text
werkstatt project add|list|show|remove
werkstatt work import|refresh|show|packet
werkstatt workspace register|status|forget
werkstatt execution start|status|cancel|handoff
werkstatt evidence record|show
werkstatt finding add|resolve
werkstatt export
```

Rules:

- `project remove` deletes local state only;
- local work import accepts bounded Markdown or versioned JSON and never invents missing decisions;
- optional GitHub import uses fixed read-only `gh issue view ... --json ...` arguments under existing user authentication;
- W2 stores no GitHub token and performs no mutation;
- workspace audit covers repository identity, branch/head, base relation, dirty/untracked paths, Git operation, remote mismatch, and worktree administrative state;
- output always states that a registered checkout is not a process, filesystem, network, or credential sandbox;
- W2 records human ownership but cannot prevent external writers or terminate arbitrary editor processes;
- manually recorded external evidence remains unverified until a future adapter verifies it.

## Work-contract normalization

W2 recognizes versioned JSON or Dornglut-compatible Markdown containing a `## Work contract` table plus goal, scope, non-goals, validation, stop conditions, and exit gate.

Normalization:

- preserves source identity, digest, and bounded raw artifact;
- uses versioned rules;
- permits repository-specific extra sections;
- reports duplicate, ambiguous, or missing material values;
- never invents decisions;
- binds an active Execution to one immutable contract version.

## Local data

W2 writes no repository configuration by default.

The local profile contains:

- SQLite operational database;
- bounded artifacts and command output;
- generated work/review packets;
- local preferences.

No secret store exists in W2.

## Human pilot reference

The exact comparative method is owned by [W2 implementation specification](w2-implementation-spec.md). It uses counterbalanced task order so learning effects are visible rather than accidentally credited to Werkstatt.
