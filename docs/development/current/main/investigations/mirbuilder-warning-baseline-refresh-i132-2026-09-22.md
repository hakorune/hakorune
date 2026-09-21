---
Status: design_stop__2026-09-22__WarningBaselineRefreshI132__AwaitingNextCohort
Task: MIRBUILDER-WARNING-BASELINE-REFRESH-I132
Date: 2026-09-22
Parent: mirbuilder-warning-generic-g0-parameters-accessor-i131-2026-09-22.md
Implementation permission: false; refresh diagnostics and select one bounded next cohort
NextCard: prefer MIR-RETIRE-FIRST-OLD-EDGE-R0 only if its caller-zero proof changes
---

# MirBuilder warning baseline refresh I132

## Six-line brief

```text
Decision: refresh the warning baseline after I131 and select exactly one
  bounded production-zero cleanup row; keep the old-edge lane parked unless
  its caller-zero or same-series switch proof changes.
Source authority + canonical issuer: quick-profile Cargo diagnostics, the
  warning classification policy, and each selected owner’s caller census.
Non-authority: warning-count guesses, blanket allow, cargo-fix, AST rescans,
  or semantic receipts without a named source owner.
Fail-fast boundary: a new warning, unclassified red, nonzero old-edge caller,
  or missing absence guard stops selection and records NoSafeSlice.
Smallest next slice: one caller-zero old edge, or one finite production-zero
  warning item after exact census.
Non-claims: no broad warning cleanup, parser semantic change, VM repair,
  fallback, or LegacyCallV0 retirement beyond one selected slice.
```

## Baseline and retirement boundary

I131 restricted `VerifiedGenericNumericFactLeaseG0::parameters` to test builds
and moved the cross-module test to existing source-parameter evidence. The
focused Generic G0 suite passed **6/6**. The new baseline is lib **1,689**
warnings and lib-test **545** warnings. The LoopBreak old-edge recheck remains
`NoSafeSlice`: registry, compatibility, parser-composite, tests, and guards
still retain live callers.

## Acceptance before selection

```text
cargo check --profile quick --lib -j4
cargo test --profile quick --lib --no-run -j4
bash tools/checks/current_state_pointer_guard.sh
```

Record both warning counts and a complete owner/caller inventory before
selecting the next fast card. Do not delete parser source-admission rows or
shared compatibility helpers without a fresh census and successor proof.
