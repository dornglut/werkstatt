# Werkstatt Testing and Validation

## Current phase

W0 is documentation-only. No Rust package, executable product, database, agent runtime, or graphical application exists yet.

Validation must therefore prove the truth and hygiene of the current documentation scope without implying runtime behavior that has not been implemented.

## W0 validation baseline

Before handoff, verify:

- required root authority files exist;
- all tracked text is valid UTF-8;
- text files end with a newline;
- no trailing whitespace or malformed Markdown tables are introduced;
- repository-relative Markdown links resolve;
- issue and authority references are current and non-contradictory;
- no tracked file exceeds 131,072 raw bytes;
- no Rust source, dependency manifest, database schema, model integration, source-writing workflow, or runtime implementation entered W0 scope;
- `git diff --check` passes in a checked-out repository;
- the exact pull-request head is reviewed;
- available independent CI validates that exact head once the repository has a maintained CI caller.

## Evidence categories

Keep evidence types distinct:

| Category | Meaning |
|---|---|
| Source inspection | Files, issues, history, and external sources inspected |
| Local validation | Commands run in a checked-out workspace |
| Exact-head CI | Independent validation of the reviewed feature head |
| Manual verification | Human observation of documentation, UX, or runtime behavior |
| Runtime proof | Executed application or integration behavior |
| Security evidence | Policy, sandbox, permission, or threat-control verification |
| Performance evidence | Reproducible measurements under stated conditions |

Do not report an unavailable category as passed.

## Future canonical validation

W1 must define the W2 implementation validation contract. W2 should establish one repository-owned canonical command shared by local development and read-only CI.

The future baseline is expected to include formatting, locked tests, strict linting, documentation checks, repository policy checks, and clean-state proof, but exact toolchain and command authority are not decided in W0.

## Independence

Authoring tools, local actors, generated packets, and model assessments do not replace independent validation. Validation workflows must remain read-only and must not create, rewrite, commit, push, or merge product source.
