---
Status: Active
Decision: accepted
Date: 2026-05-03
Scope: phase-29cv P310a, MIR JSON module function array item fact consume
Related:
  - docs/development/current/main/phases/phase-29cv/P207A-STAGE0-SIZE-GUARD.md
  - docs/development/current/main/phases/phase-29cv/P230A-MIR-JSON-MODULE-FUNCTION-ARRAY-PROOF.md
  - docs/development/current/main/phases/phase-29cv/P309A-MIR-JSON-INST-FIELD-CONSUME.md
  - lang/src/shared/mir/json_emit_box.hako
  - lang/c-abi/shims/hako_llvmc_ffi_module_generic_string_method_views.inc
  - lang/c-abi/shims/hako_llvmc_ffi_module_generic_string_function_emit.inc
---

# P310a: MIR JSON Module Function Array Consume

## Problem

P309a advances the source-exe probe to:

```text
reason=module_generic_prepass_failed
target_shape_blocker_symbol=MirJsonEmitBox._emit_module_rec/3
target_shape_blocker_reason=-
```

`MirJsonEmitBox._emit_module_rec/3` already has exact MIR-owned
`generic_method.get` rows for `functions.get(...)`:

```text
route_proof=mir_json_module_function_array_item
route_kind=array_slot_load_any
symbol=nyash.array.slot_load_hi
```

The C module generic string emitter does not yet consume this exact ArrayGet
row.

## Decision

Consume only the exact `mir_json_module_function_array_item` ArrayGet row in
the module generic string emitter.

## Non-Goals

- no generic `ArrayBox.get` widening
- no module-rec body-specific emitter
- no new `GlobalCallTargetShape`
- no MIR module schema change

## Acceptance

```bash
cargo test -q mir_json_module_function_array_item --lib
bash tools/build_hako_llvmc_ffi.sh
cargo build -q --release --bin hakorune
NYASH_LLVM_ROUTE_TRACE=1 target/release/hakorune --emit-exe /tmp/hakorune_p310a_module_function_array.exe lang/src/runner/stage1_cli_env.hako
bash tools/checks/current_state_pointer_guard.sh
git diff --check
```

Expected probe result:

```text
MirJsonEmitBox._emit_module_rec/3 no longer fails module_generic_prepass_failed
on functions.get(...).
```

## Result

Accepted.

The C module generic string emitter now consumes the exact
`mir_json_module_function_array_item` ArrayGet rows. `_emit_module_rec/3` no
longer fails module generic prepass on `functions.get(...)`.

The probe advances to the next module generic prepass blocker:

```text
reason=module_generic_prepass_failed
target_shape_blocker_symbol=MirJsonEmitBox._emit_params_rec/3
target_shape_blocker_reason=-
```
