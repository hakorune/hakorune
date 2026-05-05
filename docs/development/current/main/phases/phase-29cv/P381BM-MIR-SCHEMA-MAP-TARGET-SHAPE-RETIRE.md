---
Status: Done
Decision: accepted
Date: 2026-05-05
Scope: retire `MirSchemaMapConstructorBody` as a `GlobalCallTargetShape` while keeping its proof/return/origin contract.
Related:
  - docs/development/current/main/design/stage0-llvm-line-shape-inventory-ssot.md
  - docs/development/current/main/design/lowering-plan-json-v0-ssot.md
  - docs/development/current/main/phases/phase-29cv/P381BC-STAGE0-CAPSULE-EXIT-TASK-MAP.md
  - src/mir/global_call_route_plan/model.rs
---

# P381BM: MIR Schema Map Target-Shape Retire

## Problem

`MirSchemaMapConstructorBody` still existed as a target-shape variant even
though the route already had a narrow MIR-owned contract:

```text
proof=typed_global_call_mir_schema_map_constructor
return_shape=map_handle
value_demand=runtime_i64_or_handle
```

That left a temporary Stage0 shape carrying map-origin semantics that can be
expressed by the existing proof/return contract.

## Decision

Retire `MirSchemaMapConstructorBody` as a `GlobalCallTargetShape`.

MIR schema map constructor targets now publish:

```text
target_shape=null
proof=typed_global_call_mir_schema_map_constructor
return_shape=map_handle
value_demand=runtime_i64_or_handle
```

The C-side direct-call predicate reads proof/return facts instead of a shape
string, so `ORG_MAP_BIRTH` propagation remains tied to the LoweringPlan
contract.

## Boundary

Allowed:

- keep the existing MIR schema body recognizer as the source of this narrow
  MIR-owned contract
- keep selected-set/body emission using the generic module symbol path
- update route JSON tests to assert `target_shape=null`

Not allowed:

- add a replacement target-shape variant
- infer MIR schema map semantics from a source owner name in Stage0
- weaken `map_handle` / origin propagation checks
- merge this with `BoxTypeInspectorDescribeBody`, which intentionally remains
  a separate source-owner capsule

## Acceptance

```bash
cargo test --release mir_schema_map_constructor -- --nocapture
cargo test --release global_call_route_plan -- --nocapture
bash tools/checks/stage0_shape_inventory_guard.sh
bash tools/checks/current_state_pointer_guard.sh
git diff --check
```

## Result

Done:

- `MirSchemaMapConstructorBody` removed from `GlobalCallTargetShape`
- MIR schema map classification now uses
  `direct_contract(MirSchemaMapConstructor, MapHandle)`
- C MIR-schema map global-call predicate no longer requires `target_shape`
- MIR route JSON tests now expect `target_shape=null`
