---
Status: Landed
Date: 2026-05-29
Scope: select a typed-object field RMW/fusion seam after helper-free direct-op rejection.
Blocker: TYPED-OBJECT-FIELD-RMW-FUSION-SELECTION-296X-001
Related:
  - docs/development/current/main/phases/phase-296x/296x-220-MIR-TYPED-FIELD-DIRECT-OP-SELECTED-METHOD-KEEPER.md
  - docs/development/current/main/phases/phase-296x/296x-219-MIR-TYPED-FIELD-DIRECT-OP-GUARD-SURFACE.md
---

# 296x-221 Typed-Object Field RMW Fusion Selection

## Purpose

Select the next helper-reduction seam after rejecting helper-free typed-field
direct-op under the current opaque handle / Rust TLS Vec storage ABI.

This row does not implement the fused helper. It proves the selected method has
same-block `field_get -> binop + -> field_set` patterns where two exact-slot
helpers can be replaced by one runtime-owned fused helper.

## Evidence

```text
output_contract=typed-object-field-rmw-fusion-selection-v0
input_contract=mir-typed-field-direct-op-selected-method-feasibility-v0
workload_id=representative-object-lifecycle-small-block-v0
selected_method=HakoAllocPageModel.acquire_usize/1
selected_owner=typed_object_exact_slot_rmw_fusion
rmw_candidate_count=6
rmw_candidate_usize_count=6
rmw_candidate_u64_count=0
planned_erased_get_set_helper_calls=12
planned_added_fused_helper_calls=6
planned_net_helper_call_delta=6
planned_net_helper_call_delta_positive=1
runtime_storage_owner_preserved=1
helper_free_direct_op_rejected=1
generic_residence_open=0
source_rewrite=0
candidate_0_block=35
candidate_0_field=HakoAllocPageModel.reject_count
candidate_1_block=38
candidate_1_field=HakoAllocPageModel.reject_count
candidate_2_block=41
candidate_2_field=HakoAllocPageModel.reject_count
candidate_3_block=45
candidate_3_field=HakoAllocPageModel.used
candidate_4_block=45
candidate_4_field=HakoAllocPageModel.alloc_count
candidate_5_block=45
candidate_5_field=HakoAllocPageModel.requested_bytes
selected_next=typed_object_field_rmw_fusion_keeper
by_name_special_case=0
winner_claim=0
replacement_active=0
hook_installed=0
global_allocator=0
summary=ok
```

## Decision

```text
selected_owner=typed_object_exact_slot_rmw_fusion
implementation_kind=selected_method_same_block_field_get_add_set_fusion
runtime_storage_owner_preserved=1
helper_free_direct_op_rejected=1
```

The implementation row should keep typed-object storage ownership inside
`typed_object_store.rs` and add a narrow fused runtime helper instead of
exposing Rust object storage layout to LLVM.

## Guard

```bash
bash tools/checks/k2_wide_phase296x_typed_object_field_rmw_fusion_selection_guard.sh
```
