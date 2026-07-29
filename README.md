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

W0 and W1 are accepted. W2 is active at its bounded W2A domain-and-storage foundation delivery.

It does not yet provide:

- a Codex or offline-agent adapter;
- delegated or autonomous execution;
- GitHub write integration;
- a Runenwerk graphical frontend.

W2A provides only the Rust package baseline, domain foundation, and local SQLite storage. Later W2 deliveries remain blocked; no command workflow beyond foundational diagnostics is implemented.

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

Pull requests and pushes to `main` call the same command through the pinned reusable validation workflow. Validation checks required authority files, UTF-8 text, final newlines, whitespace, repository-relative links, file-size limits, and the read-only workflow contract.

## Work authority

- [Program issue #1](https://github.com/dornglut/werkstatt/issues/1)
- [W1 issue #6](https://github.com/dornglut/werkstatt/issues/6)
- [Accepted organization ADR 0005](https://github.com/dornglut/engineering/blob/main/adrs/0005-authorize-werkstatt-pilot.md)

Git, repository source, accepted architecture, issues, pull requests, validation, and merge records retain their existing authority. Werkstatt-generated packets, matrices, summaries, and execution state are derived or local operational material and must not become parallel project authority.

## License

Werkstatt is licensed under the [MIT License](LICENSE).

## Development boundary

W2 implementation began only after accepted-main validation for W1 commit `a87d8fbc5ac1e4e31d5a3f6b601237cf8e3b8cd9` succeeded. W2A does not add agent, GitHub-write, arbitrary-command, lease-enforcement, autonomous, or Runenwerk features.
