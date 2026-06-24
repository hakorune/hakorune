---
Status: Landed
Date: 2026-05-29
Scope: close helper fallback materialization scaffolding and select the next DirectSlot lowering readiness row.
Blocker: DIRECT-SLOT-HELPER-FALLBACK-CLOSEOUT-AND-LOWERING-READINESS-SELECTION-296X-001
Related:
  - docs/development/current/main/phases/phase-296x/296x-331-DIRECT-SLOT-MATERIALIZED-VIEW-HANDLE-PILOT.md
  - docs/development/current/main/phases/phase-296x/296x-317-DIRECT-SLOT-LEASE-LOWERING-GUARD-SURFACE.md
  - docs/development/current/main/phases/phase-296x/296x-318-DIRECT-SLOT-LEASE-LOWERING-PILOT.md
---

# 296x-332 Direct Slot Helper Fallback Closeout And Lowering Readiness Selection

## Purpose

Close the DirectSlot helper fallback/materialization scaffolding sequence and
select the next lowering-readiness row.

Rows 323-331 now provide the pieces that were missing when row318 rejected
DirectSlot lowering:

- stable `DirectSlotObjectV0` layout;
- tagged stable object pointer handle;
- `DirectSlotCellV0` primary storage policy;
- explicit snapshot materialization;
- separate negative materialized view handles for helper compatibility.

This row still does not open LLVM lowering. The next row must inventory whether
the selected hot method can use the new substrate with positive helper-call
delta and explicit fallback/materialization boundaries.

## Contract

```text
output_contract=direct-slot-helper-fallback-closeout-and-lowering-readiness-selection-v0
input_contract=direct-slot-materialized-view-handle-pilot-v0
fallback_scaffolding_closeout=1
direct_slot_object_layout_ready=1
direct_slot_handle_resolution_ready=1
direct_slot_cell_primary_storage_ready=1
explicit_snapshot_materialization_ready=1
materialized_view_handle_ready=1
existing_helper_fallback_boundary_ready=1
direct_handle_helper_route_closed=1
per_helper_snapshot_routing_closed=1
typed_slot_enum_layout_exposure=0
raw_runtime_vec_pointer_exposure=0
thread_local_refcell_pointer_exposure=0
by_name_hako_alloc_special_case=0
selected_owner=direct_slot_nativedirect_lowering_readiness_inventory
selected_reason=row318_helper_free_bridge_gap_is_now_replaced_by_stable_direct_slot_object_and_explicit_materialization_boundary
selected_target_method=HakoAllocPageModel.acquire_usize/1
selected_next=direct_slot_nativedirect_lowering_readiness_inventory
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

## Next Row Requirements

The next row must be inventory-only and prove:

```text
positive_net_helper_delta_required=1
selected_method_only=1
direct_handle_available=1
slot_address_calculation_available=1
fallback_materialization_boundary_known=1
unknown_escape_barrier_count=...
materialized_view_boundary_count=...
silent_fallback_allowed=0
```

If those facts are not positive, the lane must not open lowering.

## Guard

```bash
bash tools/checks/k2_wide_phase296x_direct_slot_helper_fallback_closeout_and_lowering_readiness_selection_guard.sh
```
