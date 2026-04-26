---
Status: Landed
Date: 2026-04-26
Scope: JoinIR route detector compatibility module ownership closeout
Related:
  - src/mir/loop_route_detection/mod.rs
  - src/mir/loop_route_detection/support/mod.rs
  - src/mir/loop_route_detection/support/README.md
  - docs/development/current/main/phases/phase-291x/291x-370-joinir-route-detector-compatibility-module-ownership-inventory-card.md
  - docs/development/current/main/phases/phase-291x/291x-381-joinir-route-detector-body-local-compatibility-export-prune-card.md
---

# 291x-382: JoinIR Route Detector Compatibility Module Ownership Closeout

## Goal

Close out the route detector compatibility module ownership series.

This is docs/comment-only BoxShape cleanup.

## Result

The parent `loop_route_detection` public surface is now:

```text
classify
LoopFeatures
LoopRouteKind
support
```

The parent no longer re-exports legacy-named support modules.

Stable support owner paths are:

```text
loop_route_detection::support::condition_scope
loop_route_detection::support::function_scope
loop_route_detection::support::trim
loop_route_detection::support::body_local::{carrier, condition}
loop_route_detection::support::break_condition
loop_route_detection::support::locals::{pinned, mutable_accumulator}
```

`legacy/` is private implementation storage.

## Change

Aligned the parent module docs with the final surface:

```text
private legacy storage
public support facades
classifier API is the route-selection API
```

## Preserved Behavior

- No code behavior changed.
- No visibility changed in this card.
- No route classifier behavior changed.
- No route lowerer behavior changed.

## Next Cleanup

Inventory whether any support facade should become a physical owner module next.

Do not move files until that inventory separates:

```text
semantic owner module target
current legacy storage file
caller count
test/gate coverage
```

## Non-Goals

- No physical file move.
- No support facade deletion.
- No route classifier API change.

## Validation

```bash
cargo check -q
bash tools/checks/current_state_pointer_guard.sh
git diff --check
```
