---
Status: Active
Decision: accepted
Date: 2026-05-03
Scope: phase-29cv P203, MIR schema module-root map constructor body
Related:
  - docs/development/current/main/phases/phase-29cv/P196-MIR-SCHEMA-MAP-CONSTRUCTOR-BODY.md
  - docs/development/current/main/phases/phase-29cv/P200-MIR-SCHEMA-MAP-WRAPPER-BODY.md
  - docs/development/current/main/phases/phase-29cv/P202-PATTERN-UTIL-LOCAL-VALUE-PROBE-BODY.md
  - lang/src/shared/mir/mir_schema_box.hako
  - src/mir/global_call_route_plan/mir_schema_map_constructor_body.rs
---

# P203: MIR Schema Module Root Constructor

## Problem

P202 moved the active source-execution probe to:

```text
target_shape_blocker_symbol=MirSchemaBox.module/1
target_shape_blocker_reason=generic_string_return_object_abi_not_handle_compatible
```

`MirSchemaBox.module/1` is still a MIR schema map constructor. It builds the
root module map:

```hako
local m = new MapBox()
m.set("version", this.i(0))
m.set("kind", "MIR")
local funcs = [ fn_main ]
m.set("functions", funcs)
return m
```

The previous schema constructor facts required `ArrayBox.push` payloads to be
proven MapBox values. In this body, the pushed function map is the single
parameter and the source signature is currently unknown, so the existing proof
never reaches the classifier.

## Decision

Keep this in the existing shape:

```text
target_shape=mir_schema_map_constructor_body
proof=typed_global_call_mir_schema_map_constructor
return_shape=map_handle
value_demand=runtime_i64_or_handle
```

Do not introduce a new shape and do not add this to `generic_string_body.rs`.
The extension is limited to the module-root surface: `version`, `kind="MIR"`,
and `functions` array.

## Boundary

The classifier may additionally observe:

- unknown return signature
- local `MapBox` root constructor
- `version`, `kind`, `MIR`, and `functions` string markers
- local `ArrayBox` birth for `functions`
- `ArrayBox.push` of the function-map parameter
- `MapBox.set("functions", funcs)`
- return of the root map

The shape must not:

- accept arbitrary unknown-return MapBox helpers
- accept arbitrary `ArrayBox.push` of unknown payloads outside the module-root
  marker surface
- match `MirSchemaBox.module/1` by exact name
- treat the root map as a string handle

## Implementation

- Extended `mir_schema_map_constructor_body` candidate facts with a
  module-root marker surface.
- The marker surface requires `version`, `kind`, `MIR`, and `functions`
  constants before an unknown `ArrayBox.push` payload is allowed.
- The existing map-handle DirectAbi metadata remains unchanged:

```text
target_shape=mir_schema_map_constructor_body
proof=typed_global_call_mir_schema_map_constructor
return_shape=map_handle
value_demand=runtime_i64_or_handle
```

## Probe Result

P203 removes the previous blocker:

```text
target_shape_blocker_symbol=MirSchemaBox.module/1
target_shape_blocker_reason=generic_string_return_object_abi_not_handle_compatible
```

The source-execution probe now reaches:

```text
target_shape_blocker_symbol=LowerIfCompareFoldVarIntBox._fold_bin_varint/3
target_shape_blocker_reason=generic_string_unsupported_instruction
```

## Acceptance

```bash
cargo test -q mir_schema_module_root --lib
cargo test -q mir_schema_map_constructor --lib
cargo test -q global_call_routes --lib
cargo fmt --check
bash tools/build_hako_llvmc_ffi.sh
cargo build -q --release --bin hakorune
NYASH_LLVM_ROUTE_TRACE=1 target/release/hakorune --emit-exe /tmp/hakorune_p203_mir_schema_module_root.exe lang/src/runner/stage1_cli_env.hako
bash tools/checks/current_state_pointer_guard.sh
git diff --check
```

The `--emit-exe` command remains a next-blocker probe.
