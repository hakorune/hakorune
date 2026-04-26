---
Status: Landed
Date: 2026-04-26
Scope: JoinIR route detector break-condition physical owner move
Related:
  - src/mir/loop_route_detection/support/break_condition.rs
  - src/mir/loop_route_detection/support/mod.rs
  - src/mir/loop_route_detection/legacy/mod.rs
  - src/mir/loop_route_detection/legacy/README.md
  - src/mir/loop_route_detection/support/README.md
  - src/mir/builder/control_flow/facts/ast_feature_extractor.rs
  - docs/development/current/main/phases/phase-291x/291x-386-joinir-route-detector-locals-physical-owner-move-card.md
---

# 291x-387: JoinIR Route Detector Break-Condition Physical Owner Move

## Goal

Move `support::break_condition` from private `legacy/` storage into the
stable support owner path.

This is a BoxShape-only physical owner move.

## Change

Moved:

```text
legacy/break_condition_analyzer.rs
  -> support/break_condition.rs
```

Updated `support/mod.rs` so `support::break_condition` is a real support
module rather than a re-export facade. Removed the legacy module declaration
and updated local README boundary notes.

## Preserved Behavior

- Existing caller path remains:

```text
loop_route_detection::support::break_condition::BreakConditionAnalyzer
```

- No route classifier behavior changed.
- `legacy/` stays private.

## Next Cleanup

Move the next low-risk facade:

```text
support::trim
```

Keep one support family per commit and keep the no-regrowth guard green.

## Validation

```bash
bash tools/checks/route_detector_legacy_surface_guard.sh
cargo check --bin hakorune
```
