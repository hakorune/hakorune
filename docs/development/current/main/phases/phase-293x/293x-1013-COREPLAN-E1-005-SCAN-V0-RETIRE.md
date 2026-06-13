---
Status: Landed
Date: 2026-06-14
Scope: fourth one-v0 retire slice.
Related:
  - docs/development/current/main/workstreams/compiler-foundation-current.md
  - tools/checks/coreplan_scan_v0_retire_guard.sh
  - tools/checks/coreplan_active_v0_inventory_guard.sh
  - src/mir/builder/control_flow/plan/REGISTRY.md
  - src/mir/builder/control_flow/plan/LEGACY_V0_BOUNDARY.md
---

# COREPLAN-E1-005: Scan V0 Retire

## Purpose

Retire `loop_scan_v0` after proving the focused comma/close and
`i <= n - 1` scan fixtures stay covered by the existing `LoopCondBreak` route.

## Decision

```text
one_v0_box_retired=1
retired_box=loop_scan_v0
replacement_owner=loop_cond_break_continue
accepted_shape_added=0
route_wiring_removed_for_one_box=1
facts_field_removed_for_one_box=1
recipe_module_removed_for_one_box=1
plan_module_removed_for_one_box=1
active_v0_box_count=2
summary=ok
```

## Guard

```bash
bash tools/checks/coreplan_scan_v0_retire_guard.sh
bash tools/checks/coreplan_active_v0_inventory_guard.sh
bash tools/smokes/v2/profiles/integration/joinir/phase29bq_fast_gate_vm.sh --only scan_loop_v0_comma_close_min
bash tools/smokes/v2/profiles/integration/joinir/phase29bq_fast_gate_vm.sh --only scan_loop_v0_lte_n_minus1_min
```

## Stop Lines

```text
do not retire another v0 box in this card
do not add a new accepted source shape
do not reintroduce scan_v0-specific route wiring
do not turn LoopCondBreak into a scan_v0-specific policy shelf
```
