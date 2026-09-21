---
Status: closed__2026-09-21__WarningBaselineRefreshI60__SelectedNestedPredicateTestImport
Task: MIRBUILDER-WARNING-BASELINE-REFRESH-I60
Date: 2026-09-21
Parent: mirbuilder-warning-direct-accum-test-imports-i0-2026-09-21.md
Implementation permission: true for the selected test-only nested-predicate import only
NextCard: MIRBUILDER-WARNING-NESTED-PREDICATE-TEST-IMPORT-I0
---

# MirBuilder warning baseline refresh I60

## Six-line brief

```text
Decision: refresh both warning surfaces after the DirectAccum test-import
  cohort before selecting another cleanup row.
Source authority + canonical issuer: fixed quick-profile Cargo diagnostics and
  the checked-in warning classification policy.
Non-authority: historical counts, cargo-fix, blanket allow, visibility edits,
  dead-code guesses, or semantic interpretation of lint output.
Fail-fast boundary: command drift, new failure name, unclassified test-only
  reference, or a warning family outside the inventory stops selection.
Smallest next slice: compare current lib/lib-test inventories against
  1,761/561 and select at most one caller-zero import cohort.
Non-claims: no broad warning deletion, suppression, semantic refactor, or
  production route change.
```

## Acceptance

Run `cargo check --profile quick --lib -j4` and
`cargo test --profile quick --lib --no-run -j4` once each, sequentially. Record
lint, file:line, owner, and production/test/compat/generated role, then select
one cohort or write `NoSafeSlice`. Keep dead-code/private-interface rows with
 their owners. No code edit is permitted until the selection is recorded.

The previous fixed baseline is lib **1,761** and lib-test **561** after the
selected DirectAccum test-import cohort.


## Selection evidence

The fixed gates completed sequentially with exit 0 after the DirectAccum cohort:

| surface | warnings | evidence |
| --- | ---: | --- |
| lib | 1,761 | `/tmp/hakorune-warning-i60-lib-20260921.log` |
| lib test | 561 | `/tmp/hakorune-warning-i60-lib-test-20260921.log` |

The first warning group is the unused `ResolvedModuleLoweringInputV1` import at
`src/mir/compiler/resolved_nested_predicate_cutover.rs:12`. A source census shows
it is referenced only by the `#[cfg(test)]` late-failure seam; production nested
predicate lowering receives its canonical plan and prepared product. The bounded
slice is therefore a test-only import gate, with expected lib **1,761 → 1,760**
and lib-test unchanged at **561**. No nested-predicate production lowering,
cutover, or test body is selected.
