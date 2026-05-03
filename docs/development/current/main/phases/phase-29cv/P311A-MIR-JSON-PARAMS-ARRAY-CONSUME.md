---
Status: Active
Decision: accepted
Date: 2026-05-03
Scope: phase-29cv P311a, MIR JSON params array item fact consume
Related:
  - docs/development/current/main/phases/phase-29cv/P207A-STAGE0-SIZE-GUARD.md
  - docs/development/current/main/phases/phase-29cv/P224A-MIR-JSON-PARAMS-ARRAY-PROOF.md
  - docs/development/current/main/phases/phase-29cv/P310A-MIR-JSON-MODULE-FUNCTION-ARRAY-CONSUME.md
  - lang/src/shared/mir/json_emit_box.hako
  - lang/c-abi/shims/hako_llvmc_ffi_module_generic_string_method_views.inc
  - lang/c-abi/shims/hako_llvmc_ffi_module_generic_string_function_emit.inc
---

# P311a: MIR JSON Params Array Consume

## Problem

P310a advances the source-exe probe to:

```text
reason=module_generic_prepass_failed
target_shape_blocker_symbol=MirJsonEmitBox._emit_params_rec/3
target_shape_blocker_reason=-
```

`MirJsonEmitBox._emit_params_rec/3` already has exact MIR-owned
`generic_method.get` rows for parameter array item reads:

```text
route_proof=mir_json_params_array_item
route_kind=array_slot_load_any
symbol=nyash.array.slot_load_hi
```

The C module generic string emitter does not yet consume this exact ArrayGet
row.

## Decision

Consume only the exact `mir_json_params_array_item` ArrayGet row in the module
generic string emitter.

## Non-Goals

- no generic `ArrayBox.get` widening
- no params-rec body-specific emitter
- no new `GlobalCallTargetShape`
- no MIR params schema change

## Acceptance

```bash
cargo test -q proves_mir_json_params_array_item_runtime_data_get --lib
bash tools/build_hako_llvmc_ffi.sh
cargo build -q --release --bin hakorune
NYASH_LLVM_ROUTE_TRACE=1 target/release/hakorune --emit-exe /tmp/hakorune_p311a_params_array.exe lang/src/runner/stage1_cli_env.hako
bash tools/checks/current_state_pointer_guard.sh
git diff --check
```

Expected probe result:

```text
MirJsonEmitBox._emit_params_rec/3 no longer fails module_generic_prepass_failed
on params.get(...).
```

## Result

Accepted. The C module generic string emitter now consumes exact
`mir_json_params_array_item` ArrayGet rows and lowers them through the planned
`nyash.array.slot_load_hi` route.

Validation:

```bash
cargo test -q proves_mir_json_params_array_item_runtime_data_get --lib
bash tools/build_hako_llvmc_ffi.sh
cargo build -q --release --bin hakorune
NYASH_LLVM_ROUTE_TRACE=1 target/release/hakorune --emit-exe /tmp/hakorune_p311.exe lang/src/runner/stage1_cli_env.hako
```

The probe advanced past `MirJsonEmitBox._emit_params_rec/3` to:

```text
reason=module_generic_prepass_failed
target_shape_blocker_symbol=MirJsonEmitBox._emit_phi_incoming_rec/3
target_shape_blocker_reason=-
```
