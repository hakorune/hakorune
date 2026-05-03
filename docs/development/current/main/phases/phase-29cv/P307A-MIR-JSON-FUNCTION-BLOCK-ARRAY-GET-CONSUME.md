---
Status: Active
Decision: accepted
Date: 2026-05-03
Scope: phase-29cv P307a, MIR JSON function block array get fact consume
Related:
  - docs/development/current/main/phases/phase-29cv/P207A-STAGE0-SIZE-GUARD.md
  - docs/development/current/main/phases/phase-29cv/P223A-MIR-JSON-FUNCTION-BLOCK-ARRAY-PROOF.md
  - docs/development/current/main/phases/phase-29cv/P306A-MIR-JSON-FUNCTION-FIELD-CONSUME.md
  - lang/src/shared/mir/json_emit_box.hako
  - lang/c-abi/shims/hako_llvmc_ffi_module_generic_string_method_views.inc
  - lang/c-abi/shims/hako_llvmc_ffi_module_generic_string_function_emit.inc
---

# P307a: MIR JSON Function Block Array Get Consume

## Problem

P306a advances the source-exe probe to:

```text
reason=module_generic_prepass_failed
target_shape_blocker_symbol=MirJsonEmitBox._emit_function_rec/3
target_shape_blocker_reason=-
```

`_emit_function_rec/3` already has an exact MIR-owned route:

```text
blocks.get(idx) -> proof=mir_json_function_block_array_item
route_kind=array_slot_load_any
helper=nyash.array.slot_load_hi
```

The module generic string emitter does not yet consume this exact ArrayGet row.

## Decision

Consume only the exact `mir_json_function_block_array_item` ArrayGet row in
the module generic string emitter.

## Non-Goals

- no generic `ArrayBox.get` widening
- no function-rec body-specific emitter
- no new `GlobalCallTargetShape`
- no MIR JSON function blocks schema change

## Acceptance

```bash
cargo test -q proves_mir_json_function_block_array_item_runtime_data_get --lib
bash tools/build_hako_llvmc_ffi.sh
cargo build -q --release --bin hakorune
NYASH_LLVM_ROUTE_TRACE=1 target/release/hakorune --emit-exe /tmp/hakorune_p307a_function_block_array.exe lang/src/runner/stage1_cli_env.hako
bash tools/checks/current_state_pointer_guard.sh
git diff --check
```

Expected probe result:

```text
_emit_function_rec/3 no longer fails module_generic_prepass_failed at blocks.get(idx)
```

Observed next blocker:

```text
reason=missing_multi_function_emitter
target_shape_blocker_symbol=LowerLoopCountParamBox._finish_count_param_text/5
target_shape_blocker_reason=generic_string_unsupported_void_sentinel_const
```
