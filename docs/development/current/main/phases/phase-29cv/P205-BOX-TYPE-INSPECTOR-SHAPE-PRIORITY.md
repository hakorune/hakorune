---
Status: Active
Decision: accepted
Date: 2026-05-03
Scope: phase-29cv P205, BoxTypeInspector shape priority before broad MapBox schema candidate
Related:
  - docs/development/current/main/phases/phase-29cv/P196-MIR-SCHEMA-MAP-CONSTRUCTOR-BODY.md
  - docs/development/current/main/phases/phase-29cv/P201-BOX-TYPE-INSPECTOR-DESCRIBE-BODY.md
  - docs/development/current/main/phases/phase-29cv/P204-LOWER-IF-COMPARE-FOLD-VARINT-EXPLICIT-I64-COERCE.md
  - src/mir/global_call_route_plan.rs
  - src/mir/global_call_route_plan/box_type_inspector_describe_body.rs
  - src/mir/global_call_route_plan/mir_schema_map_constructor_body.rs
---

# P205: BoxTypeInspector Shape Priority

## Problem

P204 moved the active source-execution probe to:

```text
target_shape_blocker_symbol=BoxTypeInspectorBox._describe/1
target_shape_blocker_reason=generic_string_return_object_abi_not_handle_compatible
```

P201 already introduced the dedicated shape:

```text
target_shape=box_type_inspector_describe_body
proof=typed_global_call_box_type_inspector_describe
return_shape=map_handle
```

The current blocker is an ordering bug, not a missing body shape. The real
`BoxTypeInspectorBox._describe/1` currently has a concrete `box<MapBox>` return
type. The broad MIR schema map constructor candidate runs before the
BoxTypeInspector candidate and treats any concrete `MapBox` return as a schema
candidate entry. Non-schema metadata maps then fail before the more specific
BoxTypeInspector marker gate can run.

## Decision

Evaluate the more specific BoxTypeInspector metadata-map shape before the broad
MIR schema map constructor shape.

This preserves the existing shape vocabulary:

```text
BoxTypeInspector metadata map -> box_type_inspector_describe_body
MIR schema maps               -> mir_schema_map_constructor_body
```

Do not add this blocker to `generic_string_body.rs`, do not introduce a new
wrapper shape, and do not weaken MIR schema map constructor facts.

## Boundary

This card may change only route-shape priority and the regression test that
locks it.

The implementation must not:

- add a new target shape
- match `BoxTypeInspectorBox._describe/1` by exact name
- accept arbitrary `MapBox` returns as BoxTypeInspector metadata maps
- change `mir_schema_map_constructor_body` acceptance facts
- change C shim emitters

## Implementation

- Move the `is_box_type_inspector_describe_body_candidate` classification before
  `is_mir_schema_map_constructor_body_candidate`.
- Add a regression test where the BoxTypeInspector body has a concrete
  `MirType::Box("MapBox")` return type, proving it is not intercepted by the
  broader MIR schema map constructor candidate.

## Probe Result

P205 removes the previous priority blocker:

```text
target_shape_blocker_symbol=BoxTypeInspectorBox._describe/1
target_shape_blocker_reason=generic_string_return_object_abi_not_handle_compatible
```

The source-execution probe now reaches the same body with the more precise
classifier-local reason:

```text
target_shape_blocker_symbol=BoxTypeInspectorBox._describe/1
target_shape_blocker_reason=generic_string_unsupported_method_call
```

This confirms the dedicated BoxTypeInspector shape is now reached before the
broad MIR schema MapBox candidate. The remaining blocker is inside
`box_type_inspector_describe_body` value-flow classification.

## Acceptance

```bash
cargo test -q box_type_inspector_describe --lib
cargo test -q mir_schema_map_constructor --lib
cargo test -q global_call_routes --lib
cargo fmt --check
bash tools/build_hako_llvmc_ffi.sh
cargo build -q --release --bin hakorune
NYASH_LLVM_ROUTE_TRACE=1 target/release/hakorune --emit-exe /tmp/hakorune_p205_box_type_inspector_shape_priority.exe lang/src/runner/stage1_cli_env.hako
bash tools/checks/current_state_pointer_guard.sh
git diff --check
```

The `--emit-exe` command remains a next-blocker probe.
