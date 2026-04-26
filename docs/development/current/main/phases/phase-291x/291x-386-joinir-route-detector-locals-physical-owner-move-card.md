---
Status: Landed
Date: 2026-04-26
Scope: JoinIR route detector locals physical owner move
Related:
  - src/mir/loop_route_detection/support/locals/README.md
  - src/mir/loop_route_detection/support/locals/mod.rs
  - src/mir/loop_route_detection/support/locals/pinned.rs
  - src/mir/loop_route_detection/support/locals/mutable_accumulator.rs
  - src/mir/loop_route_detection/support/mod.rs
  - src/mir/loop_route_detection/legacy/mod.rs
  - src/mir/loop_route_detection/legacy/README.md
  - src/mir/loop_route_detection/support/README.md
  - docs/development/current/main/phases/phase-291x/291x-385-joinir-route-detector-legacy-surface-guard-card.md
---

# 291x-386: JoinIR Route Detector Locals Physical Owner Move

## Goal

Move the lowest-risk route detector support family from private `legacy/`
storage into its stable semantic owner path.

This is a BoxShape-only physical owner move.

## Change

Moved:

```text
legacy/pinned_local_analyzer.rs
  -> support/locals/pinned.rs

legacy/mutable_accumulator_analyzer.rs
  -> support/locals/mutable_accumulator.rs
```

Added:

```text
support/locals/mod.rs
```

Updated `support/mod.rs` so `support::locals` is a real module owner rather
than a re-export facade over `legacy/`.

## Preserved Behavior

- Existing caller path remains:

```text
loop_route_detection::support::locals::{pinned, mutable_accumulator}
```

- No route classifier behavior changed.
- `legacy/` stays private.

## Why Locals First

The locals family had no internal dependencies on the remaining route detector
legacy families. Existing non-legacy callers were already migrated to the
`support::locals::*` path, so this move only changes physical ownership.

## Next Cleanup

Move the next low-risk family:

```text
support::break_condition
```

Keep one support family per commit and keep the no-regrowth guard green.

## Validation

```bash
bash tools/checks/route_detector_legacy_surface_guard.sh
cargo check --bin hakorune
```
