---
Status: Landed
Date: 2026-04-26
Scope: JoinIR route detector legacy internal import owner-path migration
Related:
  - src/mir/loop_route_detection/legacy/loop_body_carrier_promoter.rs
  - src/mir/loop_route_detection/legacy/loop_body_cond_promoter.rs
  - src/mir/loop_route_detection/legacy/loop_body_digitpos_promoter.rs
  - docs/development/current/main/phases/phase-291x/291x-357-joinir-route-detector-unused-module-export-inventory-card.md
---

# 291x-358: JoinIR Route Detector Legacy Internal Import Owner-Path

## Goal

Stop legacy-internal detector helpers from importing through the parent
`crate::mir::loop_route_detection` compatibility surface.

This is BoxShape-only. Do not prune parent exports in this card.

## Change

Moved these legacy-internal imports to owner-local paths:

```text
loop_body_carrier_promoter -> super::trim_detector::TrimDetector
loop_body_cond_promoter -> super::loop_body_digitpos_promoter::{...}
loop_body_digitpos_promoter -> super::digitpos_detector::DigitPosDetector
```

## Preserved Behavior

- No detector logic changed.
- No route classifier behavior changed.
- No parent module export was removed.
- No non-legacy caller path changed.

## Boundary Improvement

The parent `loop_route_detection` module is now only needed for external
compatibility for those helper modules, not for legacy-internal routing.

This makes the next module export prune local and auditable.

## Next Cleanup

Prune parent-module exports for legacy-internal-only modules:

```text
condition_var_analyzer
loop_body_digitpos_promoter
digitpos_detector
trim_detector
```

Keep externally used module exports.

## Non-Goals

- No parent module export pruning in this card.
- No route-shape function deletion.
- No caller migration outside legacy internals.

## Validation

```bash
cargo check -q
bash tools/checks/current_state_pointer_guard.sh
git diff --check
```
