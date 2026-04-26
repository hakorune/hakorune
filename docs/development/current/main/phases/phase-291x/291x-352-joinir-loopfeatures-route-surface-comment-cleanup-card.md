---
Status: Landed
Date: 2026-04-26
Scope: JoinIR LoopFeatures route surface comment cleanup
Related:
  - src/mir/loop_route_detection/mod.rs
  - src/mir/loop_route_detection/classify.rs
  - src/mir/join_ir/lowering/loop_route_router.rs
  - docs/development/current/main/phases/phase-291x/291x-351-joinir-loopfeatures-route-surface-review-card.md
---

# 291x-352: JoinIR LoopFeatures Route Surface Comment Cleanup

## Goal

Align comments and module docs with the pruned `LoopFeatures` route surface.

This is docs/comment-only BoxShape cleanup.

## Change

Clarified:

```text
LoopFeatures classify owns flat route families.
NestedLoopMinimal remains a route kind but is selected by AST/StepTree.
LoopForm router uses LoopForm-observable flat route facts.
AST route path does StepTree nested-loop precheck before flat classify.
```

Updated:

```text
src/mir/loop_route_detection/mod.rs
src/mir/loop_route_detection/classify.rs
src/mir/join_ir/lowering/loop_route_router.rs
```

## Preserved Behavior

- No code behavior changed.
- No route-kind enum changed.
- No classifier behavior changed.

## Non-Goals

- No legacy export cleanup.
- No nested_minimal lowerer change.
- No route path rewrite.

## Validation

```bash
cargo check -q
bash tools/checks/current_state_pointer_guard.sh
git diff --check
```
