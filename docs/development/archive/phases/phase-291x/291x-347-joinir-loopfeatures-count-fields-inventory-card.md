---
Status: Landed
Date: 2026-04-26
Scope: JoinIR LoopFeatures count fields inventory
Related:
  - src/mir/loop_route_detection/features.rs
  - src/mir/loop_route_detection/classify.rs
  - src/mir/builder/control_flow/facts/ast_feature_extractor.rs
  - src/mir/join_ir/lowering/loop_scope_shape/case_a_lowering_shape.rs
---

# 291x-347: JoinIR LoopFeatures Count Fields Inventory

## Goal

Inventory the remaining count fields on `LoopFeatures`.

This is BoxShape-only. Do not change route behavior in this card.

## Findings

`LoopFeatures` still carries:

```text
carrier_count
break_count
continue_count
```

`carrier_count` is live:

```text
classify(...)
  -> IfPhiJoin when has_if && carrier_count >= 1

CaseALoweringShape / LoopViewBuilder
  -> builds stub LoopFeatures with carrier_count from LoopScopeShape
```

`break_count` and `continue_count` are not read by any live route decision after
291x-344 removed `classify_with_diagnosis(...)`.

```text
features.rs
  -> writes break_count / continue_count

ast_feature_extractor.rs
  -> writes break_count / continue_count

tests.rs
  -> writes break_count / continue_count
```

No classifier or lowerer reads them.

## Decision

Keep `carrier_count`; prune `break_count` and `continue_count`.

The live route booleans are already:

```text
has_break
has_continue
```

Count fields are stale diagnostic scaffolding and duplicate the route booleans
without a consumer.

## Next Cleanup

Prune:

```text
LoopFeatures.break_count
LoopFeatures.continue_count
producer assignments for both fields
test initializers for both fields
```

Preserve:

```text
LoopFeatures.carrier_count
LoopFeatures.has_break
LoopFeatures.has_continue
```

## Non-Goals

- No route classifier behavior change.
- No Case-A behavior change.
- No control-flow counter utilities outside `LoopFeatures`.

## Validation

```bash
rg -n "break_count|continue_count" src/mir/loop_route_detection src/mir/builder/control_flow/facts/ast_feature_extractor.rs -g '*.rs' || true
cargo test -q loop_update_
cargo check -q
bash tools/checks/current_state_pointer_guard.sh
git diff --check
```
