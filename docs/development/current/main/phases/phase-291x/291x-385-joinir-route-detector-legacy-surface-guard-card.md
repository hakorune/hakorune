---
Status: Landed
Date: 2026-04-26
Scope: JoinIR route detector legacy surface no-regrowth guard
Related:
  - tools/checks/route_detector_legacy_surface_guard.sh
  - tools/checks/dev_gate.sh
  - docs/tools/check-scripts-index.md
  - src/mir/loop_route_detection/mod.rs
  - src/mir/loop_route_detection/support/mod.rs
  - docs/development/current/main/phases/phase-291x/291x-384-joinir-route-detector-legacy-support-readme-boundary-card.md
---

# 291x-385: JoinIR Route Detector Legacy Surface Guard

## Goal

Prevent the route detector private `legacy/` storage from becoming public
surface again while physical owner moves are still in progress.

This is a BoxShape guard card. It does not add a route shape or change
classification behavior.

## Change

Added:

```text
tools/checks/route_detector_legacy_surface_guard.sh
```

The guard rejects:

```text
pub mod legacy
pub use legacy::
loop_route_detection::<old compatibility module>
loop_route_detection::legacy outside support/mod.rs
```

Integrated the guard into:

```text
tools/checks/dev_gate.sh quick
docs/tools/check-scripts-index.md
```

## Preserved Behavior

- `src/mir/loop_route_detection/legacy/` remains private implementation
  storage.
- `src/mir/loop_route_detection/support/mod.rs` remains the temporary public
  semantic facade over private legacy storage.
- Route selection remains `LoopFeatures -> classify() -> LoopRouteKind`.

## Next Cleanup

Inventory and start family-sized physical owner moves from `legacy/` into
`support/`, starting with the smallest support families. Keep the compatibility
surface closed while moving files.

## Non-Goals

- No physical files moved in this card.
- No classifier or route API changes.
- No support facade split beyond the existing semantic modules.

## Validation

```bash
bash tools/checks/route_detector_legacy_surface_guard.sh
bash tools/checks/current_state_pointer_guard.sh
git diff --check
```
