---
Status: Landed
Date: 2026-05-29
Scope: freeze recordSuccess helper-fusion guard surface after ValueAggregate rejection.
Blocker: RECORD-SUCCESS-HELPER-FUSION-GUARD-SURFACE-296X-001
Related:
  - docs/development/current/main/phases/phase-296x/296x-280-CAPSULE-VALUE-RESULT-CALLER-REGION-INVENTORY.md
---

# 296x-281 RecordSuccess Helper Fusion Guard Surface

## Purpose

Freeze the narrow implementation surface for result-capsule recordSuccess
helper fusion.

ValueAggregate was checked first and rejected for this surface because public
facade method returns force materialization. This row permits a narrow runtime
helper fusion as a bounded ExactSlotObject improvement.

## Evidence

```text
output_contract=record-success-helper-fusion-guard-surface-v0
input_contract=capsule-value-result-caller-region-inventory-v0
workload_id=representative-object-lifecycle-small-block-v0
selected_owner=record_success_helper_fusion
selected_owner_kind=runtime_exact_slot_record_success_helper
value_aggregate_rejected=1
value_aggregate_rejected_reason=public_method_return_boundary_prevents_value_delta_deferral
target_method_count=2
target_method_0=HakoAllocObjectLifecycleAllocResult.recordSuccess/1
target_method_0_shape=branch_aware_selected_kind
target_method_0_runtime_helper=nyash.object.exact_slot_record_alloc_success_hii
target_method_0_helper_contract=handle_selected_kind
target_method_0_erased_field_op_count=8
target_method_0_added_helper_count=1
target_method_1=HakoAllocObjectLifecycleReleaseResult.recordSuccess/2
target_method_1_shape=straightline_page_block
target_method_1_runtime_helper=nyash.object.exact_slot_record_release_success_hiii
target_method_1_helper_contract=handle_page_id_block_id
target_method_1_erased_field_op_count=6
target_method_1_added_helper_count=1
planned_erased_exact_slot_get_set_count=14
planned_added_record_success_helper_count=2
planned_net_helper_call_delta=12
planned_net_helper_call_delta_positive=1
requires_new_runtime_symbols=1
requires_c_abi_same_module_emit=1
requires_hako_source_change=0
semantic_proof_app=apps/hako-alloc-mimalloc-comparison-in-process-object-lifecycle-small-block-proof/main.hako
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

## Runtime Helper Contracts

```text
nyash.object.exact_slot_record_alloc_success_hii(handle, selected_kind)
  last_reason = 0
  last_ok = 1
  success_count += 1
  if selected_kind == 1: reusable_success_count += 1
  if selected_kind == 2: active_success_count += 1
  return 1 on success

nyash.object.exact_slot_record_release_success_hiii(handle, page_id, block_id)
  last_page_id = page_id
  last_block_id = block_id
  last_reason = 0
  last_ok = 1
  success_count += 1
  return 1 on success
```

The helpers must preserve existing exact-slot helper fallback semantics for all
other methods and all non-selected shapes.

## Decision

```text
selected_next=record_success_helper_fusion_implementation
next_row=record_success_helper_fusion_implementation
optimization_open=0
```

The implementation row may add the two runtime helper symbols and select only
the two named same-module methods. It must not reopen ValueAggregate,
source-level inline success, generic typed-field residence, CSE, provider
activation, replacement, hooks, or globals.

## Guard

```bash
bash tools/checks/k2_wide_phase296x_record_success_helper_fusion_guard_surface_guard.sh
```
