---
Status: design_stop__2026-09-21__WarningBaselineRefreshI116__AwaitingNextCohort
Task: MIRBUILDER-WARNING-BASELINE-REFRESH-I116
Date: 2026-09-21
Parent: mirbuilder-warning-lineage-accessors-i0-2026-09-21.md
Implementation permission: false; refresh diagnostics and select one bounded next cohort
NextCard: awaiting one named warning cohort after the I116 refresh
---

# MirBuilder warning baseline refresh I116

## Six-line brief

```text
Decision: refresh the warning surface after deleting the merged-lineage
accessor group, then select at most one bounded row.
Source authority + canonical issuer: quick-profile Cargo diagnostics and the
warning classification policy, with the current owner census as the boundary.
Non-authority: warning-count guesses, cargo-fix, blanket allow, or guessed
source/production callers.
Fail-fast boundary: a new warning, unclassified red, nonzero caller, or route
authority mismatch stops selection and records NoSafeSlice.
Smallest next slice: compare 1,695/550, classify one warning, and choose
Delete > Stop > Promote > split > T0.
Non-claims: no broad cleanup, route switch, parser expansion, fallback repair,
or LegacyCallV0 retirement.
```

## Refresh evidence

The preceding I115 row deleted the unused `MergedSourceLineageV1::root`,
`segments`, and `edges` accessors. The existing `pub(crate)` fields remain the
lineage data authority, and its focused strip suite passed 5/5; the merged
parser route suite passed 2/2. Sequential validation produced **1,695 lib
warnings** and **550 lib-test warnings**; formatting, diff, and pointer guards
passed.

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
