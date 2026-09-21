---
Status: design_stop__2026-09-22__WarningBaselineRefreshI148__NoSafeSlice
Task: MIRBUILDER-WARNING-BASELINE-REFRESH-I148
Date: 2026-09-22
Parent: mirbuilder-warning-parser-admission-rows-test-accessor-i147-2026-09-22.md
Implementation permission: false; refresh diagnostics and select one finite next cohort
NextCard: owner-decision__I148_warning_census
---

# MirBuilder warning baseline refresh I148

## Six-line brief

```text
Decision: refresh the warning baseline after I147 and select at most one
  production-zero declaration or caller-zero old edge.
Source authority + canonical issuer: quick-profile Cargo diagnostics, the
  warning classification policy, and the selected owner's source references.
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

I147 closed with **1,672 lib warnings**, **543 lib-test warnings**, and the
focused admission evidence 3/3. I148 must rerun the quick baseline and
classify the proposed diagnostic as `current-change failure`, `known baseline
debt`, or `informational census`. Retired I139, I141, I143, I145, and I147
symbols are not candidates again. A new candidate requires a complete
owner/caller/delete-set tuple and focused guard before implementation.

The LoopBreak old-edge lane remains a priority check, not automatic
permission: `route_loop_break_recipe` must prove caller-zero and a successor
for its live compatibility caller. If it remains live, I148 may select one
safe production-zero warning row instead.
