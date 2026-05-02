---
Status: Active
Decision: accepted
Date: 2026-05-03
Scope: phase-29cv P207g, MIR JSON emitter box-name split from generic JSON stringify facade
Related:
  - docs/development/current/main/phases/phase-29cv/P207A-STAGE0-SIZE-GUARD.md
  - docs/development/current/main/phases/phase-29cv/P207F-BOX-TYPE-INSPECTOR-IS-ARRAY-DIRECT-SCALAR.md
  - docs/development/current/main/CURRENT_STATE.toml
  - lang/src/shared/mir/json_emit_box.hako
  - lang/src/compiler/emit/common/json_emit_box.hako
---

# P207g: MIR JSON Emitter Name Split

## Problem

P207f moved the sibling `BoxTypeInspectorBox.is_array/1` wrapper to the scalar
lane. The remaining probe blocker is now propagated through the JSON emit
chain:

```text
JsonEmitBox.to_json/1 -> JSON.stringify/1 -> JSON.stringify_map/1
target_shape_blocker_symbol=BoxTypeInspectorBox.is_map/1
target_shape_blocker_reason=generic_string_global_target_shape_unknown
```

The direct `BoxTypeInspectorBox.is_map/1` and `BoxHelpers.is_map/1` calls are
already `DirectAbi generic_i64_body`. The problem is not the predicate itself.

The source tree has two different static boxes named `JsonEmitBox`:

```text
lang/src/shared/mir/json_emit_box.hako          # MIR(JSON) schema emitter
lang/src/compiler/emit/common/json_emit_box.hako # generic JSON.stringify facade
```

Route inventory shows one merged `JsonEmitBox.to_json/1` symbol whose body is
the generic `JSON.stringify/1` facade, while MIR-specific helper methods from
`lang/src/shared/mir/json_emit_box.hako` also appear under the same box name.
That name collision sends MIR module emitters toward general JSON stringify.

If we respond by teaching Stage0 `generic_string_body` or the C shim to execute
recursive JSON stringify over MapBox/ArrayBox, Stage0 becomes a JSON semantic
clone. That violates the P207a size guard.

## Decision

Split the MIR emitter name:

```text
generic JSON facade: JsonEmitBox
MIR(JSON) schema emitter: MirJsonEmitBox
```

MIR-specific callers must import/call `MirJsonEmitBox.to_json(...)`. The
generic `JsonEmitBox` facade remains available for true arbitrary JSON
stringify use.

This is a BoxShape cleanup. It changes the route owner for MIR module emission;
it must not add a new acceptance shape to `generic_string_body` or ny-llvmc.

## Boundary

This card may:

- rename `lang/src/shared/mir/json_emit_box.hako`'s static box to
  `MirJsonEmitBox`
- update MIR-specific users of `selfhost.shared.mir.json_emit` to call
  `MirJsonEmitBox`
- keep the generic `lang/src/compiler/emit/common/json_emit_box.hako`
  `JsonEmitBox` facade unchanged

It must not:

- add `JSON.stringify` / MapBox / ArrayBox body semantics to Stage0
- add C shim JSON stringify lowering
- change the generic JSON facade contract
- change MIR(JSON) output schema intentionally
- delete compat emitters in the same card

## Probe Contract

Before this card, route inventory shows MIR emit helpers reaching the generic
facade:

```text
IfMirEmitBox._module_json/1 -> JsonEmitBox.to_json/1
JsonEmitBox.to_json/1 -> JSON.stringify/1
```

After this card, MIR-specific calls should target:

```text
IfMirEmitBox._module_json/1 -> MirJsonEmitBox.to_json/1
```

The source-exe probe may still stop on a later MIR emitter/helper blocker, but
it should not require adding arbitrary JSON stringify semantics to Stage0 to
pass this seam.

## Probe Result

After this card, MIR emitter callers target the split owner:

```text
IfMirEmitBox._module_json/1 -> MirJsonEmitBox.to_json/1
CompatMirEmitBox._module_with_blocks/1 -> MirJsonEmitBox.to_json/1
```

The old transitive blocker is gone from the refreshed route inventory:

```text
target_shape_blocker_symbol=BoxTypeInspectorBox.is_map/1
```

The source-exe probe now stops later at:

```text
target_shape_blocker_symbol=LowerIfCompareFoldVarIntBox._resolve_side/3
target_shape_blocker_reason=generic_string_return_not_string
backend_reason=missing_multi_function_emitter
```

This confirms P207g removed the generic `JSON.stringify` seam without adding
MapBox/ArrayBox JSON semantics to Stage0. The next card should inventory
`LowerIfCompareFoldVarIntBox._resolve_side/3` as a source-flow return-shape
issue before considering any MIR/backend expansion.

## Acceptance

```bash
cargo test -q global_call_routes --lib
cargo fmt --check
bash tools/build_hako_llvmc_ffi.sh
cargo build -q --release --bin hakorune
NYASH_LLVM_ROUTE_TRACE=1 target/release/hakorune --emit-exe /tmp/hakorune_p207g_mir_json_emitter_name_split.exe lang/src/runner/stage1_cli_env.hako
bash tools/checks/current_state_pointer_guard.sh
git diff --check
```
