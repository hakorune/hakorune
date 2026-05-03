---
Status: Active
Decision: accepted
Date: 2026-05-03
Scope: phase-29cv P314a, MIR JSON value-id array item fact consume
Related:
  - docs/development/current/main/phases/phase-29cv/P207A-STAGE0-SIZE-GUARD.md
  - docs/development/current/main/phases/phase-29cv/P217A-MIR-JSON-VID-ARRAY-ITEM-PROOF.md
  - docs/development/current/main/phases/phase-29cv/P313A-COUNT-PARAM-NORM-PAIR-PROJECTION.md
  - lang/src/shared/mir/json_emit_box.hako
  - lang/c-abi/shims/hako_llvmc_ffi_module_generic_string_method_views.inc
  - lang/c-abi/shims/hako_llvmc_ffi_module_generic_string_function_emit.inc
---

# P314a: MIR JSON VID Array Consume

## Problem

P313a advances the source-exe probe to:

```text
reason=module_generic_prepass_failed
target_shape_blocker_symbol=MirJsonEmitBox._emit_vid_array_rec/3
target_shape_blocker_reason=-
```

`MirJsonEmitBox._emit_vid_array_rec/3` already has exact MIR-owned
`generic_method.get` rows from P217a:

```text
proof = mir_json_vid_array_item
route_kind = array_slot_load_any
return_shape = scalar_i64_or_missing_zero
value_demand = scalar_i64
```

The C module generic string emitter does not yet consume this exact ArrayGet
row.

## Decision

Consume only the exact `mir_json_vid_array_item` ArrayGet row in the module
generic string emitter.

## Non-Goals

- no generic `ArrayBox.get` widening
- no VID array body-specific emitter
- no new `GlobalCallTargetShape`
- no MIR call-args schema change

## Acceptance

```bash
cargo test -q proves_mir_json_vid_array_item_runtime_data_get --lib
bash tools/build_hako_llvmc_ffi.sh
cargo build -q --release --bin hakorune
NYASH_LLVM_ROUTE_TRACE=1 target/release/hakorune --emit-exe /tmp/hakorune_p314a_vid_array.exe lang/src/runner/stage1_cli_env.hako
bash tools/checks/current_state_pointer_guard.sh
git diff --check
```

Expected probe result:

```text
MirJsonEmitBox._emit_vid_array_rec/3 no longer fails module_generic_prepass_failed
on arr.get(idx).
```

## Result

Accepted. The C module generic string emitter now consumes exact
`mir_json_vid_array_item` ArrayGet rows through the planned
`nyash.array.slot_load_hi` route.

Validation:

```bash
cargo test -q proves_mir_json_vid_array_item_runtime_data_get --lib
bash tools/build_hako_llvmc_ffi.sh
cargo build -q --release --bin hakorune
NYASH_LLVM_ROUTE_TRACE=1 target/release/hakorune --emit-exe /tmp/hakorune_p314.exe lang/src/runner/stage1_cli_env.hako
```

The probe advanced past `MirJsonEmitBox._emit_vid_array_rec/3` to:

```text
reason=module_generic_prepass_failed
target_shape_blocker_symbol=MirJsonEmitBox._expect_i64/2
target_shape_blocker_reason=-
```
