---
Status: Landed
Date: 2026-05-29
Scope: freeze result capsule reset field-batching guard surface before implementation.
Blocker: RESULT-CAPSULE-RESET-FIELD-BATCHING-GUARD-SURFACE-296X-001
Related:
  - docs/development/current/main/phases/phase-296x/296x-256-RESULT-CAPSULE-OWNER-SELECTION.md
---

# 296x-257 Result Capsule Reset Field-Batching Guard Surface

## Purpose

Freeze the narrow implementation surface for result capsule reset field
batching.

Both selected reset methods write the same four exact i64 slots:

```text
slot_0 last_page_id = -1
slot_1 last_block_id = -1
slot_2 last_reason = HakoAllocObjectLifecycleFacadeReason.ok() = 0
slot_3 last_ok = 0
```

The selected implementation must keep `.hako` source unchanged and add a
runtime-owned exact-slot batch helper rather than opening generic typed-field
residence, broad CSE, or capsule flattening.

## Evidence

```text
output_contract=result-capsule-reset-field-batching-guard-surface-v0
input_contract=result-capsule-owner-selection-v0
workload_id=representative-object-lifecycle-small-block-v0
selected_owner=result_capsule_reset_field_batching
selected_owner_kind=runtime_exact_slot_batch_helper
target_method_count=2
target_method_0=HakoAllocObjectLifecycleAllocResult.reset/0
target_method_1=HakoAllocObjectLifecycleReleaseResult.reset/0
target_field_count_per_method=4
target_field_0=last_page_id
target_slot_0=0
target_storage_0=i64
target_value_0=-1
target_field_1=last_block_id
target_slot_1=1
target_storage_1=i64
target_value_1=-1
target_field_2=last_reason
target_slot_2=2
target_storage_2=i64
target_value_2=0
target_field_3=last_ok
target_slot_3=3
target_storage_3=i64
target_value_3=0
new_runtime_helper_symbol=nyash.object.exact_slot_set4_i64_hiiiii
new_runtime_helper_contract=handle_start_slot_value0_value1_value2_value3
helper_sets_consecutive_i64_slots=1
helper_start_slot=0
planned_erased_exact_slot_set_count=8
planned_added_batch_helper_count=2
planned_net_helper_call_delta=6
planned_net_helper_call_delta_positive=1
requires_c_abi_same_module_emit=1
requires_new_runtime_symbol=1
requires_hako_source_change=0
semantic_proof_app=apps/hako-alloc-mimalloc-comparison-in-process-object-lifecycle-small-block-proof/main.hako
generic_typed_field_residence_open=0
generic_cse_open=0
capsule_flattening_open=0
birth_batching_open=0
record_success_fusion_open=0
source_rewrite=0
winner_claim=0
replacement_active=0
hook_installed=0
global_allocator=0
summary=ok
```

## Decision

```text
selected_next=result_capsule_reset_field_batching_implementation
next_row=result_capsule_reset_field_batching_implementation
optimization_open=0
```

The implementation row may add `nyash.object.exact_slot_set4_i64_hiiiii` and
lower only the two selected reset methods to that helper. It must preserve the
existing exact-slot helper ABI and keep fallback semantics for all other field
access.

## Guard

```bash
bash tools/checks/k2_wide_phase296x_result_capsule_reset_field_batching_guard_surface_guard.sh
```
