---
Status: Landed
Date: 2026-05-29
Scope: select the first materialization bridge for direct_slot_exact before helper routing or lowering.
Blocker: DIRECT-SLOT-BACKEND-MATERIALIZATION-POLICY-SELECTION-296X-001
Related:
  - docs/development/current/main/phases/phase-296x/296x-324-DIRECT-SLOT-MATERIALIZATION-FALLBACK-SYNC-SSOT.md
  - docs/development/current/main/phases/phase-296x/296x-326-DIRECT-SLOT-OBJECT-BACKEND-CONNECTION-PILOT.md
  - crates/nyash_kernel/src/exports/typed_object_pinned_arena.rs
  - crates/nyash_kernel/src/exports/typed_object_store.rs
---

# 296x-327 Direct Slot Backend Materialization Policy Selection

## Purpose

Select the first materialization bridge for the `direct_slot_exact` backend.

The bridge must preserve the row324 rule: `DirectSlotCellV0` is the primary
DirectSlot storage, and `TypedSlot` is only a compatibility/materialization view.
This row does not implement the bridge, does not route existing helpers to the
DirectSlot backend, and does not open LLVM lowering.

## Contract

```text
output_contract=direct-slot-backend-materialization-policy-selection-v0
input_contract=direct-slot-object-backend-connection-pilot-v0
selected_owner=direct_slot_object_to_typed_slot_snapshot_materialization
selected_reason=existing_helpers_must_not_route_to_direct_backend_until_explicit_materialization_view_exists
selected_bridge=direct_slot_object_v0_to_typed_slot_object_snapshot
sync_direction=direct_cell_to_typed_slot_snapshot
direct_cell_primary_storage=1
typed_slot_role=fallback_materialization_debug_view
typed_slot_primary_storage_in_direct_backend=0
dual_truth_allowed=0
implicit_sync_on_every_direct_write=0
per_write_typed_slot_update=0
materialization_trigger=explicit_only
materialization_view_lifetime=snapshot
materialization_requires_tagged_direct_handle=1
materialization_requires_generation_validation=1
materialization_requires_supported_storage_tags=1
unsupported_storage_tag_policy=fail_or_none_not_silent_fallback
generic_helper_route_to_direct_backend=0
exact_slot_helper_route_to_direct_backend=0
fallback_helper_reads_direct_cell_before_bridge=0
helper_routing_implementation_open=0
materialization_bridge_implementation_open=0
new_c_abi_helper_symbols=0
llvm_lowering_open=0
native_direct_open=0
direct_load_store_open=0
implementation_open=0
optimization_open=0
selected_next=direct_slot_backend_materialization_snapshot_pilot
winner_claim=0
replacement_active=0
hook_installed=0
global_allocator=0
summary=ok
```

## Decision

The first bridge is an explicit snapshot materialization:

```text
DirectSlotObjectV0 / DirectSlotCellV0
  -> TypedSlotObject / TypedSlot snapshot
```

This is not a permanent dual-storage policy. The snapshot is a compatibility
view for future helper fallback, debug, and public observer boundaries. Existing
helpers remain closed for `direct_slot_exact` until a later row implements and
guards the snapshot bridge.

## Rejected

```text
rejected=route_existing_helpers_directly_to_direct_slot_object
reason=no_materialization_view_exists_yet

rejected=update_typed_slot_view_on_every_direct_write
reason=would_create_dual_truth_and_reintroduce_per_write_overhead

rejected=llvm_lowering_against_direct_slot_object
reason=materialization_and_helper_fallback_policy_not_implemented_yet
```

## Guard

```bash
bash tools/checks/k2_wide_phase296x_direct_slot_backend_materialization_policy_selection_guard.sh
```
