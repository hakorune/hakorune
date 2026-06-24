---
Status: Landed
Date: 2026-05-29
Scope: implement selected result capsule reset field batching.
Blocker: RESULT-CAPSULE-RESET-FIELD-BATCHING-IMPLEMENTATION-296X-001
Related:
  - docs/development/current/main/phases/phase-296x/296x-257-RESULT-CAPSULE-RESET-FIELD-BATCHING-GUARD-SURFACE.md
---

# 296x-258 Result Capsule Reset Field-Batching Implementation

## Purpose

Implement the selected result capsule reset field-batching helper while keeping
the optimization narrow:

- runtime owns the exact-slot storage mutation
- C ABI same-module emit selects only the two reset methods
- `.hako` source stays unchanged
- generic typed-field residence, CSE, and capsule flattening stay closed

## Evidence

```text
output_contract=result-capsule-reset-field-batching-implementation-v0
input_contract=result-capsule-reset-field-batching-guard-surface-v0
implementation_owner=c_abi_same_module_result_capsule_reset_batching
runtime_helper_symbol=nyash.object.exact_slot_set4_i64_hiiiii
runtime_helper_contract=handle_start_slot_value0_value1_value2_value3
runtime_helper_exported=1
runtime_helper_sets_consecutive_i64_slots=1
runtime_helper_uses_single_thread_exact_store=1
same_module_emit_selected_method_count=2
same_module_emit_target_0=HakoAllocObjectLifecycleAllocResult.reset/0
same_module_emit_target_1=HakoAllocObjectLifecycleReleaseResult.reset/0
same_module_emit_target_slots=0,1,2,3
same_module_emit_target_values=-1,-1,0,0
exact_exe_set4_symbol_present=1
semantic_proof_app=apps/hako-alloc-mimalloc-comparison-in-process-object-lifecycle-small-block-proof/main.hako
semantic_proof_summary=ok
planned_erased_exact_slot_set_count=8
planned_added_batch_helper_count=2
planned_net_helper_call_delta=6
requires_hako_source_change=0
generic_typed_field_residence_open=0
generic_cse_open=0
capsule_flattening_open=0
source_rewrite=0
winner_claim=0
replacement_active=0
hook_installed=0
global_allocator=0
summary=ok
```

## Changed Surfaces

```text
crates/nyash_kernel/src/exports/typed_object.rs
crates/nyash_kernel/src/exports/typed_object_store.rs
lang/c-abi/shims/hako_llvmc_ffi_pure_compile_generic_lowering_prescan.inc
lang/c-abi/shims/hako_llvmc_ffi_same_module_body_emit.inc
```

## Decision

```text
selected_next=result_capsule_reset_field_batching_measurement
next_row=result_capsule_reset_field_batching_measurement
optimization_open=0
```

The next row should measure body time and refresh owner evidence before
accepting the keeper as performance material.

## Guard

```bash
bash tools/checks/k2_wide_phase296x_result_capsule_reset_field_batching_implementation_guard.sh
```
