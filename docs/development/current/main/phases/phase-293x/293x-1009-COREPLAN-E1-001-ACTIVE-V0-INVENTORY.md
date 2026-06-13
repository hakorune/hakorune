---
Status: Landed
Date: 2026-06-14
Scope: active routed loop_*_v0 inventory guard.
Related:
  - docs/development/current/main/workstreams/compiler-foundation-current.md
  - docs/development/current/main/phases/phase-293x/293x-1006-COREPLAN-FOUND-002-REMAINING-FAMILY-INVENTORY.md
  - tools/checks/coreplan_active_v0_inventory_guard.sh
  - src/mir/builder/control_flow/plan/REGISTRY.md
  - src/mir/builder/control_flow/plan/LEGACY_V0_BOUNDARY.md
---

# COREPLAN-E1-001: Active V0 Inventory

## Purpose

Make active routed `loop_*_v0` surfaces mechanically visible before retiring
one of them.

E1 has two separate closeout surfaces:

```text
legacy normalizer table
active routed loop_*_v0 boxes
```

The first can be empty while the second is still active. This card fixes that
separation and adds a guard for the active routed v0 surface.

## Decision

```text
active_v0_inventory_guard=1
active_v0_box_count_reported=1
legacy_normalizer_empty_and_active_v0_empty_are_separate=1
one_v0_box_per_retire_slice=1
release_default_changed=0
accepted_shape_added=0
summary=ok
```

## Guard

```bash
bash tools/checks/coreplan_active_v0_inventory_guard.sh
```

The guard tracks the current active set:

```text
loop_scan_v0
loop_scan_methods_v0
loop_scan_methods_block_v0
loop_scan_phi_vars_v0
loop_collect_using_entries_v0
loop_bundle_resolver_v0
```

## Stop Lines

```text
do not remove active v0 route wiring without fixture/gate proof
do not mix planner_compat facade retirement with v0 route removal
do not add new loop_*_v0 boxes
remove only one v0 box per slice
```
