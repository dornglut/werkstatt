# Werkstatt Testing and Validation

## Current phase

W1 is documentation and architecture only. No Rust package, CLI, database, actor runtime, or graphical application exists.

Validation must prove documentation integrity, authority truth, scope, and implementation readiness without implying runtime behavior.

## Canonical command

```text
python scripts/validate.py
```

The same command runs in read-only CI for pull requests and pushes to `main`.

## W1 baseline

Before handoff, verify:

- all required authority files exist;
- tracked text is valid UTF-8 and ends with a newline;
- no trailing whitespace or tab characters;
- repository-relative Markdown links resolve;
- no tracked file exceeds 131,072 raw bytes;
- the workflow inventory and immutable reusable-workflow pin are exact;
- lifecycle wording distinguishes accepted W0, active W1, and future W2;
- W0 proposal evidence is preserved under `docs/history/`;
- no Rust, Cargo, SQLite migration, command runtime, agent integration, GitHub write integration, or Runenwerk code entered W1;
- the W1 design covers every required output in issue #6;
- the exact reviewed feature head passes CI;
- moved heads invalidate prior validation claims.

## Architecture conformance

| Requirement | Required W1 evidence |
|---|---|
| One human/agent model | domain and role/capability design |
| External authority preserved | authority-reference and synchronization contract |
| State meanings separated | explicit state machines and transitions |
| Deny-by-default security | policy, approval, denial, and threat-control mapping |
| Safe workspace semantics | identity, audit, lease, cancellation, and recovery model |
| Independent evidence | receipt, review, validation, acceptance, and reconciliation separation |
| Provider-independent core | port contracts and protocol mappings |
| Local-only persistence | SQLite logical schema, transaction, migration, and recovery policy |
| Human-first W2 | complete CLI contract and comparative pilot |
| Implementation readiness | package, dependency, file, use-case, and acceptance-test plan |
| No W3 leakage | explicit implementation exclusions |

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

## W2 validation transition

W1 binds W2’s required implementation validation but does not change the canonical command yet.

W2 must choose one repository-owned canonical command that includes:

- formatting;
- locked compilation and tests;
- strict linting;
- documentation/authority/link/file-size checks;
- CLI contract and fixture tests;
- Git adapter tests;
- SQLite integrity, foreign-key, transaction, and schema tests;
- exact-head CI;
- accepted-main validation.

The W2 implementation issue must decide whether to extend `scripts/validate.py` or add a durable Rust validation entrypoint. It must not create multiple competing gates.

## Validation limitations

W1 cannot provide runtime, security-enforcement, performance, agent, or UI evidence because those systems do not exist. Their contracts are design evidence only and their implementation phases must prove them independently.
