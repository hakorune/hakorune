---
Status: Landed
Date: 2026-04-26
Scope: JoinIR route detector direct legacy type export inventory
Related:
  - src/mir/loop_route_detection/mod.rs
  - src/mir/loop_route_detection/legacy/mod.rs
  - docs/development/current/main/phases/phase-291x/291x-354-joinir-route-detector-legacy-wildcard-export-prune-card.md
---

# 291x-355: JoinIR Route Detector Direct Type Export Inventory

## Goal

Inventory direct legacy convenience type exports from
`crate::mir::loop_route_detection::*` before pruning them from the parent
module.

This is BoxShape-only. Do not delete modules or change route behavior in this
card.

## Findings

The parent module still exposes legacy convenience types directly:

```text
crate::mir::loop_route_detection::TrimLoopHelper
crate::mir::loop_route_detection::DigitPosDetectionResult
crate::mir::loop_route_detection::DigitPosDetector
crate::mir::loop_route_detection::TrimDetectionResult
crate::mir::loop_route_detection::TrimDetector
```

Repository search found no non-doc external caller using those direct root
paths.

Live code uses module paths instead:

```text
crate::mir::loop_route_detection::trim_loop_helper::TrimLoopHelper
crate::mir::loop_route_detection::trim_detector::TrimDetector
crate::mir::loop_route_detection::digitpos_detector::DigitPosDetector
```

Internal legacy modules also use sibling/module paths.

## Decision

The direct root convenience type exports can be pruned from
`src/mir/loop_route_detection/mod.rs`.

Keep the legacy modules exported for compatibility in the same prune card.

## Next Cleanup

Remove the direct root convenience type export group from the parent module:

```text
DigitPosDetectionResult
DigitPosDetector
TrimDetectionResult
TrimDetector
TrimLoopHelper
```

Do not delete the underlying legacy module exports.

## Non-Goals

- No legacy module deletion.
- No caller path migration.
- No detector behavior change.
- No route classifier behavior change.

## Validation

```bash
cargo check -q
bash tools/checks/current_state_pointer_guard.sh
git diff --check
```
