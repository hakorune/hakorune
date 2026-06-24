---
Status: Landed
Date: 2026-05-29
Scope: inventory whether HakoAllocPageModel.acquire_usize/1 can use DirectSlot NativeDirect lowering with positive net helper delta.
Blocker: DIRECT-SLOT-NATIVEDIRECT-LOWERING-READINESS-INVENTORY-296X-001
Related:
  - docs/development/current/main/phases/phase-296x/296x-332-DIRECT-SLOT-HELPER-FALLBACK-CLOSEOUT-AND-LOWERING-READINESS-SELECTION.md
  - docs/development/current/main/phases/phase-296x/296x-316-DIRECT-SLOT-LEASE-SELECTED-METHOD-INVENTORY.md
  - docs/development/current/main/phases/phase-296x/296x-304-TYPED-OBJECT-RESIDENT-SCALAR-FEASIBILITY-CLOSEOUT.md
---

# 296x-333 Direct Slot NativeDirect Lowering Readiness Inventory

## Purpose

Inventory whether `HakoAllocPageModel.acquire_usize/1` can re-enter a
NativeDirect lowering guard surface now that DirectSlot storage and explicit
fallback/materialization boundaries exist.

This row is inventory-only. It does not implement LLVM lowering.

## Evidence

```text
output_contract=direct-slot-nativedirect-lowering-readiness-inventory-v0
input_contract=direct-slot-helper-fallback-closeout-and-lowering-readiness-selection-v0
workload_id=representative-object-lifecycle-small-block-v0
selected_method=HakoAllocPageModel.acquire_usize/1
candidate_representation=NativeDirect
storage_substrate=DirectSlotObjectV0
direct_cell_layout=DirectSlotCellV0
direct_handle_kind=tagged_stable_direct_slot_object_pointer
fallback_boundary=explicit_materialized_view_handle
prior_resident_scalar_erased_field_get_count=11
prior_resident_scalar_erased_field_set_count=8
prior_resident_scalar_inserted_helper_load_count=11
prior_resident_scalar_inserted_helper_writeback_count=8
prior_resident_scalar_net_helper_call_delta=0
candidate_exact_slot_get_count=13
candidate_exact_slot_set_count=8
candidate_exact_slot_helper_count=21
resident_field_key_count=11
planned_erased_helper_ops=21
planned_added_helper_ops=0
planned_net_helper_delta=21
planned_net_helper_delta_positive=1
direct_handle_available=1
slot_address_calculation_available=1
materialized_view_boundary_available=1
helper_free_bridge_available=1
unknown_receiver_count=0
unknown_field_plan_count=0
unsupported_storage_count=0
weak_field_count=0
barrier_unknown_call_count=1
barrier_phi_count=1
barrier_return_count=5
fallback_materialization_boundary_known=1
silent_fallback_allowed=0
selected_next=direct_slot_nativedirect_lowering_guard_surface
implementation_open=0
optimization_open=0
llvm_lowering_open=0
native_direct_open=0
direct_load_store_open=0
winner_claim=0
replacement_active=0
hook_installed=0
global_allocator=0
summary=ok
```

## Decision

The selected method may move to a guard-surface row.

The key difference from row304 is that the candidate no longer replaces field
helpers with helper loads/writebacks. The DirectSlot path now has a stable object
layout, direct cell layout, tagged pointer handle, and explicit materialized
view fallback boundary. Those facts make a helper-free NativeDirect guard
surface meaningful again.

## Guard

```bash
bash tools/checks/k2_wide_phase296x_direct_slot_nativedirect_lowering_readiness_inventory_guard.sh
```
