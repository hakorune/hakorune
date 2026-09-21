---
Status: design_stop__2026-09-21__WarningBaselineRefreshI118__AwaitingNextCohort
Task: MIRBUILDER-WARNING-BASELINE-REFRESH-I118
Date: 2026-09-21
Parent: mirbuilder-warning-a-prime-wrapper-i0-2026-09-21.md
Implementation permission: false; refresh diagnostics and select one bounded next cohort
NextCard: awaiting one named warning cohort after the I118 refresh
---

# MirBuilder warning baseline refresh I118

## Six-line brief

```text
Decision: refresh the warning surface after deleting the obsolete A-prime
wrapper, then select at most one bounded row.
Source authority + canonical issuer: quick-profile Cargo diagnostics and the
warning classification policy, with the current owner census as the boundary.
Non-authority: warning-count guesses, cargo-fix, blanket allow, or guessed
source/production callers.
Fail-fast boundary: a new warning, unclassified red, nonzero caller, or route
authority mismatch stops selection and records NoSafeSlice.
Smallest next slice: compare 1,694/550, classify one warning, and choose
Delete > Stop > Promote > split > T0.
Non-claims: no broad cleanup, route switch, parser expansion, fallback repair,
or LegacyCallV0 retirement.
```

## Refresh evidence

The preceding I117 row deleted the production-zero
`insert_a_prime_i64_physical_receipt_json` wrapper and its wrapper-only
absence test. The selected metadata path still uses the value encoder; the
A-prime encoder suite passed 1/1 and the io suite passed 8/8. Sequential
validation produced **1,694 lib warnings** and **550 lib-test warnings**;
formatting and diff checks passed.

The old-edge lane remains `NoSafeSlice`: this warning baseline refresh does
not authorize a legacy route or compatibility deletion. A fresh finite
warning census must identify the next owner and caller boundary before any
implementation change.

## Acceptance

```text
cargo check --profile quick --lib -j4
cargo test --profile quick --lib --no-run -j4
bash tools/checks/current_state_pointer_guard.sh
```
