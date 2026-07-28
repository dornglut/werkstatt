# Dornglut Werkstatt

Dornglut Werkstatt is an experimental human-first engineering workbench for understanding, performing, reviewing, and coordinating software work across human and automated actors.

The repository is in its initial investigation phase. It does not yet provide an application, command-line interface, agent runtime, autonomous delivery system, or stable public API.

Current work is owned by:

- [program issue #1](https://github.com/dornglut/werkstatt/issues/1);
- [W0 investigation issue #2](https://github.com/dornglut/werkstatt/issues/2);
- [organization reevaluation issue dornglut/engineering#23](https://github.com/dornglut/engineering/issues/23).

The first accepted phase defines the product boundary, authority model, human and agent workflows, security model, and implementation gates before Rust implementation begins.

## Validation

Canonical read-only validation:

```text
python scripts/validate.py
```

The same command is invoked through the pinned organization-owned reusable validation workflow for pull requests and pushes to `main`.

## License

Werkstatt is proposed under the [MIT License](LICENSE). The license remains reviewable until the bootstrap pull request is explicitly accepted and merged.
