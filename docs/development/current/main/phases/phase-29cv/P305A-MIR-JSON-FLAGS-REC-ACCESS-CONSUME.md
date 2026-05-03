---
Status: Active
Decision: accepted
Date: 2026-05-03
Scope: phase-29cv P305a, MIR JSON flags recursion access fact consume
Related:
  - docs/development/current/main/phases/phase-29cv/P207A-STAGE0-SIZE-GUARD.md
  - docs/development/current/main/phases/phase-29cv/P225A-MIR-JSON-FLAGS-REC-ACCESS-PROOF.md
  - docs/development/current/main/phases/phase-29cv/P304A-MIR-JSON-FLAGS-KEYS-LOWERING-PLAN-CONSUME.md
  - lang/src/shared/mir/json_emit_box.hako
  - lang/c-abi/shims/hako_llvmc_ffi_module_generic_string_method_views.inc
  - lang/c-abi/shims/hako_llvmc_ffi_module_generic_string_function_emit.inc
---

# P305a: MIR JSON Flags Rec Access Consume

## Problem

P304a advances the source-exe probe to:

```text
reason=module_generic_prepass_failed
target_shape_blocker_symbol=MirJsonEmitBox._emit_flags_rec/4
target_shape_blocker_reason=-
```

`_emit_flags_rec/4` already has exact MIR-owned route facts:

```text
keys.get(idx) -> proof=mir_json_flags_rec_access route_kind=array_slot_load_any
flags.get(k) -> proof=mir_json_flags_rec_access route_kind=runtime_data_load_any
```

The module generic string emitter currently consumes only the older generic
surface get rows plus later MIR JSON schema field rows. It does not consume
these exact flags-recursion access rows, so prepass cannot type the function.

## Decision

Consume only the exact `mir_json_flags_rec_access` rows in the module generic
string emitter:

```text
generic_method.get + ArrayGet + array_slot_load_any + mir_json_flags_rec_access
generic_method.get + MapGet + runtime_data_load_any + mir_json_flags_rec_access
```

Lowering stays helper-based:

```text
ArrayGet -> nyash.array.slot_load_hi
MapGet   -> nyash.runtime_data.get_hh
```

## Non-Goals

- no generic `RuntimeDataBox.get` widening
- no generic map iteration semantics
- no new `GlobalCallTargetShape`
- no ny-llvmc body-specific emitter
- no changes to `MirJsonEmitBox._emit_flags_rec/4` source shape

## Acceptance

```bash
cargo test -q proves_mir_json_flags_rec_access_runtime_data_get --lib
bash tools/build_hako_llvmc_ffi.sh
cargo build -q --release --bin hakorune
NYASH_LLVM_ROUTE_TRACE=1 target/release/hakorune --emit-exe /tmp/hakorune_p305a_flags_rec.exe lang/src/runner/stage1_cli_env.hako
bash tools/checks/current_state_pointer_guard.sh
git diff --check
```

Expected probe result:

```text
_emit_flags_rec/4 no longer fails module_generic_prepass_failed at exact get sites
```

Observed next blocker:

```text
reason=module_generic_prepass_failed
target_shape_blocker_symbol=MirJsonEmitBox._emit_function/1
target_shape_blocker_reason=-
```
