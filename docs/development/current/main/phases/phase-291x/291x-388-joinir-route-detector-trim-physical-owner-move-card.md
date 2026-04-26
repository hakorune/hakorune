---
Status: Landed
Date: 2026-04-26
Scope: JoinIR route detector trim physical owner move
Related:
  - src/mir/loop_route_detection/support/trim.rs
  - src/mir/loop_route_detection/support/mod.rs
  - src/mir/loop_route_detection/legacy/loop_body_carrier_promoter.rs
  - src/mir/loop_route_detection/legacy/mod.rs
  - src/mir/loop_route_detection/legacy/README.md
  - src/mir/loop_route_detection/support/README.md
  - docs/development/current/main/phases/phase-291x/291x-387-joinir-route-detector-break-condition-physical-owner-move-card.md
---

# 291x-388: JoinIR Route Detector Trim Physical Owner Move

## Goal

Move `support::trim` from private `legacy/` storage into the stable support
owner path.

This is a BoxShape-only physical owner move.

## Change

Moved:

```text
legacy/trim_loop_helper.rs
  -> support/trim.rs
```

Updated the remaining carrier promoter to reference `TrimLoopHelper` through
the stable support path. Updated support and legacy README boundary notes.

## Preserved Behavior

- Existing caller path remains:

```text
loop_route_detection::support::trim::TrimLoopHelper
```

- No route classifier behavior changed.
- `legacy/` stays private.

## Next Cleanup

Inventory and move the next support family:

```text
support::function_scope
```

Keep one support family per commit and keep the no-regrowth guard green.

## Validation

```bash
bash tools/checks/route_detector_legacy_surface_guard.sh
cargo check --bin hakorune
```
