---
Status: closed__2026-09-21__WarningBaselineRefreshI8__NormalModuleTransactionTestFacadeSelected
Task: MIRBUILDER-WARNING-BASELINE-REFRESH-I8
Date: 2026-09-21
Parent: mirbuilder-warning-raw-root-test-facade-i0-2026-09-21.md
Implementation permission: true for one cfg(test) normal-module-transaction facade-scope move only
NextCard: MIRBUILDER-WARNING-NORMAL-MODULE-TRANSACTION-TEST-FACADE-I0
---

# MirBuilder warning baseline refresh I8

## Six-line brief

```text
Decision: refresh both warning surfaces after the raw-root test-facade cohort
  before selecting another cleanup row.
Source authority + canonical issuer: fixed quick-profile Cargo diagnostics and
  the checked-in warning classification policy.
Non-authority: historical counts, cargo-fix, blanket allow, visibility edits,
  dead-code guesses, or semantic interpretation of lint output.
Fail-fast boundary: command drift, new failure name, unclassified test-only
  reference, or a warning family outside the inventory stops selection.
Smallest next slice: compare current lib/lib-test inventories against
  1,831/561 and select at most one caller-zero import cohort.
Non-claims: no broad warning deletion, suppression, semantic refactor, or
  production route change.
```

## Acceptance

Run `cargo check --profile quick --lib -j4` and
`cargo test --profile quick --lib --no-run -j4` once each. Record lint,
file:line, owner, and production/test/compat/generated role, then select one
cohort or write `NoSafeSlice`. Keep dead-code/private-interface rows with
their owners. No code edit is permitted until the selection is recorded.

## I8 inventory and decision

The fresh commands both exited 0: lib produced 1,831 warnings and lib-test
produced 561 warnings. Five lib-only unused-import diagnostics in
`src/mir/builder/normal_module_transaction/mod.rs` form one caller-zero
test-facade cohort: `NormalCallableBatchErrorV1`,
`NormalCallableCommitErrorV1`, `NormalMainModuleTransactionStageV1`,
`RejectedNormalModuleTransactionSchemaV1`, and `NormalModuleDraftRoleV1`.
Repository-wide reference search finds their remaining consumers only in the
module's production defining submodules and its `#[cfg(test)]` tests; the
facade re-exports themselves are test-only. The selected next slice is five
individual `cfg(test)` moves, with expected lib 1,826 and lib-test 561.
