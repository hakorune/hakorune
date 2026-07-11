---
Status: Landed
Date: 2026-04-26
Scope: JoinIR LoopFeatures nesting stub inventory
Related:
  - src/mir/loop_route_detection/features.rs
  - src/mir/loop_route_detection/classify.rs
  - src/mir/join_ir/lowering/loop_route_router.rs
  - src/mir/builder/control_flow/joinir/routing.rs
  - src/mir/control_tree/step_tree/types.rs
---

# 291x-341: JoinIR LoopFeatures Nesting Stub Inventory

## Goal

Inventory whether `LoopFeatures.max_loop_depth` and
`LoopFeatures.has_inner_loops` are still live route-feature fields.

This is BoxShape-only. Do not change route behavior in this card.

## Findings

`LoopFeatures` still carries nesting fields:

```text
max_loop_depth
has_inner_loops
```

But no `LoopFeatures` producer writes real nesting data.

```text
loop_route_detection::features::extract_features(loop_form)
  -> max_loop_depth = 1
  -> has_inner_loops = false

builder/control_flow/facts/ast_feature_extractor::extract_features(...)
  -> LoopFeatures { ..Default::default() }

Case-A / LoopViewBuilder stubs
  -> LoopFeatures { ..Default::default() }
```

The live NestedLoopMinimal route selection does not use `LoopFeatures`
nesting fields. It uses StepTree directly in the AST routing path:

```text
builder/control_flow/joinir/routing.rs
  -> StepTreeBuilderBox::build_from_ast(...)
  -> tree.features.max_loop_depth == 2
  -> is_nested_loop_minimal_lowerable(...)
  -> LoopRouteKind::NestedLoopMinimal
```

StepTree owns real nesting observation:

```text
control_tree::step_tree::StepTreeFeatures.max_loop_depth
```

## Decision

`LoopFeatures.max_loop_depth` and `LoopFeatures.has_inner_loops` are dead
reserved surfaces.

Keeping them creates a false second owner for nested-loop facts. The route
boundary should be:

```text
StepTree
  -> nesting facts / NestedLoopMinimal selection

LoopFeatures
  -> flat route-shape facts only
```

## Next Cleanup

Prune the `LoopFeatures` nesting stub surface:

```text
LoopFeatures.max_loop_depth
LoopFeatures.has_inner_loops
classify(...) NestedLoopMinimal/depth branch based on LoopFeatures
loop_route_router.rs depth-specific Unknown diagnostic
```

Preserve:

```text
LoopRouteKind::NestedLoopMinimal
StepTreeFeatures.max_loop_depth
builder/control_flow/joinir/routing.rs NestedLoopMinimal selection
nested_minimal lowerer/module
```

## Non-Goals

- No NestedLoopMinimal route behavior change.
- No StepTree behavior change.
- No AST routing behavior change.
- No route-kind enum deletion.

## Validation

```bash
rg -n "LoopFeatures.*max_loop_depth|has_inner_loops" src/mir -g '*.rs'
bash tools/checks/current_state_pointer_guard.sh
git diff --check
```
