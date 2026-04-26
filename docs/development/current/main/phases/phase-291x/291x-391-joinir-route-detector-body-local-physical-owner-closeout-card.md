---
Status: Landed
Date: 2026-04-26
Scope: JoinIR route detector body-local physical owner closeout
Related:
  - src/mir/loop_route_detection/support/body_local/README.md
  - src/mir/loop_route_detection/support/body_local/mod.rs
  - src/mir/loop_route_detection/support/body_local/carrier.rs
  - src/mir/loop_route_detection/support/body_local/condition.rs
  - src/mir/loop_route_detection/support/body_local/digitpos.rs
  - src/mir/loop_route_detection/support/body_local/digitpos_detector.rs
  - src/mir/loop_route_detection/support/body_local/trim_detector.rs
  - src/mir/loop_route_detection/support/mod.rs
  - src/mir/loop_route_detection/mod.rs
  - tools/checks/route_detector_legacy_surface_guard.sh
  - docs/tools/check-scripts-index.md
  - docs/development/current/main/phases/phase-291x/291x-390-joinir-route-detector-condition-scope-physical-owner-move-card.md
---

# 291x-391: JoinIR Route Detector Body-Local Physical Owner Closeout

## Goal

Move the remaining body-local support family out of route detector `legacy/`
storage and close the legacy storage path.

This is a BoxShape-only closeout card.

## Change

Moved:

```text
legacy/loop_body_carrier_promoter.rs -> support/body_local/carrier.rs
legacy/loop_body_cond_promoter.rs    -> support/body_local/condition.rs
legacy/loop_body_digitpos_promoter.rs -> support/body_local/digitpos.rs
legacy/digitpos_detector.rs          -> support/body_local/digitpos_detector.rs
legacy/trim_detector.rs              -> support/body_local/trim_detector.rs
```

Added:

```text
support/body_local/README.md
support/body_local/mod.rs
```

Removed the now-empty `legacy/` module declaration and storage files.

## Guard Tightening

`tools/checks/route_detector_legacy_surface_guard.sh` now rejects:

```text
mod legacy;
loop_route_detection::legacy
src/mir/loop_route_detection/legacy/* files
old compatibility module paths
```

## Preserved Behavior

- Existing caller paths remain:

```text
loop_route_detection::support::body_local::carrier
loop_route_detection::support::body_local::condition
```

- No route classifier behavior changed.
- `LoopFeatures -> classify() -> LoopRouteKind` remains the route-selection
  owner.

## Next Cleanup

Run a closeout review for stale route detector legacy mentions and decide the
next compiler-cleanliness seam.

## Validation

```bash
bash tools/checks/route_detector_legacy_surface_guard.sh
bash tools/checks/current_state_pointer_guard.sh
cargo check --bin hakorune
git diff --check
```
