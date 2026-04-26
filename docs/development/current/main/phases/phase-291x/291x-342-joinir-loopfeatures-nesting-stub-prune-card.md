---
Status: Landed
Date: 2026-04-26
Scope: JoinIR LoopFeatures nesting stub prune
Related:
  - src/mir/loop_route_detection/features.rs
  - src/mir/loop_route_detection/classify.rs
  - src/mir/join_ir/lowering/loop_route_router.rs
  - docs/development/current/main/phases/phase-291x/291x-341-joinir-loopfeatures-nesting-stub-inventory-card.md
---

# 291x-342: JoinIR LoopFeatures Nesting Stub Prune

## Goal

Remove the dead `LoopFeatures` nesting fields identified by 291x-341.

This is behavior-preserving BoxShape cleanup. NestedLoopMinimal remains owned
by the StepTree/AST route path.

## Change

Removed from `LoopFeatures`:

```text
max_loop_depth
has_inner_loops
```

Removed from `loop_route_detection::classify(...)`:

```text
LoopFeatures.max_loop_depth > 2 guard
LoopFeatures.max_loop_depth == 2 && has_inner_loops -> NestedLoopMinimal
```

Removed from the LoopForm route router:

```text
Unknown route depth-specific diagnostic based on LoopFeatures.max_loop_depth
```

## Boundary After Prune

`LoopFeatures` is now a flat route-shape feature vector.

```text
LoopFeatures
  -> break / continue / if / carrier_count / infinite-loop flag
```

Nested-loop facts stay on StepTree:

```text
StepTreeFeatures.max_loop_depth
  -> builder/control_flow/joinir/routing.rs
  -> LoopRouteKind::NestedLoopMinimal
```

## Preserved Behavior

- NestedLoopMinimal route selection remains in the AST routing path.
- `LoopRouteKind::NestedLoopMinimal` is preserved.
- StepTree nesting fields are unchanged.
- No new route shape is accepted.

## Non-Goals

- No StepTree behavior change.
- No nested_minimal lowerer change.
- No route-kind enum deletion.
- No AST routing change.

## Validation

```bash
rg -n "LoopFeatures.*max_loop_depth|has_inner_loops" src/mir -g '*.rs' || true
cargo test -q loop_update_
cargo check -q
bash tools/checks/current_state_pointer_guard.sh
git diff --check
```
