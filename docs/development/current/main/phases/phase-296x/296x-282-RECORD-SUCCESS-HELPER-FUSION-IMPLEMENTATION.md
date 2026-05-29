---
Status: Landed
Date: 2026-05-29
Scope: implement selected recordSuccess exact-slot helper fusion.
Blocker: RECORD-SUCCESS-HELPER-FUSION-IMPLEMENTATION-296X-001
Related:
  - docs/development/current/main/phases/phase-296x/296x-281-RECORD-SUCCESS-HELPER-FUSION-GUARD-SURFACE.md
---

# 296x-282 RecordSuccess Helper Fusion Implementation

## Purpose

Implement the narrow recordSuccess helper-fusion surface selected in row281.

This row keeps the optimization at the runtime exact-slot helper and C ABI
same-module body emit boundary:

- `.hako` source stays unchanged
- existing generic field helpers stay unchanged
- ValueAggregate, generic typed-field residence, and CSE stay closed
- provider activation, replacement, hooks, and globals stay closed

## Evidence

```text
output_contract=record-success-helper-fusion-implementation-v0
input_contract=record-success-helper-fusion-guard-surface-v0
implementation_owner=c_abi_same_module_record_success_helper_fusion
runtime_helper_symbol_0=nyash.object.exact_slot_record_alloc_success_hii
runtime_helper_symbol_1=nyash.object.exact_slot_record_release_success_hiii
runtime_helper_exported_count=2
runtime_helper_uses_single_thread_exact_store=1
same_module_emit_selected_method_count=2
same_module_emit_target_0=HakoAllocObjectLifecycleAllocResult.recordSuccess/1
same_module_emit_target_1=HakoAllocObjectLifecycleReleaseResult.recordSuccess/2
alloc_helper_contract=handle_selected_kind
release_helper_contract=handle_page_id_block_id
default_exact_helper_emission=0
exact_exe_record_success_alloc_symbol_present=1
exact_exe_record_success_release_symbol_present=1
semantic_proof_app=apps/hako-alloc-mimalloc-comparison-in-process-object-lifecycle-small-block-proof/main.hako
semantic_proof_summary=ok
planned_erased_exact_slot_get_set_count=14
planned_added_record_success_helper_count=2
planned_net_helper_call_delta=12
requires_hako_source_change=0
generic_typed_field_residence_open=0
generic_cse_open=0
capsule_value_aggregate_open=0
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
selected_next=record_success_helper_fusion_measurement
next_row=record_success_helper_fusion_measurement
optimization_open=0
```

The next row should measure the exact-EXE body time and refresh weighted owner
evidence before deciding whether this helper fusion is performance material.

## Guard

```bash
bash tools/checks/k2_wide_phase296x_record_success_helper_fusion_implementation_guard.sh
```
