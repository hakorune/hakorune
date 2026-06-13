---
Status: Landed
Date: 2026-06-14
Scope: second one-v0 retire pilot.
Related:
  - docs/development/current/main/workstreams/compiler-foundation-current.md
  - tools/checks/coreplan_collect_using_entries_v0_retire_guard.sh
  - tools/checks/coreplan_active_v0_inventory_guard.sh
  - src/mir/builder/control_flow/plan/REGISTRY.md
  - src/mir/builder/control_flow/plan/LEGACY_V0_BOUNDARY.md
---

# COREPLAN-E1-003: Collect Using Entries V0 Retire

## Purpose

Retire `loop_collect_using_entries_v0` after proving the focused
Stage1UsingResolver `_collect_using_entries` fixture stays on an existing route.

## Decision

```text
one_v0_box_retired=1
retired_box=loop_collect_using_entries_v0
replacement_owner=loop_simple_while
accepted_shape_added=0
route_wiring_removed_for_one_box=1
facts_field_removed_for_one_box=1
recipe_module_removed_for_one_box=1
plan_module_removed_for_one_box=1
active_v0_box_count=4
summary=ok
```

## Guard

```bash
bash tools/checks/coreplan_collect_using_entries_v0_retire_guard.sh
bash tools/checks/coreplan_active_v0_inventory_guard.sh
bash tools/smokes/v2/profiles/integration/joinir/phase29bq_fast_gate_vm.sh --only selfhost_collect_using_entries_loop_min
```

## Stop Lines

```text
do not retire another v0 box in this card
do not add a new accepted source shape
do not reintroduce collect_using_entries-specific route wiring
```
