---
Status: closed__2026-09-21__WarningBaselineRefreshI59__SelectedDirectAccumTestImports
Task: MIRBUILDER-WARNING-BASELINE-REFRESH-I59
Date: 2026-09-21
Parent: mirbuilder-warning-normal-source-plan-test-import-i0-2026-09-21.md
Implementation permission: true for the selected direct-accum test-only imports only
NextCard: MIRBUILDER-WARNING-DIRECT-ACCUM-TEST-IMPORTS-I0
---

# MirBuilder warning baseline refresh I59

## Six-line brief

```text
Decision: refresh both warning surfaces after the normal source-plan test
  re-export cohort before selecting another cleanup row.
Source authority + canonical issuer: fixed quick-profile Cargo diagnostics and
  the checked-in warning classification policy.
Non-authority: historical counts, cargo-fix, blanket allow, visibility edits,
  dead-code guesses, or semantic interpretation of lint output.
Fail-fast boundary: command drift, new failure name, unclassified test-only
  reference, or a warning family outside the inventory stops selection.
Smallest next slice: compare current lib/lib-test inventories against
  1,763/561 and select at most one caller-zero import cohort.
Non-claims: no broad warning deletion, suppression, semantic refactor, or
  production route change.
```

## Acceptance

Run `cargo check --profile quick --lib -j4` and
`cargo test --profile quick --lib --no-run -j4` once each, sequentially. Record
lint, file:line, owner, and production/test/compat/generated role, then select
one cohort or write `NoSafeSlice`. Keep dead-code/private-interface rows with
 their owners. No code edit is permitted until the selection is recorded.

The previous fixed baseline is lib **1,763** and lib-test **561** after the
selected normal source-plan test re-export cohort.


## Selection evidence

The fixed gates completed sequentially with exit 0 after the normal source-plan
cohort:

| surface | warnings | evidence |
| --- | ---: | --- |
| lib | 1,763 | `/tmp/hakorune-warning-i59-lib-20260921.log` |
| lib test | 561 | `/tmp/hakorune-warning-i59-lib-test-20260921.log` |

The first two warning groups are the unused test-only imports at
`src/mir/compiler/resolved_direct_accum_cutover.rs:10,15`: the family-plan and
preflight types, plus `ResolvedModuleLoweringInputV1`. A source census shows all
three are referenced only by the `#[cfg(test)]` snapshot seam; production direct
accum lowering receives its canonical plan and does not use this resolved-input
seam. The bounded same-owner slice gates these imports together, with expected
lib **1,763 → 1,761** and lib-test unchanged at **561**. No direct-accum
production lowering, cutover, or test body is selected.
