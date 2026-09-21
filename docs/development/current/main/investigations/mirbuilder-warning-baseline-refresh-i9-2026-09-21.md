---
Status: closed__2026-09-21__WarningBaselineRefreshI9__CommonV2SessionTestFacadeSelected
Task: MIRBUILDER-WARNING-BASELINE-REFRESH-I9
Date: 2026-09-21
Parent: mirbuilder-warning-normal-module-transaction-test-facade-i0-2026-09-21.md
Implementation permission: true for one cfg(test) common-v2-session facade-scope move only
NextCard: MIRBUILDER-WARNING-COMMON-V2-SESSION-TEST-FACADE-I0
---

# MirBuilder warning baseline refresh I9

## Six-line brief

```text
Decision: refresh both warning surfaces after the normal-module-transaction
  test-facade cohort before selecting another cleanup row.
Source authority + canonical issuer: fixed quick-profile Cargo diagnostics and
  the checked-in warning classification policy.
Non-authority: historical counts, cargo-fix, blanket allow, visibility edits,
  dead-code guesses, or semantic interpretation of lint output.
Fail-fast boundary: command drift, new failure name, unclassified test-only
  reference, or a warning family outside the inventory stops selection.
Smallest next slice: compare current lib/lib-test inventories against
  1,826/561 and select at most one caller-zero import cohort.
Non-claims: no broad warning deletion, suppression, semantic refactor, or
  production route change.
```

## Acceptance

Run `cargo check --profile quick --lib -j4` and
`cargo test --profile quick --lib --no-run -j4` once each. Record lint,
file:line, owner, and production/test/compat/generated role, then select one
cohort or write `NoSafeSlice`. Keep dead-code/private-interface rows with
their owners. No code edit is permitted until the selection is recorded.

## I9 inventory and decision

The fresh commands both exited 0: lib produced 1,826 warnings and lib-test
produced 561 warnings. Five lib-only unused-import diagnostics in
`src/mir/builder/resolved_lowering/common_v2_session/mod.rs` form one
caller-zero test-facade cohort: the re-exports for
`LengthCallDirectEmitterRejectV1`, `InitialIndexSeedMaterializationRejectV1`,
the two condition-bool reject types, `S6CTextEqOperandIssuerRejectV1`, and
`S6CTextEqOccurrenceViewRejectV1`. Repository-wide reference search finds the
parent re-exports used only by tests; production code uses defining modules or
direct paths. The selected next slice is five individual `cfg(test)` moves,
with expected lib 1,821 and lib-test 561.
