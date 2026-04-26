---
Status: Landed
Date: 2026-04-26
Scope: JoinIR route detector support facade add
Related:
  - src/mir/loop_route_detection/mod.rs
  - src/mir/loop_route_detection/support/mod.rs
  - src/mir/loop_route_detection/support/README.md
  - docs/development/current/main/phases/phase-291x/291x-370-joinir-route-detector-compatibility-module-ownership-inventory-card.md
---

# 291x-371: JoinIR Route Detector Support Facade Add

## Goal

Add stable semantic owner paths for route detector support helpers without
moving physical files or migrating callers yet.

This is BoxShape-only. Do not change route behavior.

## Change

Added:

```text
src/mir/loop_route_detection/support/mod.rs
src/mir/loop_route_detection/support/README.md
```

and exposed:

```text
loop_route_detection::support::condition_scope
loop_route_detection::support::function_scope
loop_route_detection::support::trim
loop_route_detection::support::body_local::{carrier, condition}
loop_route_detection::support::break_condition
loop_route_detection::support::locals::{pinned, mutable_accumulator}
```

The facades currently re-export public items from private `legacy/` storage.

## Preserved Behavior

- No caller path changed.
- No parent compatibility export was removed.
- No physical file was moved.
- No route classifier behavior changed.

## Boundary Improvement

New migrations can target semantic owner paths instead of legacy-named parent
compatibility modules.

Physical file moves can happen later after callers no longer depend on the
compatibility exports.

## Next Cleanup

Migrate the small families first:

```text
break_condition_analyzer -> support::break_condition
pinned_local_analyzer -> support::locals::pinned
mutable_accumulator_analyzer -> support::locals::mutable_accumulator
```

Do not prune parent compatibility exports until source callers are gone.

## Non-Goals

- No caller migration in this card.
- No compatibility export deletion.
- No support helper logic change.

## Validation

```bash
cargo check -q
bash tools/checks/current_state_pointer_guard.sh
git diff --check
```
