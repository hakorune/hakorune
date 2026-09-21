---
Status: closed__2026-09-21__WarningBaselineRefreshI10__CallableBatchTestFacadeSelected
Task: MIRBUILDER-WARNING-BASELINE-REFRESH-I10
Date: 2026-09-21
Parent: mirbuilder-warning-common-v2-session-test-facade-i0-2026-09-21.md
Implementation permission: true for one cfg(test) callable-batch facade-scope move only
NextCard: MIRBUILDER-WARNING-CALLABLE-BATCH-TEST-FACADE-I0
---

# MirBuilder warning baseline refresh I10

## Six-line brief

```text
Decision: refresh both warning surfaces after the common-V2-session test-facade
  cohort before selecting another cleanup row.
Source authority + canonical issuer: fixed quick-profile Cargo diagnostics and
  the checked-in warning classification policy.
Non-authority: historical counts, cargo-fix, blanket allow, visibility edits,
  dead-code guesses, or semantic interpretation of lint output.
Fail-fast boundary: command drift, new failure name, unclassified test-only
  reference, or a warning family outside the inventory stops selection.
Smallest next slice: compare current lib/lib-test inventories against
  1,821/561 and select at most one caller-zero import cohort.
Non-claims: no broad warning deletion, suppression, semantic refactor, or
  production route change.
```

## Acceptance

Run `cargo check --profile quick --lib -j4` and
`cargo test --profile quick --lib --no-run -j4` once each. Record lint,
file:line, owner, and production/test/compat/generated role, then select one
cohort or write `NoSafeSlice`. Keep dead-code/private-interface rows with
their owners. No code edit is permitted until the selection is recorded.

## I10 inventory and decision

The fresh commands both exited 0: lib produced 1,821 warnings and lib-test
produced 561 warnings. Two lib-only unused-import diagnostics in
`src/mir/builder/module_invocation_callable_batch.rs` form one caller-zero
test-facade cohort: the acyclic and recursive callable-module plan types are
referenced only inside the file's `#[cfg(test)] source_from_test` helper.
Production uses the source proof and capability directly and has no parent
plan import consumer. The selected next slice is one `cfg(test)` import group,
with expected lib 1,820 and lib-test 561.
