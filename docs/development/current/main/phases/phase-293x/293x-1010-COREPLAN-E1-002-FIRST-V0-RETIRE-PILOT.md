---
Status: Landed
Date: 2026-06-14
Scope: first one-v0 retire pilot.
Related:
  - docs/development/current/main/workstreams/compiler-foundation-current.md
  - docs/development/current/main/phases/phase-293x/293x-1009-COREPLAN-E1-001-ACTIVE-V0-INVENTORY.md
  - tools/checks/coreplan_first_v0_retire_guard.sh
  - tools/checks/coreplan_active_v0_inventory_guard.sh
  - src/mir/builder/control_flow/plan/REGISTRY.md
  - src/mir/builder/control_flow/plan/LEGACY_V0_BOUNDARY.md
---

# COREPLAN-E1-002: First V0 Retire Pilot

## Purpose

Retire exactly one active `loop_*_v0` box without adding a new accepted source
shape.

Selected box:

```text
loop_scan_methods_block_v0
```

Replacement owner:

```text
loop_scan_methods_v0
```

## Decision

```text
one_v0_box_retired=1
retired_box=loop_scan_methods_block_v0
replacement_owner=loop_scan_methods_v0
accepted_shape_added=0
route_wiring_removed_for_one_box=1
facts_field_removed_for_one_box=1
recipe_module_removed_for_one_box=1
plan_module_removed_for_one_box=1
active_v0_box_count=5
summary=ok
```

## Implementation Boundary

`loop_scan_methods_block_v0` used to exist only to recognize block-wrapped
scan-methods bodies. That observation is now owned by `loop_scan_methods_v0`:

```text
block-wrapped scan_methods body
  -> flatten_scope_boxes
  -> loop_scan_methods_v0 facts
  -> LinearBlockRecipe::{NoExit|ExitAllowed}
  -> loop_scan_methods_v0 lowering
```

No new route, source shape, or fallback was added.

## Guard

```bash
bash tools/checks/coreplan_first_v0_retire_guard.sh
bash tools/checks/coreplan_active_v0_inventory_guard.sh
```

The first guard checks that:

```text
loop_scan_methods_block_v0 code references are absent
loop_scan_methods_v0 owns ExitAllowed linear segments
loop_scan_methods_v0 lowering calls lower_exit_allowed_block_verified
REGISTRY / LEGACY boundary keep retired history
```

## Stop Lines

```text
do not retire a second v0 box in this card
do not add a new loop_*_v0 box
do not remove historical docs/archive cards
do not treat TypeAbiCatalog or JoinIR as the acceptance truth
```
