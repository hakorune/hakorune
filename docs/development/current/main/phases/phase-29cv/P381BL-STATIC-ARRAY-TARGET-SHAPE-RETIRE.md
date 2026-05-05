---
Status: Done
Decision: accepted
Date: 2026-05-05
Scope: retire `StaticStringArrayBody` as a `GlobalCallTargetShape` while keeping its proof/return/origin contract.
Related:
  - docs/development/current/main/design/stage0-llvm-line-shape-inventory-ssot.md
  - docs/development/current/main/design/lowering-plan-json-v0-ssot.md
  - docs/development/current/main/phases/phase-29cv/P381BK-STATIC-ARRAY-CONSUMER-CONTRACT-READ.md
  - src/mir/global_call_route_plan/model.rs
---

# P381BL: Static Array Target-Shape Retire

## Problem

`StaticStringArrayBody` still existed as a target-shape variant even after
P381BK moved the Rust generic-method consumer to read the stored
`proof=typed_global_call_static_string_array` and `return_shape=array_handle`
contract.

That left one more temporary Stage0 shape for a contract that was already
explicit in MIR metadata.

## Decision

Retire `StaticStringArrayBody` as a `GlobalCallTargetShape`.

Static string array targets now publish:

```text
target_shape=null
proof=typed_global_call_static_string_array
return_shape=array_handle
value_demand=runtime_i64_or_handle
```

The C-side direct-call predicate also reads proof/return facts instead of a
shape string, so `ORG_ARRAY_STRING_BIRTH` propagation remains tied to the
LoweringPlan contract.

## Boundary

Allowed:

- keep the existing static-array body recognizer as the source of this narrow
  MIR-owned contract
- keep selected-set/body emission using the generic module symbol path
- update route JSON tests to assert `target_shape=null`

Not allowed:

- add a replacement target-shape variant
- infer static-array semantics from a source owner name in Stage0
- weaken `array_handle` / origin propagation checks

## Acceptance

```bash
cargo test --release static_string_array -- --nocapture
cargo test --release build_mir_json_root_emits_direct_plan_for_static_string_array_contract -- --nocapture
bash tools/checks/stage0_shape_inventory_guard.sh
bash tools/checks/current_state_pointer_guard.sh
git diff --check
```

## Result

Done:

- `StaticStringArrayBody` removed from `GlobalCallTargetShape`
- static array classification now uses `direct_contract(StaticStringArray, ArrayHandle)`
- C static-array global-call predicate no longer requires `target_shape`
- MIR route JSON and LoweringPlan JSON tests now expect `target_shape=null`
