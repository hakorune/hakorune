---
Status: Landed
Date: 2026-04-26
Scope: JoinIR LoopFeatures break/continue count prune
Related:
  - src/mir/loop_route_detection/features.rs
  - src/mir/builder/control_flow/facts/ast_feature_extractor.rs
  - src/mir/loop_route_detection/tests.rs
  - docs/development/current/main/phases/phase-291x/291x-347-joinir-loopfeatures-count-fields-inventory-card.md
---

# 291x-348: JoinIR LoopFeatures Break/continue Count Prune

## Goal

Remove dead `LoopFeatures.break_count` and `LoopFeatures.continue_count` fields
identified by 291x-347.

This is behavior-preserving BoxShape cleanup.

## Change

Removed:

```text
LoopFeatures.break_count
LoopFeatures.continue_count
LoopForm producer count locals
AST producer count assignments
test initializers for both fields
```

Preserved:

```text
LoopFeatures.has_break
LoopFeatures.has_continue
LoopFeatures.carrier_count
```

## Boundary After Prune

Break/continue route facts are boolean in `LoopFeatures`.

```text
LoopFeatures.has_break
LoopFeatures.has_continue
```

Detailed control-flow counts remain in the specialized AST counter utilities,
not in the route feature vector.

## Preserved Behavior

- `classify(...)` still reads `has_break` and `has_continue`.
- LoopForm route extraction still derives booleans from target vectors.
- AST route extraction still uses its precomputed booleans.
- No route shape is added or removed.

## Non-Goals

- No carrier-count behavior change.
- No control-flow counter utility change.
- No route classifier behavior change.

## Validation

```bash
rg -n "break_count|continue_count" src/mir/loop_route_detection src/mir/builder/control_flow/facts/ast_feature_extractor.rs -g '*.rs' || true
cargo test -q loop_update_
cargo check -q
bash tools/checks/current_state_pointer_guard.sh
git diff --check
```
