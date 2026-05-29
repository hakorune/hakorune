---
Status: Landed
Date: 2026-05-29
Scope: implement explicit DirectSlot materialization into a separate helper-compatible negative TypedSlot view handle.
Blocker: DIRECT-SLOT-MATERIALIZED-VIEW-HANDLE-PILOT-296X-001
Related:
  - docs/development/current/main/phases/phase-296x/296x-330-DIRECT-SLOT-MATERIALIZED-VIEW-HANDLE-POLICY-SELECTION.md
  - crates/nyash_kernel/src/exports/typed_object_store.rs
---

# 296x-331 Direct Slot Materialized View Handle Pilot

## Purpose

Implement explicit materialization into a separate helper-compatible view handle.

The positive DirectSlot object handle remains closed to existing helpers. A
caller must explicitly materialize a snapshot view to obtain a negative
`TypedSlotObject` view handle. Existing helpers can operate on that view handle,
but writes to the view do not write back to the DirectSlot primary storage.

## Evidence

```text
output_contract=direct-slot-materialized-view-handle-pilot-v0
input_contract=direct-slot-materialized-view-handle-policy-selection-v0
implemented_owner=crates/nyash_kernel/src/exports/typed_object_store.rs
implemented_api=materialize_direct_slot_view_handle
materialized_view_storage=separate_thread_local_typed_slot_object_vec
direct_handle_sign=positive_tagged
materialized_view_handle_sign=negative_index_handle
direct_handle_helper_route=closed
materialized_view_helper_route=implemented
materialized_view_source=explicit_direct_slot_snapshot
materialized_view_lifetime=until_store_reset_or_process_exit
view_writeback_to_direct_slot=0
direct_cell_primary_storage=1
typed_slot_role=materialized_view_not_primary_storage
dual_truth_allowed=0
implicit_sync_on_every_direct_write=0
generic_helper_route_to_direct_backend=0
exact_slot_helper_route_to_direct_backend=0
new_c_abi_helper_symbols=0
llvm_lowering_open=0
native_direct_open=0
direct_load_store_open=0
direct_materialized_view_handle_smoke=ok
winner_claim=0
replacement_active=0
hook_installed=0
global_allocator=0
summary=ok
```

## Guard

```bash
bash tools/checks/k2_wide_phase296x_direct_slot_materialized_view_handle_pilot_guard.sh
```
