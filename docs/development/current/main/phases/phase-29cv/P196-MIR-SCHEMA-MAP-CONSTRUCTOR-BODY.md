---
Status: Active
Decision: accepted
Date: 2026-05-02
Scope: phase-29cv P196, MIR schema MapBox constructor body shape
Related:
  - docs/development/current/main/phases/phase-29cv/P195-GENERIC-STRING-BOOL-NOT-FLOW.md
  - lang/src/shared/mir/mir_schema_box.hako
  - src/mir/global_call_route_plan/model.rs
  - src/mir/global_call_route_plan/mir_schema_map_constructor_body.rs
---

# P196: MIR Schema Map Constructor Body

## Problem

P195 moved the active source-execution blocker to:

```text
target_shape_blocker_symbol=MirSchemaBox.inst_const/2
target_shape_blocker_reason=generic_string_return_object_abi_not_handle_compatible
```

The target is not a string body. It returns a `MapBox` schema object:

```hako
local m = new MapBox()
m.set("op", "const")
m.set("dst", this.i(dst))
m.set("value", this.i(val))
return m
```

The same boundary appears across `MirSchemaBox.i/1`, `inst_ret/1`,
`inst_binop/4`, `block/2`, `module/1`, and related schema constructors.

## Decision

Introduce a dedicated shape:

```text
mir_schema_map_constructor_body
```

This shape returns a `map_handle`, not a string handle. Generic string/i64
classifiers may consume its result only as a MapBox value. The C shim must read
the lowering plan and mark direct-call results with MapBox origin; it must not
rediscover `MirSchemaBox` names from raw callee text.

## Boundary

The shape may observe:

- `new MapBox()`
- `MapBox.set` / `RuntimeDataBox.set`
- string/scalar constants
- child global calls that already return map/array/string/scalar handles
- returning the constructed `MapBox`

The shape must not:

- match `MirSchemaBox` by exact name
- make arbitrary object-return functions DirectAbi
- treat MapBox schema values as strings
- hide unsupported child constructors

## Acceptance

```bash
cargo test -q mir_schema_map_constructor --lib
cargo test -q global_call_routes --lib
cargo fmt --check
bash tools/build_hako_llvmc_ffi.sh
cargo build -q --release --bin hakorune
NYASH_LLVM_ROUTE_TRACE=1 target/release/hakorune --emit-exe /tmp/hakorune_p196_mir_schema_map_constructor_probe.exe lang/src/runner/stage1_cli_env.hako
bash tools/checks/current_state_pointer_guard.sh
git diff --check
```

The `--emit-exe` command remains a next-blocker probe.

## Implementation

- Added `GlobalCallTargetShape::MirSchemaMapConstructorBody`.
- Added `src/mir/global_call_route_plan/mir_schema_map_constructor_body.rs`.
- Kept non-matching `box<MapBox>` returns on the existing object ABI blocker
  path unless a child shape blocker must be propagated.
- Taught string/scalar classifiers that direct schema constructors return a
  MapBox object handle, not a string.
- Added LoweringPlan C shim validation for:

```text
proof=typed_global_call_mir_schema_map_constructor
target_shape=mir_schema_map_constructor_body
return_shape=map_handle
value_demand=runtime_i64_or_handle
```

## Probe Result

P196 removes the previous blocker:

```text
target_shape_blocker_symbol=MirSchemaBox.inst_const/2
target_shape_blocker_reason=generic_string_return_object_abi_not_handle_compatible
```

The stage1 `--emit-exe` probe now fails later at:

```text
target_shape_blocker_symbol=LowerReturnBoolBox.try_lower/1
target_shape_blocker_reason=generic_string_unsupported_void_sentinel_const
```

The generated MIR JSON contains `typed_global_call_mir_schema_map_constructor`
routes for `MirSchemaBox.inst_const/2` and `MirSchemaBox.i/1`.
