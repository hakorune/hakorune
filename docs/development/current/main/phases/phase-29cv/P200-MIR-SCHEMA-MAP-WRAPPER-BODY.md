---
Status: Active
Decision: accepted
Date: 2026-05-02
Scope: phase-29cv P200, MIR schema map wrapper bodies with unknown return signatures
Related:
  - docs/development/current/main/phases/phase-29cv/P196-MIR-SCHEMA-MAP-CONSTRUCTOR-BODY.md
  - docs/development/current/main/phases/phase-29cv/P199-GENERIC-STRING-COLLECTION-BUILDER-SURFACES.md
  - lang/src/mir/builder/internal/if_mir_emit_box.hako
  - src/mir/global_call_route_plan/mir_schema_map_constructor_body.rs
---

# P200: MIR Schema Map Wrapper Body

## Problem

P199 moved the active source-execution probe to:

```text
target_shape_blocker_symbol=IfMirEmitBox._ret_block/3
target_shape_blocker_reason=generic_string_return_object_abi_not_handle_compatible
```

`IfMirEmitBox._ret_block/3` is not a string body. It builds an `ArrayBox` of
MIR schema instruction maps, then returns the result of a direct
`mir_schema_map_constructor_body` child:

```hako
local insts = new ArrayBox()
insts.push(MirSchemaBox.inst_const(dst, value))
insts.push(MirSchemaBox.inst_ret(dst))
return MirSchemaBox.block(id, insts)
```

The source signature is currently unknown (`target_return_type=?`), so P196's
`box<MapBox>` entry condition never reaches the structural classifier.

## Decision

Keep this in the existing dedicated object shape:

```text
target_shape=mir_schema_map_constructor_body
proof=typed_global_call_mir_schema_map_constructor
return_shape=map_handle
```

Do not add it to `generic_string_body.rs`, and do not match
`IfMirEmitBox._ret_block/3` by name.

## Boundary

The existing local map constructor remains accepted:

- `new MapBox()`
- `MapBox.set` / `RuntimeDataBox.set`
- return of the constructed map

Additionally accept a narrow wrapper:

- unknown return signature
- `ArrayBox` instruction-list birth
- `ArrayBox.push` of proven map schema values
- return of a proven `mir_schema_map_constructor_body` global result that
  consumes that array

The wrapper must not:

- treat arbitrary unknown-return helpers as map constructors
- accept string-return wrappers through this shape
- use callee-name exceptions for `IfMirEmitBox`
- hide unsupported child schema constructors

## Implementation

- `mir_schema_map_constructor_body.rs` now has a candidate gate for unknown
  return signatures.
- The unknown-return candidate requires an `ArrayBox` instruction list, pushes
  of proven map schema values, and return of a map constructor child that
  consumes that array.
- The existing map constructor facts accept either the original local
  `MapBox.set` constructor or the new array-wrapped map-return constructor.
- `generic_string_body.rs` remains unchanged.

## Probe Result

P200 removes the previous blocker:

```text
target_shape_blocker_symbol=IfMirEmitBox._ret_block/3
target_shape_blocker_reason=generic_string_return_object_abi_not_handle_compatible
```

The source-execution probe now reaches:

```text
target_shape_blocker_symbol=BoxTypeInspectorBox._describe/1
target_shape_blocker_reason=generic_string_return_object_abi_not_handle_compatible
```

## Acceptance

```bash
cargo test -q mir_schema_map_wrapper --lib
cargo test -q mir_schema_map_constructor --lib
cargo test -q global_call_routes --lib
cargo fmt --check
bash tools/build_hako_llvmc_ffi.sh
cargo build -q --release --bin hakorune
NYASH_LLVM_ROUTE_TRACE=1 target/release/hakorune --emit-exe /tmp/hakorune_p200_mir_schema_map_wrapper_probe.exe lang/src/runner/stage1_cli_env.hako
bash tools/checks/current_state_pointer_guard.sh
git diff --check
```

The `--emit-exe` command remains a next-blocker probe.
