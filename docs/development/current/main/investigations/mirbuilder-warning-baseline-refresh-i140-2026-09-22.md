---
Status: design_stop__2026-09-22__WarningBaselineRefreshI140__NoSafeSlice
Task: MIRBUILDER-WARNING-BASELINE-REFRESH-I140
Date: 2026-09-22
Parent: mirbuilder-warning-loop-target-policy-retire-i139-2026-09-22.md
Implementation permission: false; refresh diagnostics and select one finite next cohort
NextCard: owner-decision__I140_warning_census
---

# MirBuilder warning baseline refresh I140

## Six-line brief

```text
Decision: refresh the warning baseline after I139 and select at most one
  finite production-zero cleanup candidate from the measured diagnostics.
Source authority + canonical issuer: quick-profile Cargo diagnostics,
  warning classification policy, and each selected owner's source references.
Non-authority: warning-count guesses, blanket allow, cargo-fix, AST rescans,
  inferred route identity, or new semantic receipts.
Fail-fast boundary: unclassified red, a non-test caller, changed owner/route
  inventory, or a missing focused guard returns to design_stop.
Smallest next slice: one declaration-only production-zero item or one
  caller-zero old edge with an explicit successor proof.
Non-claims: no broad warning sweep, parser semantic expansion, VM repair,
  fallback, backend promotion, or LegacyCallV0 retirement.
```

## Selection gate

The I139 closeout measured lib **1,677 warnings** and lib-test **544
warnings**. I140 must rerun the quick baseline and classify each proposed
warning as `current-change failure`, `known baseline debt`, or
`informational census`. A candidate is selectable only after recording its
declaration, all callers, owner authority, focused guard, and exact delete
set. The old five-name loop policy is retired and is not a candidate again.

Boundary: quick-profile `src/lib.rs` diagnostics -> one selected owner and
its focused guard; includes Rust production and test references for the
candidate, excludes semantic expansion and unrelated grouped diagnostics.

Until that census is complete, no new warning cohort is selected and the
work mode remains `design_stop`.
