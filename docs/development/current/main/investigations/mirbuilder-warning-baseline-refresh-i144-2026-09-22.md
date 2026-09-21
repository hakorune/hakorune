---
Status: design_stop__2026-09-22__WarningBaselineRefreshI144__NoSafeSlice
Task: MIRBUILDER-WARNING-BASELINE-REFRESH-I144
Date: 2026-09-22
Parent: mirbuilder-warning-initial-source-missing-slot-retire-i143-2026-09-22.md
Implementation permission: false; refresh diagnostics and select one finite next cohort
NextCard: owner-decision__I144_warning_census
---

# MirBuilder warning baseline refresh I144

## Six-line brief

```text
Decision: refresh the warning baseline after I143 and select at most one
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

I143 measured lib **1,675 warnings** and lib-test **543 warnings**. I144 must
rerun the quick baseline and classify proposed warnings as
`current-change failure`, `known baseline debt`, or `informational census`.
The retired I139, I141, and I143 symbols are not candidates again. A new
candidate needs a complete owner/caller/delete-set tuple before selection.

Boundary: quick-profile `src/lib.rs` diagnostics -> one selected owner and
its focused guard; includes Rust production and test references for the
candidate, excludes semantic expansion, feature-only consumers, historical
snapshots, and unrelated grouped diagnostics.

Until that census is complete, no new warning cohort is selected and the
work mode remains `design_stop`.
