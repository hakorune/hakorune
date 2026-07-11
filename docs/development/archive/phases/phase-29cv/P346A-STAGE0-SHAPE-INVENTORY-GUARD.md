---
Status: Completed
Decision: accepted
Date: 2026-05-03
Scope: executable guard for the Stage0 LLVM line shape inventory SSOT
Related:
  - docs/development/current/main/design/stage0-llvm-line-shape-inventory-ssot.md
  - docs/tools/check-scripts-index.md
  - src/mir/global_call_route_plan/model.rs
  - tools/checks/stage0_shape_inventory_guard.sh
---

# P346A: Stage0 Shape Inventory Guard

## Intent

Make the P345A Stage0 body-shape inventory executable.

The inventory already states that new `GlobalCallTargetShape` variants must
not appear without status, owner reading, and removal path. This card adds a
small check script so undocumented shape growth fails before a source-execution
cleanup card can accidentally widen Stage0.

## Boundary

This is BoxShape guard work only.

Allowed:

- add a `tools/checks` guard that compares the Rust enum with the design SSOT
- document the guard in the check scripts index
- update current-state pointers

Not allowed:

- add or remove `GlobalCallTargetShape` variants
- change route classification behavior
- add Stage0 C shim body-specific emitters
- accept a new source-execution body shape

## Implementation

- Added `tools/checks/stage0_shape_inventory_guard.sh`.
- The guard extracts `GlobalCallTargetShape` variants from
  `src/mir/global_call_route_plan/model.rs`.
- The guard requires each variant to appear in
  `docs/development/current/main/design/stage0-llvm-line-shape-inventory-ssot.md`.
- Updated `docs/tools/check-scripts-index.md` so new check surface remains
  discoverable.
- Wired the guard into `tools/checks/dev_gate.sh quick` so undocumented
  shape growth is caught by the daily lightweight gate.

## Acceptance

```bash
tools/checks/stage0_shape_inventory_guard.sh
bash -n tools/checks/dev_gate.sh
bash tools/checks/current_state_pointer_guard.sh
git diff --check
```

Expected:

- shape inventory guard reports the documented variant count
- current-state pointer guard reports `ok`
- diff check reports no whitespace errors
