# 296x-1654: Boxed Sum Const Payload Definition Index

Status: Complete
Date: 2026-06-24
Token: BOXED-SUM-CONST-PAYLOAD-DEF-INDEX-001

## Decision

Move boxed-sum I64 payload constant recovery out of the emit layer.

`hako_llvmc_ffi_pure_compile_boxed_sum_emit.inc` currently emits correct code,
but `emit_boxed_sum_i64_variant_make()` still scans earlier instructions to
rediscover a payload's defining `const`. That is `definition_discovery` inside
the backend emitter.

Worker audit note:

```text
correctness:
  current linear scan is not known-wrong

structural problem:
  emit layer is rediscovering definition facts
  instead of consuming a named owner

cleanup direction:
  ValueId -> definition/const facts are owned by compiler state or a
  published lowering/prepass fact table; boxed-sum emit only consumes them.
```

This card intentionally fixes the local definition lookup first. Broader
`.inc` responsibility drain is tracked separately so this slice does not turn
into a mixed cleanup.

## Scope

```text
current file:
  lang/c-abi/shims/hako_llvmc_ffi_pure_compile_boxed_sum_emit.inc

allowed supporting files:
  lang/c-abi/shims/hako_llvmc_ffi_compiler_state.inc
  lang/c-abi/shims/hako_llvmc_ffi_pure_compile_generic_lowering_prescan.inc
  lang/c-abi/shims/hako_llvmc_ffi_same_module_prepass.inc

expected change:
  const / definition information is available from a named ValueId owner.
  boxed-sum emit code consumes that owner only.
```

## Non-Claims

```text
new boxed payload class = 0
new canonical MIR instruction = 0
new route descriptor = 0
boxed-sum lowering facade = 0
same-module fusion cleanup = 0
runtime fallback = 0
```

## Acceptance

```text
emit_boxed_sum_i64_variant_make contains no prior-instruction scan
const payload lookup is O(1) from a named owner
Some(-1) vs handle(-1) remains protected by value-class facts, not sign
missing const/value fact fails closed with a stable reason
unit / handle / I64 boxed-sum probes stay EXE/AOT green
metadata_context_region_parent_backend=green
rust_mirbuilder_converter_matrix_guard=green
runtime_try_hako_then_rust_fallback=0
```

## Parked

```text
BOXED-SUM-LOWERING-FACADE-001
C-SHIM-PREPASS-FACT-OWNER-DRAIN-001
SAME-MODULE-FUSION-PLAN-SSOT-001
```

## Closeout

```text
emit_boxed_sum_i64_variant_make_prior_instruction_scan=0
const_payload_lookup_owner=compiler_state_ValueId_const_table
generic_variant_make_callsite=updated
same_module_variant_make_callsite=updated
cargo_check=green
cargo_test_boxed_sum_site=green
metadata_context_region_parent_guard=green
rust_mirbuilder_converter_matrix_guard=green
current_state_pointer_guard=green
runtime_try_hako_then_rust_fallback=0
```

## Follow-Up Split

```text
next local cleanup:
  BOXED-SUM-LOWERING-FACADE-001
  - one boxed-sum make/tag/project lowering facade
  - generic and same-module paths consume the same facade

broader shim cleanup:
  C-ABI-SHIM-RESPONSIBILITY-INVENTORY-001
  - classify .inc files that still own route policy, definition discovery,
    value-class derivation, object-storage inference, or fusion windows
  - no behavior change in the inventory task
```
