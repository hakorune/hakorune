---
Status: Landed
Date: 2026-04-26
Scope: JoinIR route detector legacy route function export prune
Related:
  - src/mir/loop_route_detection/mod.rs
  - src/mir/join_ir/lowering/loop_routes/simple_while.rs
  - src/mir/join_ir/lowering/loop_routes/with_break.rs
  - src/mir/join_ir/lowering/loop_routes/with_continue.rs
  - src/mir/join_ir/lowering/loop_routes/with_if_phi.rs
  - docs/development/current/main/phases/phase-291x/291x-360-joinir-route-detector-legacy-route-function-export-inventory-card.md
---

# 291x-361: JoinIR Route Detector Legacy Route Function Export Prune

## Goal

Remove parent-module exports for legacy route-shape detector functions.

This is BoxShape-only. Keep the legacy definitions for a separate inventory.

## Change

Removed the parent export group:

```text
is_if_phi_join_route
is_loop_break_route
is_loop_continue_only_route
is_loop_simple_while_route
```

Updated stale `rust,ignore` examples in route lowerer stubs to point at the
current route-selection authority instead:

```text
LoopFeatures -> classify() -> LoopRouteKind
```

## Preserved Behavior

- No route classifier behavior changed.
- No route lowerer behavior changed.
- No legacy function definition was deleted.

## Boundary Improvement

The parent `loop_route_detection` module no longer advertises Phase 188
route-shape functions as the current route-selection entry.

Current callers should use the structure classifier surface.

## Next Cleanup

Inventory whether the legacy route-shape function definitions in
`src/mir/loop_route_detection/legacy/mod.rs` can be deleted or moved behind a
more explicit legacy-only test surface.

## Non-Goals

- No legacy function definition deletion.
- No route lowerer implementation change.
- No classifier API change.

## Validation

```bash
cargo check -q
bash tools/checks/current_state_pointer_guard.sh
git diff --check
```
