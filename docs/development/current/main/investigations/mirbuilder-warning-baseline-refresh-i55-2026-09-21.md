---
Status: closed__2026-09-21__WarningBaselineRefreshI55__SelectedDynamicPhysicalInputTestImport
Task: MIRBUILDER-WARNING-BASELINE-REFRESH-I55
Date: 2026-09-21
Parent: mirbuilder-warning-external-commit-test-imports-i0-2026-09-21.md
Implementation permission: true for the selected test-only import cohort only
NextCard: MIRBUILDER-WARNING-DYNAMIC-PHYSICAL-INPUT-TEST-IMPORT-I0
---

# MirBuilder warning baseline refresh I55

## Six-line brief

```text
Decision: refresh both warning surfaces after the I54 external-commit test
  import cohort before selecting another cleanup row.
Source authority + canonical issuer: fixed quick-profile Cargo diagnostics and
  the checked-in warning classification policy.
Non-authority: historical counts, cargo-fix, blanket allow, visibility edits,
  dead-code guesses, or semantic interpretation of lint output.
Fail-fast boundary: command drift, new failure name, unclassified test-only
  reference, or a warning family outside the inventory stops selection.
Smallest next slice: compare current lib/lib-test inventories against
  1,768/561 and select at most one caller-zero import cohort.
Non-claims: no broad warning deletion, suppression, semantic refactor, or
  production route change.
```

## Acceptance

Run `cargo check --profile quick --lib -j4` and
`cargo test --profile quick --lib --no-run -j4` once each, sequentially. Record
lint, file:line, owner, and production/test/compat/generated role, then select
one cohort or write `NoSafeSlice`. Keep dead-code/private-interface rows with
their owners. No code edit is permitted until the selection is recorded.

The previous fixed baseline is lib **1,768** and lib-test **561** after I54.


## Selection evidence

The fixed gates completed sequentially with exit 0 after I54:

| surface | warnings | evidence |
| --- | ---: | --- |
| lib | 1,768 | `/tmp/hakorune-warning-i55-lib-20260921.log` |
| lib test | 561 | `/tmp/hakorune-warning-i55-lib-test-20260921.log` |

The first warning group is the unused `LoopJoinNextItemV1` import at
`src/mir/compiler/dynamic_full_body_recipe/coseal/semantic_program/physical_input.rs:18`.
A source census shows the type is referenced only by the module's
`#[cfg(test)] mod tests` at line 459; production physical-input code uses the
other loop-contract types from the same import group. The selected bounded slice
is therefore a test-only import gate, with expected lib **1,768 → 1,767** and
lib-test unchanged at **561**. No production route, Recipe, physical consumer,
or test body is selected.
