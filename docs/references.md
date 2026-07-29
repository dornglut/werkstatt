# External References

Reviewed for W0/W1 on 2026-07-29. External protocols and tools evolve; implementation phases must recheck the exact version they integrate.

## Agent and orchestration references

### OpenAI Codex App Server

- [Codex App Server README](https://github.com/openai/codex/blob/main/codex-rs/app-server/README.md)
- [Codex App Server protocol source](https://github.com/openai/codex/tree/main/codex-rs/app-server-protocol)

Relevant lessons:

- rich clients use a versioned bidirectional protocol with initialization, threads, turns, streamed items, diffs, approvals, and authentication;
- schemas can be generated for the installed Codex version;
- provider threads and turns remain actor-adapter state, not Werkstatt WorkItems or acceptance;
- provider approval requests must be evaluated through Werkstatt policy and cannot widen it;
- experimental fields and transports require explicit capability and version handling.

### OpenAI Symphony

- [Symphony specification](https://github.com/openai/symphony/blob/main/SPEC.md)

Relevant lessons:

- issue-driven dispatch, isolated workspaces, retries, reconciliation, and visible execution state are useful;
- tracker state and orchestrator claim state must remain separate;
- successful execution may end at human review rather than acceptance;
- Werkstatt additionally requires first-class manual work and architecture/review surfaces.

### GitHub Spec Kit

- [Spec Kit repository](https://github.com/github/spec-kit)

Relevant lessons:

- requirements, plans, tasks, and cross-artifact consistency can improve agent handoff;
- Werkstatt adopts progressive formalization but rejects universal generated specification and task authority.

## Interoperability protocols

### Model Context Protocol

- [MCP architecture](https://modelcontextprotocol.io/docs/learn/architecture)
- [MCP specification](https://modelcontextprotocol.io/specification/)

Relevant lessons:

- MCP defines host/client/server communication, capability negotiation, tools, resources, prompts, and transport;
- it is an adapter boundary for context and operations;
- it does not define Werkstatt work lifecycle, leases, validation, review, or acceptance.

### Agent2Agent Protocol

- [A2A specification](https://a2a-protocol.org/latest/specification/)

Relevant lessons:

- A2A supports agent discovery, stateful tasks, messages, artifacts, streaming updates, cancellation, and terminal states;
- an A2A Task is a remote actor-runtime correlation, not a Werkstatt WorkItem;
- A2A terminal status cannot accept project work by itself.

## Repository and workspace references

### Git worktrees

- [Git worktree documentation](https://git-scm.com/docs/git-worktree)
- [Git worktree source documentation](https://github.com/git/git/blob/master/Documentation/git-worktree.adoc)

Relevant lessons:

- linked worktrees share repository data while retaining separate working trees and per-worktree state;
- porcelain output is intended for stable scripting;
- Git worktree locking protects administrative worktree handling and is distinct from Werkstatt’s writer lease;
- worktree move, repair, prune, and remove behavior remains adapter-specific.

## SQLite references

- [Transactions](https://www.sqlite.org/lang_transaction.html)
- [Write-ahead logging](https://www.sqlite.org/wal.html)
- [Foreign keys](https://www.sqlite.org/foreignkeys.html)
- [PRAGMA reference](https://www.sqlite.org/pragma.html)
- [Database file format](https://www.sqlite.org/fileformat.html)
- [Official application-ID magic registry](https://github.com/sqlite/sqlite/blob/master/magic.txt)

Relevant lessons:

- all reads and writes occur within transactions;
- multiple readers are supported but only one write transaction exists at a time;
- WAL permits readers with one writer but requires same-host shared memory, WAL/checkpoint handling, and companion files;
- foreign keys must be enabled explicitly per connection rather than assuming defaults;
- `application_id` can identify the application database format;
- `user_version` is application-owned schema-version storage;
- local storage transactions must not wrap long-running network, process, or actor work.

The W1 design reserves provisional application ID `0x574B5354` (`WKST`). It was absent from the current official `magic.txt` registry during W1 review. W2 must recheck that registry at implementation pickup and stop for a design correction if a collision appears.

## Security and validation references

### NIST Secure Software Development Framework

- [NIST SSDF](https://csrc.nist.gov/Projects/ssdf)

Relevant lesson: security practices should integrate into the normal development lifecycle rather than form a disconnected parallel process.

### GitHub Actions concurrency

- [Control workflow concurrency](https://docs.github.com/en/actions/how-tos/write-workflows/choose-when-workflows-run/control-workflow-concurrency)

Relevant lesson: narrow scoped serialization is useful; it does not justify global project locks or authority.

## Dornglut primary evidence

- [Accepted ADR 0003](https://github.com/dornglut/engineering/blob/main/adrs/0003-retire-provider-neutral-repository-automation.md)
- [Accepted ADR 0004](https://github.com/dornglut/engineering/blob/main/adrs/0004-organization-work-and-repository-standardization.md)
- [Accepted ADR 0005](https://github.com/dornglut/engineering/blob/main/adrs/0005-authorize-werkstatt-pilot.md)
- [Repository standard](https://github.com/dornglut/engineering/blob/main/standards/repositories.md)
- [Validation standard](https://github.com/dornglut/engineering/blob/main/standards/validation.md)

These sources own Dornglut-specific authority. External references inform design but do not override accepted Dornglut decisions.
