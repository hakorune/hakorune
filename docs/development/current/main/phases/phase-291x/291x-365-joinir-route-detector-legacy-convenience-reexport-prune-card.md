---
Status: Landed
Date: 2026-04-26
Scope: JoinIR route detector legacy convenience re-export prune
Related:
  - src/mir/loop_route_detection/legacy/mod.rs
  - docs/development/current/main/phases/phase-291x/291x-364-joinir-route-detector-legacy-convenience-reexport-inventory-card.md
---

# 291x-365: JoinIR Route Detector Legacy Convenience Re-Export Prune

## Goal

Remove unused convenience re-exports from
`src/mir/loop_route_detection/legacy/mod.rs`.

This is BoxShape-only. Keep owning modules exported.

## Change

Removed:

```text
pub use trim_loop_helper::TrimLoopHelper;
pub use digitpos_detector::{DigitPosDetectionResult, DigitPosDetector};
pub use trim_detector::{TrimDetectionResult, TrimDetector};
```

Kept:

```text
pub mod trim_loop_helper;
pub mod digitpos_detector;
pub mod trim_detector;
```

## Preserved Behavior

- No detector/promoter module was deleted.
- No caller path changed.
- No route classifier behavior changed.

## Boundary Improvement

`legacy/mod.rs` no longer provides shortcut type exports. Users must import
types from their owning legacy support modules.

## Next Cleanup

Review the final `loop_route_detection` parent/legacy surface and close out the
route detector export cleanup if no further stale route API remains.

## Non-Goals

- No detector/promoter module deletion.
- No route lowerer change.
- No classifier API change.

## Validation

```bash
cargo check -q
bash tools/checks/current_state_pointer_guard.sh
git diff --check
```
