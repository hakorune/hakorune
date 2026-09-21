---
Status: closed__2026-09-21__WarningNestedPredicateTestImport__Execution
Task: MIRBUILDER-WARNING-NESTED-PREDICATE-TEST-IMPORT-I0
Date: 2026-09-21
Parent: mirbuilder-warning-baseline-refresh-i60-2026-09-21.md
Implementation permission: true for the selected `ResolvedModuleLoweringInputV1` import only
NextCard: MIRBUILDER-WARNING-BASELINE-REFRESH-I61
---

# Warning cleanup: nested-predicate test import

## Six-line brief

```text
Decision: gate the unused ResolvedModuleLoweringInputV1 import for the
  nested-predicate late-failure fixture.
Source authority + canonical issuer: lowering_input owns the type; production
  nested-predicate lowering consumes the canonical plan instead.
Non-authority: cargo-fix, wildcard imports, visibility changes, production
  nested-predicate cutover, or warning guesses.
Fail-fast boundary: any production reference, compile error, changed test
  warning, or warning-count mismatch rejects the slice.
Smallest next slice: add cfg(test) to the single import item, then run the
  fixed gates sequentially.
Non-claims: no nested-predicate production lowering, cutover, admission, or
  test-body change.
```

## Preconditions and acceptance

I60 selected the unused import at
`src/mir/compiler/resolved_nested_predicate_cutover.rs:12`.
`ResolvedModuleLoweringInputV1` is referenced only by the `#[cfg(test)]`
late-failure seam. Production nested-predicate lowering uses the canonical
nested-predicate plan and prepared external-commit product.

Split the import so only `ResolvedModuleLoweringInputV1` receives `#[cfg(test)]`.
Do not edit nested-predicate lowering, cutover, admission, or test bodies.

Expected result: one warning diagnostic is removed, so lib warnings
**1,761 → 1,760** and lib-test warnings remain **561**. Both fixed gates must
exit 0:

```bash
cargo check --profile quick --lib -j4
cargo test --profile quick --lib --no-run -j4
```

## Closeout evidence

`ResolvedModuleLoweringInputV1` now receives `#[cfg(test)]`; nested-predicate
production lowering, cutover, admission, and test body were unchanged. The fixed
gates completed sequentially with exit 0:

| surface | warnings | evidence |
| --- | ---: | --- |
| lib | 1,760 | `/tmp/hakorune-warning-i0-nested-predicate-test-import-lib-20260921.log` |
| lib test | 561 | `/tmp/hakorune-warning-i0-nested-predicate-test-import-lib-test-20260921.log` |

The expected one-diagnostic lib reduction was observed; the lib-test baseline
was unchanged.
