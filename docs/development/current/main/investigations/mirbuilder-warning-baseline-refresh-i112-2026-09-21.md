---
Status: design_stop__2026-09-21__WarningBaselineRefreshI112__AwaitingNextCohort
Task: MIRBUILDER-WARNING-BASELINE-REFRESH-I112
Date: 2026-09-21
Parent: mirbuilder-warning-build-gate-brand-i0-2026-09-21.md
Implementation permission: false; refresh diagnostics and select one bounded next cohort
NextCard: awaiting one named warning cohort after the I112 refresh
---

# MirBuilder warning baseline refresh I112

## Six-line brief

```text
Decision: refresh the warning surface after removing the redundant BuildGate
brand projection, then select at most one bounded row.
Source authority + canonical issuer: quick-profile Cargo diagnostics and the
warning classification policy, with the current owner census as the boundary.
Non-authority: warning-count guesses, cargo-fix, blanket allow, or guessed
source/production callers.
Fail-fast boundary: a new warning, unclassified red, nonzero caller, or route
authority mismatch stops selection and records NoSafeSlice.
Smallest next slice: compare 1,697/551, classify one warning, and choose
Delete > Stop > Promote > split > T0.
Non-claims: no broad cleanup, route switch, parser expansion, fallback repair,
or LegacyCallV0 retirement.
```

## Refresh evidence

The preceding BuildGate row removed the unused outer `brand` field and
accessor while preserving row-level parser identity. Its focused suite passed
10/10. The sequential check produced **1,697 lib warnings** and the test build
produced **551 lib-test warnings**; formatting, diff, and pointer guards pass.

The old-edge lane remains `NoSafeSlice`: no named successor or caller-zero
proof authorizes legacy-edge deletion in this refresh. The next worker must
perform a fresh finite warning census and select one bounded row before any
implementation change.

## Acceptance

```text
cargo check --profile quick --lib -j4
cargo test --profile quick --lib --no-run -j4
bash tools/checks/current_state_pointer_guard.sh
```
