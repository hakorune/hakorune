---
Status: Landed
Date: 2026-05-29
Scope: implement explicit DirectSlotObjectV0 to TypedSlot snapshot materialization without helper routing or lowering.
Blocker: DIRECT-SLOT-BACKEND-MATERIALIZATION-SNAPSHOT-PILOT-296X-001
Related:
  - docs/development/current/main/phases/phase-296x/296x-327-DIRECT-SLOT-BACKEND-MATERIALIZATION-POLICY-SELECTION.md
  - crates/nyash_kernel/src/exports/typed_object_pinned_arena.rs
  - crates/nyash_kernel/src/exports/typed_object_store.rs
---

# 296x-328 Direct Slot Backend Materialization Snapshot Pilot

## Purpose

Implement the first explicit materialization bridge:

```text
DirectSlotObjectV0 / DirectSlotCellV0
  -> TypedSlotObject / TypedSlot snapshot
```

This is still runtime/storage-only. Existing helpers do not automatically route
to the DirectSlot backend, and LLVM lowering remains closed.

## Evidence

```text
output_contract=direct-slot-backend-materialization-snapshot-pilot-v0
input_contract=direct-slot-backend-materialization-policy-selection-v0
implemented_bridge=direct_slot_object_v0_to_typed_slot_object_snapshot
implemented_owner=crates/nyash_kernel/src/exports/typed_object_pinned_arena.rs
backend_entry_owner=crates/nyash_kernel/src/exports/typed_object_store.rs
materialization_trigger=explicit_only
materialization_view_lifetime=snapshot
sync_direction=direct_cell_to_typed_slot_snapshot
direct_cell_primary_storage=1
typed_slot_role=fallback_materialization_debug_view
dual_truth_allowed=0
implicit_sync_on_every_direct_write=0
per_write_typed_slot_update=0
supported_storage_tags=i64,u64,handle
unsupported_storage_tag_policy=none_not_silent_fallback
tagged_pointer_handle_validation=1
direct_slot_snapshot_smoke=ok
generic_helper_route_to_direct_backend=0
exact_slot_helper_route_to_direct_backend=0
helper_routing_implementation_open=0
new_c_abi_helper_symbols=0
llvm_lowering_open=0
native_direct_open=0
direct_load_store_open=0
winner_claim=0
replacement_active=0
hook_installed=0
global_allocator=0
summary=ok
```

## Guard

```bash
bash tools/checks/k2_wide_phase296x_direct_slot_backend_materialization_snapshot_pilot_guard.sh
```
