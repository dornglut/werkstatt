# Dornglut Werkstatt

Dornglut Werkstatt is an experimental human-first engineering workbench for understanding, performing, reviewing, and coordinating software work across human and automated actors.

Werkstatt is intended to support:

- manual human development without any model configured;
- interactive assistance;
- delegated local and offline agents;
- GPT web coordination through GitHub;
- later policy-controlled autonomous delivery.

All operating modes share one role-aware work model. Technical capability does not imply product, architecture, validation, approval, or acceptance authority.

## Current maturity

W0 and W1 are accepted. W2A is accepted at `bd4f12f770fa82d25657f41fd8cff5e3a299a8a1`. W2B is the active bounded delivery.

W2B adds read-only repository and work-source observation only:

- shell-free fixed-argument Git CLI inspection;
- heuristic repository fingerprint, exact head, branch/detached-head, remote, status, worktree, operation, relation, and bounded staged/unstaged diff observations;
- bounded repository-local Markdown and schema-version-1 JSON normalization;
- optional bounded read-only `gh issue view` observation using existing human authentication;
- safe source digests, freshness, limitations, path handling, and credential-redacted remote identities.

It does not yet provide:

- the guided `start -> status -> validate -> review -> reconcile` workflow;
- source, Git, GitHub, issue, pull-request, or workflow mutation;
- arbitrary validation-command execution;
- a Codex or offline-agent adapter;
- delegated or autonomous execution;
- policy or writer-lease enforcement;
- a Runenwerk graphical frontend.

A registered checkout remains a human-owned working directory, not a process, filesystem, network, or credential sandbox.

## Start here

- [Architecture](ARCHITECTURE.md)
- [Roadmap](ROADMAP.md)
- [Testing and validation](TESTING.md)
- [Agent contract](AGENTS.md)
- [Documentation index](docs/README.md)
- [W1 design](docs/w1-design.md)
- [Work domain](docs/work-domain.md)
- [Policy and security](docs/policy-and-security.md)
- [Authority and workspace](docs/authority-workspace.md)
- [Evidence and review](docs/evidence-review.md)
- [Ports and storage](docs/ports-and-storage.md)
- [W2 CLI contract](docs/w2-cli-contract.md)
- [W2 implementation specification](docs/w2-implementation-spec.md)

## Validation

Run the repository-owned read-only validator:

```text
python scripts/validate.py
```

Pull requests and pushes to `main` call the same command through the pinned reusable validation workflow. Validation checks repository authority and text rules, tracked file-size limits, workflow pinning, locked formatting and tests, and strict Clippy.

## Work authority

- [Program issue #1](https://github.com/dornglut/werkstatt/issues/1)
- [W2 umbrella issue #8](https://github.com/dornglut/werkstatt/issues/8)
- [Active W2B issue #10](https://github.com/dornglut/werkstatt/issues/10)
- [Accepted organization ADR 0005](https://github.com/dornglut/engineering/blob/main/adrs/0005-authorize-werkstatt-pilot.md)

Git, repository source, accepted architecture, issues, pull requests, validation, and merge records retain their existing authority. Werkstatt observations, packets, matrices, summaries, and execution state are derived or local operational material and must not become parallel project authority.

## License

The current Werkstatt source is available under the [GNU Affero General Public License version 3 only](LICENSE) (`AGPL-3.0-only`). A separate commercial license may be available from copyright holder(s) with sufficient rights to grant it; see [LICENSING.md](LICENSING.md) for the licensing policy and earlier-revision terms.

## Development boundary

W2B performs read-only observation. It stores no GitHub token, invokes no shell, mutates no external authority, and implements no later W2 command workflow or W3 execution policy.
