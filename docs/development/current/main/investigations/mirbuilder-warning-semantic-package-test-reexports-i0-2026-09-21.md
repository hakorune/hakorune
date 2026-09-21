---
Status: closed__2026-09-21__WarningSemanticPackageTestReexports__Execution
Task: MIRBUILDER-WARNING-SEMANTIC-PACKAGE-TEST-REEXPORTS-I0
Date: 2026-09-21
Parent: mirbuilder-warning-baseline-refresh-i62-2026-09-21.md
Implementation permission: true for the selected two test-only semantic-package re-exports only
NextCard: MIRBUILDER-WARNING-BASELINE-REFRESH-I63
---

# Warning cleanup: semantic-package test re-exports

## Six-line brief

```text
Decision: gate two unused semantic-package parent re-exports used only by
  package fixtures.
Source authority + canonical issuer: issuer and model own the types; production
  code uses those owning modules directly.
Non-authority: cargo-fix, wildcard imports, visibility changes, package issuer,
  model semantics, or warning guesses.
Fail-fast boundary: any production reference, compile error, changed test
  warning, or warning-count mismatch rejects the slice.
Smallest next slice: add cfg(test) to the two selected re-export items, then run
  the fixed gates sequentially.
Non-claims: no semantic-package issuance, model, resolver, or test-body change.
```

## Preconditions and acceptance

I62 selected the two unused re-exports at
`src/mir/normal_callable_semantic_package/mod.rs:103,113`.
`NormalCallableSemanticPackageIssueV1` and
`NormalCallableDynamicProjectionRefV1` are referenced through this parent only by
`#[cfg(test)]` fixtures. Production issuer and model code use their owning
submodules directly.

Split the re-exports so only these two items receive `#[cfg(test)]`; leave the
package issuer function and other production model exports unconditional. Do not
edit package issuance, model semantics, resolver behavior, or test bodies.

Expected result: two warning diagnostics are removed, so lib warnings
**1,759 → 1,757** and lib-test warnings remain **561**. Both fixed gates must
exit 0:

```bash
cargo check --profile quick --lib -j4
cargo test --profile quick --lib --no-run -j4
```

## Closeout evidence

`NormalCallableSemanticPackageIssueV1` and
`NormalCallableDynamicProjectionRefV1` now receive `#[cfg(test)]` on the parent
re-exports; semantic-package issuance, model semantics, resolver behavior, and
test bodies were unchanged. The fixed gates completed sequentially with exit 0:

| surface | warnings | evidence |
| --- | ---: | --- |
| lib | 1,757 | `/tmp/hakorune-warning-i0-semantic-package-test-reexports-lib-20260921.log` |
| lib test | 561 | `/tmp/hakorune-warning-i0-semantic-package-test-reexports-lib-test-20260921.log` |

The expected two-diagnostic lib reduction was observed; the lib-test baseline
was unchanged.
