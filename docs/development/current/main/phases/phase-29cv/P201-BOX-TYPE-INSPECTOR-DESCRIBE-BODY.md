---
Status: Active
Decision: accepted
Date: 2026-05-03
Scope: phase-29cv P201, Box type inspector describe map body shape
Related:
  - docs/development/current/main/phases/phase-29cv/P200-MIR-SCHEMA-MAP-WRAPPER-BODY.md
  - lang/src/shared/common/box_type_inspector_box.hako
  - src/mir/global_call_route_plan/model.rs
---

# P201: Box Type Inspector Describe Body

## Problem

P200 moved the active source-execution probe to:

```text
target_shape_blocker_symbol=BoxTypeInspectorBox._describe/1
target_shape_blocker_reason=generic_string_return_object_abi_not_handle_compatible
```

`_describe/1` is not a string body and is not a MIR schema constructor. It
returns a metadata `MapBox`:

```text
kind -> "Unknown" | "Null" | "MapBox" | "ArrayBox" | "StringBox" | "IntegerBox"
is_map -> 0 | 1
is_array -> 0 | 1
```

The body observes string renderings through `"" + value`, `indexOf`, scalar
flags, `MapBox.set`, optional `env.get`, and optional trace `print`.

## Decision

Introduce a dedicated shape:

```text
target_shape=box_type_inspector_describe_body
proof=typed_global_call_box_type_inspector_describe
return_shape=map_handle
value_demand=runtime_i64_or_handle
```

This is an object-handle DirectAbi body. It must not be added to
`generic_string_body.rs` or `mir_schema_map_constructor_body.rs`.

## Boundary

The classifier may observe:

- one parameter
- one or more local `MapBox` births
- string constants for metadata keys and box-kind probes
- string concat from `"" + value`
- string `indexOf` method facts
- scalar/bool compare and PHI flow
- `MapBox.set` of the metadata keys
- `env.get` and `print` for the existing trace branch
- return of the metadata map

The shape must not:

- match `BoxTypeInspectorBox` by name
- accept arbitrary MapBox-return helpers
- accept MIR schema constructors
- treat the returned map as a string handle

## Implementation

- Added `GlobalCallTargetShape::BoxTypeInspectorDescribeBody`.
- Added `src/mir/global_call_route_plan/box_type_inspector_describe_body.rs`.
- The candidate gate requires the metadata-map markers before entering the
  classifier, so generic unknown-return string/void wrappers keep their
  existing classifiers.
- Taught string/scalar/map consumers that this shape returns a MapBox handle.
- Added LoweringPlan C shim validation for:

```text
proof=typed_global_call_box_type_inspector_describe
target_shape=box_type_inspector_describe_body
return_shape=map_handle
value_demand=runtime_i64_or_handle
```

## Probe Result

P201 removes the previous blocker:

```text
target_shape_blocker_symbol=BoxTypeInspectorBox._describe/1
target_shape_blocker_reason=generic_string_return_object_abi_not_handle_compatible
```

The source-execution probe now reaches:

```text
target_shape_blocker_symbol=PatternUtilBox.find_local_bool_before/3
target_shape_blocker_reason=generic_string_return_abi_not_handle_compatible
```

## Acceptance

```bash
cargo test -q box_type_inspector_describe --lib
cargo test -q global_call_routes --lib
cargo fmt --check
bash tools/build_hako_llvmc_ffi.sh
cargo build -q --release --bin hakorune
NYASH_LLVM_ROUTE_TRACE=1 target/release/hakorune --emit-exe /tmp/hakorune_p201_box_type_inspector_describe_probe.exe lang/src/runner/stage1_cli_env.hako
bash tools/checks/current_state_pointer_guard.sh
git diff --check
```

The `--emit-exe` command remains a next-blocker probe.
