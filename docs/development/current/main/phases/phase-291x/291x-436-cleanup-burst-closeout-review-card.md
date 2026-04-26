---
Status: Landed
Date: 2026-04-27
Scope: cleanup burst closeout review
Related:
  - CURRENT_TASK.md
  - docs/development/current/main/CURRENT_STATE.toml
  - docs/development/current/main/phases/phase-291x/291x-430-cleanup-closeout-granularity-card.md
  - docs/development/current/main/phases/phase-291x/291x-435-loop-if-break-continue-scope-wording-cleanup-card.md
---

# 291x-436: Cleanup Burst Closeout Review

## Goal

Close the normalized-shadow / normalization cleanup burst and prevent further
open-ended small cleanup.

This is a closeout review. No behavior changed.

## Closed In This Burst

- ANF status wording now matches active P1/P2 paths.
- Normalized-shadow root facade exports were pruned.
- Suffix router ownership moved under `control_flow/normalization`.
- If-only baseline is fenced as a historical fossil boundary.
- Loop-if-exit wording uses route-decline terminology.
- Normalization docs use default-path wording instead of legacy-fallback
  wording.
- Loop-if-break-continue scope wording matches active P0/P1/P2 support.

## Guards

```bash
bash tools/checks/current_state_pointer_guard.sh
bash tools/checks/core_method_contract_inc_no_growth_guard.sh
rg -n "graceful fallback|legacy fallback|use legacy|Legacy path|P0 stub|not yet used in execute_box|Else branch: Not supported|Phase 143 P0: loop\\(true\\) \\+ if \\+ break|route_entry::policies::normalized_shadow_suffix_router_box|cleanup::policies::normalized_shadow_suffix_router_box" \
  src/mir/control_tree/normalized_shadow \
  src/mir/builder/control_flow/normalization \
  src/mir/builder/control_flow/joinir/route_entry \
  -g '*.rs' -g '*.md'
```

The final `rg` produced no output.

## Deferred To New Lanes

These are larger than the cleanup burst and should not be pulled into more
small cards here:

- Stage-B adapter thinning
- CoreMethodContract -> CoreOp / LoweringPlan migration
- `.inc` generated enum/table consumer migration
- MapGet return-shape metadata / proof / lowering work
- route-entry router compatibility around canonicalizer absence

## Decision

Stop this cleanup burst here. The next card should choose the next lane, not
continue normalized-shadow wording cleanup by default.
