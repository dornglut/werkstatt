# Dornglut Werkstatt

Dornglut Werkstatt is an experimental human-first engineering workbench for understanding, performing, reviewing, and coordinating software work across human and automated actors.

Werkstatt is intended to support:

- manual human development without any model configured;
- interactive assistance;
- delegated local and offline agents;
- GPT web coordination through GitHub;
- later policy-controlled autonomous delivery.

All operating modes share one role-aware work model. The product must preserve explicit authority, capabilities, approvals, isolated workspaces, independent validation, review, and acceptance.

## Current maturity

The repository is in W0 product and architecture investigation.

It does not yet provide:

- a Rust package or stable public API;
- a command-line application;
- a database;
- a Codex or offline-agent adapter;
- autonomous execution;
- GitHub lifecycle automation;
- a Runenwerk graphical frontend.

W0 defines the product boundary, authority model, human and actor workflows, security model, alternatives, success criteria, and implementation gates before product implementation begins.

## Start here

- [Architecture](ARCHITECTURE.md)
- [Roadmap](ROADMAP.md)
- [Testing and validation](TESTING.md)
- [Agent contract](AGENTS.md)
- [Documentation index](docs/README.md)
- [W0 investigation](docs/w0-investigation.md)
- [Product and architecture design](docs/product-architecture.md)
- [Human and actor workflows](docs/actor-workflows.md)
- [Security and trust model](docs/security-model.md)
- [W1 readiness](docs/w1-readiness.md)

## Work authority

- [Program issue #1](https://github.com/dornglut/werkstatt/issues/1)
- [W0 investigation issue #2](https://github.com/dornglut/werkstatt/issues/2)
- [Organization reevaluation dornglut/engineering#23](https://github.com/dornglut/engineering/issues/23)

Git, repository source, accepted architecture, issues, pull requests, validation, and merge records retain their existing authority. Werkstatt-generated packets, matrices, summaries, and execution state are derived or local operational material and must not become parallel project authority.

## Development status

Only documentation and issue work are authorized in W0. Rust implementation begins no earlier than W2 after W0 and W1 are accepted and the organization decision permits the bounded pilot.
