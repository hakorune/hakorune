---
Status: Landed
Date: 2026-04-26
Scope: JoinIR route detector legacy convenience re-export inventory
Related:
  - src/mir/loop_route_detection/legacy/mod.rs
  - docs/development/current/main/phases/phase-291x/291x-363-joinir-route-detector-legacy-route-function-definition-prune-card.md
---

# 291x-364: JoinIR Route Detector Legacy Convenience Re-Export Inventory

## Goal

Inventory convenience re-exports that remain inside
`src/mir/loop_route_detection/legacy/mod.rs`.

This is BoxShape-only. Do not prune re-exports in this card.

## Findings

The legacy module still re-exports:

```text
TrimLoopHelper
DigitPosDetectionResult
DigitPosDetector
TrimDetectionResult
TrimDetector
```

Repository search found no caller using these shortcut paths:

```text
crate::mir::loop_route_detection::legacy::TrimLoopHelper
crate::mir::loop_route_detection::legacy::DigitPosDetector
crate::mir::loop_route_detection::legacy::TrimDetector
```

Live code uses owning module paths instead:

```text
trim_loop_helper::TrimLoopHelper
digitpos_detector::DigitPosDetector
trim_detector::TrimDetector
```

## Decision

The convenience re-exports can be pruned from `legacy/mod.rs`.

Keep the owning modules exported.

## Next Cleanup

Remove:

```text
pub use trim_loop_helper::TrimLoopHelper;
pub use digitpos_detector::{DigitPosDetectionResult, DigitPosDetector};
pub use trim_detector::{TrimDetectionResult, TrimDetector};
```

Do not delete the owning modules.

## Non-Goals

- No detector/promoter module deletion.
- No caller migration.
- No route classifier behavior change.

## Validation

```bash
cargo check -q
bash tools/checks/current_state_pointer_guard.sh
git diff --check
```
