---
Status: Landed
Date: 2026-04-26
Scope: JoinIR route detector direct legacy type export prune
Related:
  - src/mir/loop_route_detection/mod.rs
  - docs/development/current/main/phases/phase-291x/291x-355-joinir-route-detector-direct-type-export-inventory-card.md
---

# 291x-356: JoinIR Route Detector Direct Type Export Prune

## Goal

Remove direct legacy convenience type exports from the parent
`loop_route_detection` module.

This is BoxShape-only. Keep module-path compatibility stable.

## Change

Removed parent-level convenience type exports:

```text
DigitPosDetectionResult
DigitPosDetector
TrimDetectionResult
TrimDetector
TrimLoopHelper
```

The underlying legacy modules remain exported:

```text
digitpos_detector
trim_detector
trim_loop_helper
```

## Preserved Behavior

- No detector logic changed.
- No route classifier behavior changed.
- No legacy module moved or deleted.
- Existing module-path callers remain supported.

## Boundary Improvement

The parent module no longer presents detector helper structs as first-class
route detector surface.

Callers must use the owning legacy module path, which keeps ownership visible:

```text
crate::mir::loop_route_detection::trim_loop_helper::TrimLoopHelper
crate::mir::loop_route_detection::trim_detector::TrimDetector
crate::mir::loop_route_detection::digitpos_detector::DigitPosDetector
```

## Next Cleanup

Inventory legacy module exports that still have no non-legacy external callers.

Do not prune module exports without checking internal legacy imports first.

## Non-Goals

- No legacy module export pruning in this card.
- No internal import migration.
- No route-shape function deletion.

## Validation

```bash
cargo check -q
bash tools/checks/current_state_pointer_guard.sh
git diff --check
```
