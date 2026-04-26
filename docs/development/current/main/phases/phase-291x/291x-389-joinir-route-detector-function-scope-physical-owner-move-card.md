---
Status: Landed
Date: 2026-04-26
Scope: JoinIR route detector function-scope physical owner move
Related:
  - src/mir/loop_route_detection/support/function_scope/README.md
  - src/mir/loop_route_detection/support/function_scope/mod.rs
  - src/mir/loop_route_detection/support/function_scope/analyzers/mod.rs
  - src/mir/loop_route_detection/support/function_scope/analyzers/tests/
  - src/mir/loop_route_detection/support/function_scope/analyzers/v1.rs
  - src/mir/loop_route_detection/support/function_scope/analyzers/v2.rs
  - src/mir/loop_route_detection/support/function_scope/helpers.rs
  - src/mir/loop_route_detection/support/function_scope/types.rs
  - src/mir/loop_route_detection/support/mod.rs
  - src/mir/loop_route_detection/legacy/mod.rs
  - src/mir/loop_route_detection/legacy/README.md
  - src/mir/loop_route_detection/support/README.md
  - docs/development/current/main/phases/phase-291x/291x-388-joinir-route-detector-trim-physical-owner-move-card.md
---

# 291x-389: JoinIR Route Detector Function-Scope Physical Owner Move

## Goal

Move `support::function_scope` from private `legacy/` storage into the stable
support owner path.

This is a BoxShape-only directory-family physical owner move.

## Change

Moved:

```text
legacy/function_scope_capture/
  -> support/function_scope/
```

Added a local README for the new physical owner directory. Updated
`support/mod.rs` so `support::function_scope` is a real support module rather
than a re-export facade.

## Preserved Behavior

- Existing caller path remains:

```text
loop_route_detection::support::function_scope
```

- No route classifier behavior changed.
- `legacy/` stays private.

## Next Cleanup

Inventory and move the next support family:

```text
support::condition_scope
```

Keep one support family per commit and keep the no-regrowth guard green.

## Validation

```bash
bash tools/checks/route_detector_legacy_surface_guard.sh
cargo check --bin hakorune
```
