# Werkstatt Testing and Validation

## Current phase

W1 and W2A are accepted. W2B adds read-only repository and work-source observation only. Guided workflow commands, validation-command execution, agents, policy enforcement, GitHub mutation, and graphical application behavior remain absent.

Validation proves repository authority and text integrity, locked formatting, executable tests, and strict Clippy for the implemented boundary. It does not imply later workflow, agent, security-enforcement, or UI behavior.

## Canonical command

```text
python scripts/validate.py
```

The same command runs in read-only CI for pull requests and pushes to `main`.

## Repository baseline

Before handoff, verify:

- all required authority and implementation files exist;
- tracked text is valid UTF-8 and ends with a newline;
- no trailing whitespace or tab characters;
- repository-relative Markdown links resolve;
- no tracked file exceeds 131,072 raw bytes;
- the workflow inventory and immutable reusable-workflow pin are exact;
- lifecycle wording distinguishes accepted W0/W1/W2A, active W2B, and blocked later deliveries;
- accepted historical evidence remains available through immutable Git history without a duplicate archive tree;
- the exact reviewed feature head passes CI;
- moved heads invalidate prior validation claims.

## W2B focused proof

Domain and source tests prove:

- one bounded observed-source value retains the exact payload and normalized observation together;
- W2A `AuthorityObservation`, immutable SHA-256 `RevisionRef`, and `SynchronizationState` own provenance and movement semantics;
- no parallel source-freshness or authority family exists;
- material work-contract values are required and never inferred;
- canonical leading Dornglut `Field: value` issue metadata and explicit work-contract tables both normalize deterministically;
- duplicate values across metadata/table forms, missing fields, malformed tables, non-UTF-8 input, oversized input, and unsupported JSON schema fail actionably;
- presentation-only backticks around scalar issue metadata do not become part of accepted revision values;
- exact source bytes produce SHA-256 revisions and source movement maps to W2A synchronization states;
- JSON duplicate keys and nesting deeper than 64 levels are rejected;
- GitHub issue sources remain explicitly mutable while each exact payload observation has an immutable revision.

Git and process tests prove:

- fixed argument vectors and no shell expansion;
- bounded timeout and output without pipe deadlock;
- unavailable executable and non-zero status remain failures;
- repository/common-directory identity, exact head, branch/detached head, remotes, ancestry, merge base, status, worktrees, and active operations are observed read-only;
- modified, staged, unstaged, untracked, renamed, conflicted, and unusual filenames are retained safely;
- staged and unstaged statistics and patch text remain separately labelled under one aggregate bound;
- external diff, text conversion, and repository-configured `core.fsmonitor` programs are disabled;
- credential-bearing remotes, forwarded provider configuration paths, and private absolute paths do not enter default/public output;
- unavailable Git and non-repository directories remain distinct.

Optional `gh` tests prove:

- exact read-only `issue view` argv and explicit JSON field selection;
- unsupported or credential-bearing URLs are rejected before execution;
- provider timeout, failure, malformed output, and output bounds do not pass;
- human configuration is forwarded portably without forwarding token variables;
- provider diagnostics redact working-directory and private configuration paths;
- no token is stored and exported Markdown/JSON remains the fallback.

## Evidence categories

Keep these distinct:

| Category | Meaning |
|---|---|
| Source inspection | files, issues, history, and primary sources inspected |
| Local validation | commands run in a checked-out workspace |
| Exact-head CI | independent validation of reviewed feature head |
| Accepted-main CI | independent validation of accepted merge revision |
| Manual verification | human observation of docs, UX, or runtime |
| Runtime proof | executed product or integration behavior |
| Security evidence | enforced policy, permission, isolation, or threat control |
| Performance evidence | reproducible measurements under stated conditions |
| Usability evidence | observed human task result and friction |

Do not report unavailable evidence as passed.

## Canonical W2 validation

W2 retains the repository-owned command:

```text
python scripts/validate.py
```

It runs, in deterministic order:

- documentation, authority, link, file-size, workflow-pin, and required-file checks;
- `cargo fmt --all --check`;
- `cargo test --workspace --locked`;
- `cargo clippy --workspace --all-targets --locked -- -D warnings`;
- bounded implementation tests required by the active delivery;
- exact-head CI;
- accepted-main validation after merge.

The validator preserves bounded head-and-tail diagnostics for failed Cargo commands so integration failures remain visible without unbounded logs.

W2 must not add an `xtask` or second canonical gate. Focused Cargo commands may support iteration but do not replace the repository baseline.

## Validation limitations

W2B provides runtime proof only for read-only observation and parsing. It provides no guided command workflow, validation execution, independent CI observation, security enforcement, agent execution, policy/lease behavior, performance claim, human-pilot result, or UI evidence.
