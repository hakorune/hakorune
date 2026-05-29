---
Status: Landed
Date: 2026-05-30
Scope: inventory whether HakoAllocPageModel.acquire_usize/1 can use DirectArray-backed ArraySlot NativeDirect lowering with positive helper-call delta.
Blocker: ARRAY-SLOT-NATIVEDIRECT-LOWERING-READINESS-INVENTORY-296X-001
Related:
  - docs/development/current/main/phases/phase-296x/296x-358-DIRECT-ARRAY-I64-HELPER-FALLBACK-CLOSEOUT-AND-LOWERING-READINESS-SELECTION.md
  - docs/development/current/main/phases/phase-296x/296x-348-ARRAY-SLOT-NATIVEDIRECT-GUARD-SURFACE.md
  - docs/development/current/main/phases/phase-296x/296x-208-MIR-ARRAY-SLOT-RESIDENCE-INVENTORY.md
---

# 296x-359 Array Slot NativeDirect Lowering Readiness Inventory

## Purpose

Inventory whether `HakoAllocPageModel.acquire_usize/1` can move from the
DirectArray scaffolding closeout into an ArraySlot NativeDirect lowering guard
surface.

This row is inventory-only. It does not implement LLVM lowering and does not
route existing ArrayBox helpers through DirectArray.

## Evidence

```text
output_contract=array-slot-nativedirect-lowering-readiness-inventory-v0
input_contract=direct-array-i64-helper-fallback-closeout-and-lowering-readiness-selection-v0
workload_id=representative-object-lifecycle-small-block-v0
selected_method=HakoAllocPageModel.acquire_usize/1
candidate_representation=NativeDirect
storage_substrate=DirectArrayI64BufferV0
direct_array_layout=repr_c_header_trailing_i64
fallback_boundary=explicit_public_arraybox_snapshot_handle
selected_block=45
candidate_array_get_count=1
candidate_array_set_count=1
candidate_array_helper_count=2
same_block_get_set_pair=1
set_uses_get_result=1
prior_array_residence_erased_get_set_helper_calls=2
prior_array_residence_added_guard_helper_calls=1
prior_array_residence_net_helper_call_delta=1
planned_erased_helper_ops=2
planned_added_helper_ops=0
planned_net_helper_delta=2
planned_net_helper_delta_positive=1
direct_array_buffer_available=1
contiguous_i64_data_available=1
materialized_view_boundary_available=1
helper_free_bridge_available=1
index_and_bounds_facts_available=1
append_policy_known=1
barrier_unknown_call_count=1
barrier_phi_count=1
fallback_materialization_boundary_known=1
silent_fallback_allowed=0
selected_next=array_slot_nativedirect_lowering_guard_surface
implementation_open=0
optimization_open=0
llvm_lowering_open=0
native_direct_open=0
direct_load_store_open=0
provider_activation=0
host_replacement=0
replacement_active=0
hook_installed=0
global_allocator=0
winner_claim=0
summary=ok
```

## Decision

The selected method may move to an ArraySlot NativeDirect lowering guard surface.

Unlike the earlier ArraySlot direct-op row, the planned representation now has a
helper-free DirectArray storage substrate and an explicit public ArrayBox
materialization boundary. That makes the planned added helper count zero for the
selected hot region, while keeping helper fallback and public ArrayBox semantics
closed until a later guard opens lowering.

## Guard

```bash
bash tools/checks/k2_wide_phase296x_array_slot_nativedirect_lowering_readiness_inventory_guard.sh
```
