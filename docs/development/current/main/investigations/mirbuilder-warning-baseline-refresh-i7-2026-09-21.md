---
Status: closed__2026-09-21__WarningBaselineRefreshI7__RawRootCompletionTestFacadeSelected
Task: MIRBUILDER-WARNING-BASELINE-REFRESH-I7
Date: 2026-09-21
Parent: mirbuilder-warning-loop-physicalizer-test-facade-i0-2026-09-21.md
Implementation permission: true for one cfg(test) raw-root facade-scope move only
NextCard: MIRBUILDER-WARNING-RAW-ROOT-TEST-FACADE-I0
---

# MirBuilder warning baseline refresh I7

## Six-line brief

```text
Decision: refresh both warning surfaces after the loop physicalizer facade
  cohort before selecting another cleanup row.
Source authority + canonical issuer: fixed quick-profile Cargo diagnostics and
  the checked-in warning classification policy.
Non-authority: historical counts, cargo-fix, blanket allow, visibility edits,
  dead-code guesses, or semantic interpretation of lint output.
Fail-fast boundary: command drift, new failure name, unclassified test-only
  reference, or a warning family outside the inventory stops selection.
Smallest next slice: compare current lib/lib-test inventories against
  1,835/561 and select at most one caller-zero import cohort.
Non-claims: no broad warning deletion, suppression, semantic refactor, or
  production route change.
```

## Acceptance

Run `cargo check --profile quick --lib -j4` and
`cargo test --profile quick --lib --no-run -j4` once each. Record lint,
file:line, owner, and production/test/compat/generated role, then select one
cohort or write `NoSafeSlice`. Keep dead-code/private-interface rows with
their owners. No code edit is permitted until the selection is recorded.

## I7 inventory and decision

The fresh commands both exited 0: lib produced 1,835 warnings and lib-test
produced 561 warnings. Four lib-only unused-import diagnostics in
`src/mir/builder/raw_root_completion.rs` form one caller-zero test-facade
cohort: `FunctionDraftKeyV1`, `ConditionFnPolicyV1`, the paired
`RawExpansionReceiptLedgerV1`/`RawExpansionReservationV1` imports, and
`PreparedRootDraftBatchV1`. Each is referenced only by the module's
`#[cfg(test)] mod tests`; production retains
`SealedRawExpansionReceiptLedgerV1` and
`RawCallableMainCompatibilityDispositionV1`. The selected next slice is the
four-import `cfg(test)` move, with expected lib 1,831 and lib-test 561.
