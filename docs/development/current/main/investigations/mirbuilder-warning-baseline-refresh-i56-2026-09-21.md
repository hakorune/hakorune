---
Status: closed__2026-09-21__WarningBaselineRefreshI56__SelectedGenericG0AdmissionTestImport
Task: MIRBUILDER-WARNING-BASELINE-REFRESH-I56
Date: 2026-09-21
Parent: mirbuilder-warning-dynamic-physical-input-test-import-i0-2026-09-21.md
Implementation permission: true for the selected test-only re-export only
NextCard: MIRBUILDER-WARNING-GENERIC-G0-ADMISSION-TEST-IMPORT-I0
---

# MirBuilder warning baseline refresh I56

## Six-line brief

```text
Decision: refresh both warning surfaces after the I0 dynamic physical-input
  test-import cohort before selecting another cleanup row.
Source authority + canonical issuer: fixed quick-profile Cargo diagnostics and
  the checked-in warning classification policy.
Non-authority: historical counts, cargo-fix, blanket allow, visibility edits,
  dead-code guesses, or semantic interpretation of lint output.
Fail-fast boundary: command drift, new failure name, unclassified test-only
  reference, or a warning family outside the inventory stops selection.
Smallest next slice: compare current lib/lib-test inventories against
  1,767/561 and select at most one caller-zero import cohort.
Non-claims: no broad warning deletion, suppression, semantic refactor, or
  production route change.
```

## Acceptance

Run `cargo check --profile quick --lib -j4` and
`cargo test --profile quick --lib --no-run -j4` once each, sequentially. Record
lint, file:line, owner, and production/test/compat/generated role, then select
one cohort or write `NoSafeSlice`. Keep dead-code/private-interface rows with
 their owners. No code edit is permitted until the selection is recorded.

The previous fixed baseline is lib **1,767** and lib-test **561** after the
selected dynamic physical-input test import cohort.


## Selection evidence

The fixed gates completed sequentially with exit 0 after the dynamic physical-input
cohort:

| surface | warnings | evidence |
| --- | ---: | --- |
| lib | 1,767 | `/tmp/hakorune-warning-i56-lib-20260921.log` |
| lib test | 561 | `/tmp/hakorune-warning-i56-lib-test-20260921.log` |

The first warning group is the unused `issue_generic_g0_physical_emitter_admission_v1`
re-export at `src/mir/compiler/generic_g0_physical_operation_cohort.rs:14`. A
source census shows production `lowering_input.rs` consumes the reject type, the
prepared admission, and the source-parent issuer; only the selected admission
function is called from `#[cfg(test)]` fixtures in `emitter_admission_tests.rs`
and `generic_g0_physical_emitter_session.rs`. The bounded slice is therefore a
test-only re-export gate, with expected lib **1,767 → 1,766** and lib-test
unchanged at **561**. No admission logic, lowering input, or test body is selected.
