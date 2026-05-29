---
Status: Landed
Date: 2026-05-30
Scope: close DirectArray helper fallback materialization scaffolding and select the next ArraySlot lowering readiness row.
Blocker: DIRECT-ARRAY-I64-HELPER-FALLBACK-CLOSEOUT-AND-LOWERING-READINESS-SELECTION-296X-001
Related:
  - docs/development/current/main/phases/phase-296x/296x-357-DIRECT-ARRAY-I64-MATERIALIZED-VIEW-HANDLE-PILOT.md
  - docs/development/current/main/phases/phase-296x/296x-348-ARRAY-SLOT-NATIVEDIRECT-GUARD-SURFACE.md
---

# 296x-358 Direct Array I64 Helper Fallback Closeout And Lowering Readiness Selection

## Purpose

Close the DirectArray helper fallback/materialization scaffolding sequence and
select the next lowering-readiness inventory row.

Rows 348-357 now provide the pieces required before ArraySlot NativeDirect can
return to lowering:

- ArraySlot NativeDirect guard surface;
- stable `DirectArrayI64BufferV0` layout;
- storage-only DirectArray buffer pilot;
- DirectArray primary / public ArrayBox materialized view policy;
- `direct_array_i64_exact` backend selection point;
- explicit public ArrayBox snapshot materialization;
- separate public ArrayBox host handles for helper compatibility.

This row still does not open LLVM lowering. The next row must inventory whether
a selected hot ArraySlot region can use the new substrate with positive
helper-call delta and explicit fallback/materialization boundaries.

## Contract

```text
output_contract=direct-array-i64-helper-fallback-closeout-and-lowering-readiness-selection-v0
input_contract=direct-array-i64-materialized-view-handle-pilot-v0
fallback_scaffolding_closeout=1
direct_array_i64_layout_ready=1
direct_array_i64_storage_ready=1
direct_array_i64_backend_selection_ready=1
direct_array_i64_primary_storage_policy_ready=1
explicit_snapshot_materialization_ready=1
materialized_view_handle_ready=1
existing_helper_fallback_boundary_ready=1
direct_array_helper_route_closed=1
per_helper_snapshot_routing_closed=1
arraybox_items_rwlock_exposure=0
array_slot_cache_vec_exposure=0
by_name_hako_alloc_special_case=0
selected_owner=array_slot_nativedirect_lowering_readiness_inventory
selected_reason=array_helper_boundary_remains_dominant_and_direct_array_scaffolding_is_ready
selected_target_method=HakoAllocPageModel.acquire_usize/1
selected_next=array_slot_nativedirect_lowering_readiness_inventory
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

## Next Row Requirements

The next row must be inventory-only and prove:

```text
positive_net_helper_delta_required=1
selected_method_only=1
direct_array_buffer_available=1
index_and_bounds_facts_available=1
fallback_materialization_boundary_known=1
unknown_escape_barrier_count=...
materialized_view_boundary_count=...
silent_fallback_allowed=0
```

If those facts are not positive, the lane must not open lowering.

## Guard

```bash
bash tools/checks/k2_wide_phase296x_direct_array_i64_helper_fallback_closeout_and_lowering_readiness_selection_guard.sh
```
