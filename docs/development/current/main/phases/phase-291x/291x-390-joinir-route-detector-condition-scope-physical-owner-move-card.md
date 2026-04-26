---
Status: Landed
Date: 2026-04-26
Scope: JoinIR route detector condition-scope physical owner move
Related:
  - src/mir/loop_route_detection/support/condition_scope/README.md
  - src/mir/loop_route_detection/support/condition_scope/mod.rs
  - src/mir/loop_route_detection/support/condition_scope/var_analyzer.rs
  - src/mir/loop_route_detection/support/mod.rs
  - src/mir/loop_route_detection/legacy/loop_body_carrier_promoter.rs
  - src/mir/loop_route_detection/legacy/loop_body_cond_promoter.rs
  - src/mir/loop_route_detection/legacy/loop_body_digitpos_promoter.rs
  - src/mir/loop_route_detection/legacy/mod.rs
  - src/mir/loop_route_detection/legacy/README.md
  - src/mir/loop_route_detection/support/README.md
  - docs/development/current/main/phases/phase-291x/291x-389-joinir-route-detector-function-scope-physical-owner-move-card.md
---

# 291x-390: JoinIR Route Detector Condition-Scope Physical Owner Move

## Goal

Move `support::condition_scope` from private `legacy/` storage into the
stable support owner path.

This is a BoxShape-only family move. The private condition variable analyzer
moved with the condition-scope family because it is an implementation helper.

## Change

Moved:

```text
legacy/loop_condition_scope.rs
  -> support/condition_scope/mod.rs

legacy/condition_var_analyzer.rs
  -> support/condition_scope/var_analyzer.rs
```

Updated remaining body-local legacy modules to depend on the stable
`support::condition_scope` path instead of sibling legacy modules.

## Preserved Behavior

- Existing caller path remains:

```text
loop_route_detection::support::condition_scope
```

- No route classifier behavior changed.
- `legacy/` stays private.

## Next Cleanup

Move the remaining body-local support families:

```text
support::body_local::{carrier, condition}
```

These are more coupled than previous families, so keep the move focused and
validate with the no-regrowth guard plus `cargo check`.

## Validation

```bash
bash tools/checks/route_detector_legacy_surface_guard.sh
cargo check --bin hakorune
```
