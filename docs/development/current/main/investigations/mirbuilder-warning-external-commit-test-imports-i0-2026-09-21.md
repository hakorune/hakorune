---
Status: closed__2026-09-21__WarningExternalCommitTestImports__Execution
Task: MIRBUILDER-WARNING-EXTERNAL-COMMIT-TEST-IMPORTS-I0
Date: 2026-09-21
Parent: mirbuilder-warning-baseline-refresh-i54-2026-09-21.md
Implementation permission: true for the two test-only imports only
NextCard: MIRBUILDER-WARNING-BASELINE-REFRESH-I55
---

# Warning cleanup: external commit test imports

## Six-line brief

```text
Decision: gate two unused final-verification seal imports used only by the
  external-commit fixture.
Source authority + canonical issuer: module_postprocess owns verification
  seals; external_commit.rs only forwards them to its test fixture.
Non-authority: cargo-fix, wildcard imports, visibility changes, commit logic,
  publication behavior, or warning guesses.
Fail-fast boundary: any production consumer, compile error, changed test
  warning, or warning-count mismatch rejects the slice.
Smallest next slice: add cfg(test) to external_commit.rs:7 for the two seal
  types, then run fixed gates sequentially.
Non-claims: no external commit preparation, publication, or verification change.
```

## Preconditions and acceptance

I54 selected the unused import group at
`src/mir/compiler/external_commit.rs:7`.
`CanonicalFinalVerificationSealInnerV1` and
`CanonicalFinalVerificationSealV1` are referenced only by the module's
`#[cfg(test)] mod tests`; production code consumes the already-sealed
`ModuleVerificationEvidenceV1` path.

Split the import so only those two types receive `#[cfg(test)]`. Do not edit
external commit logic, module postprocess, tests, or publication consumers.

Expected result: one warning diagnostic is removed, so lib warnings
**1,769 → 1,768** and lib-test warnings remain **561**. Both fixed gates must
exit 0:

```bash
cargo check --profile quick --lib -j4
cargo test --profile quick --lib --no-run -j4
```


## Closeout evidence

The two final-verification seal imports received `#[cfg(test)]`; external commit
preparation, module postprocess, publication, tests, and verification behavior
were unchanged. The fixed gates completed sequentially with exit 0:

| surface | warnings | evidence |
| --- | ---: | --- |
| lib | 1,768 | `/tmp/hakorune-warning-i0-external-commit-test-imports-lib-20260921.log` |
| lib test | 561 | `/tmp/hakorune-warning-i0-external-commit-test-imports-lib-test-20260921.log` |

The expected one-diagnostic lib reduction was observed; the lib-test baseline
was unchanged.
