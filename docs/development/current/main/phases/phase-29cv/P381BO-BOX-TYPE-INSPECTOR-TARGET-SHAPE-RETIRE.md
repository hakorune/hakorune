---
Status: Accepted
Decision: accepted
Date: 2026-05-05
Scope: phase-29cv P381BO, BoxTypeInspector target-shape retirement
Related:
  - docs/development/current/main/design/stage0-llvm-line-shape-inventory-ssot.md
  - docs/development/current/main/phases/phase-29cv/P201-BOX-TYPE-INSPECTOR-DESCRIBE-BODY.md
  - docs/development/current/main/phases/phase-29cv/P207C-BOX-TYPE-INSPECTOR-IS-MAP-DIRECT-SCALAR.md
  - docs/development/current/main/phases/phase-29cv/P207F-BOX-TYPE-INSPECTOR-IS-ARRAY-DIRECT-SCALAR.md
  - src/mir/global_call_route_plan.rs
  - lang/c-abi/shims/hako_llvmc_ffi_lowering_plan_metadata.inc
---

# P381BO: BoxTypeInspector Target Shape Retire

## Problem

`BoxTypeInspectorDescribeBody` still existed as a `GlobalCallTargetShape`
variant even though the direct ABI route already has a MIR-owned proof and
return contract:

```text
proof=typed_global_call_box_type_inspector_describe
return_shape=map_handle
value_demand=runtime_i64_or_handle
```

Earlier cleanup moved active source-owner consumers to scalar predicates
(`BoxTypeInspectorBox.is_map/1` and `BoxTypeInspectorBox.is_array/1`), so Stage0
no longer needs a public target-shape name to identify active source-owner
traffic. Keeping the shape made the inventory overstate the remaining capsule
surface.

## Decision

Retire `BoxTypeInspectorDescribeBody` as a `GlobalCallTargetShape`.

The route still accepts the exact BoxTypeInspector describe body, but stores the
ABI truth as direct contract facts:

```text
target_shape=null
proof=typed_global_call_box_type_inspector_describe
return_shape=map_handle
value_demand=runtime_i64_or_handle
```

C lowering predicates must consume proof/return facts and must not require the
legacy target-shape string.

This card does not delete the dedicated body emitter. That belongs to the later
uniform multi-function emitter cleanup.

## Boundary

Allowed:

- remove the `GlobalCallTargetShape::BoxTypeInspectorDescribeBody` variant
- keep the existing body recognizer as the proof producer
- keep `ORG_MAP_BIRTH` behavior attached to the direct map-handle contract

Not allowed:

- merge BoxTypeInspector describe with MIR-schema map construction
- reintroduce source-owner matching in Stage0 C metadata predicates
- delete the body emitter before uniform multi-function MIR emission owns the
  path

## Evidence

Current source-owner search shows active `.hako` uses route through scalar
predicates rather than `_describe/describe/kind` direct calls:

```bash
rg -n "BoxTypeInspectorBox\\._describe|BoxTypeInspectorBox\\.kind|BoxTypeInspectorBox\\.describe" lang apps tools
```

The Rust route tests now assert a null target shape while preserving the
BoxTypeInspector proof and `map_handle` return contract.

## Acceptance

```bash
cargo test --release box_type_inspector_describe -- --nocapture
cargo test --release global_call_route_plan -- --nocapture
bash tools/checks/stage0_shape_inventory_guard.sh
bash tools/checks/current_state_pointer_guard.sh
git diff --check
```
