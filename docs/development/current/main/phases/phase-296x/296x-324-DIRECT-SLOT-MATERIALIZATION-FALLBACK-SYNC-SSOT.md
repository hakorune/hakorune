---
Status: Landed
Date: 2026-05-29
Scope: define DirectSlot primary storage, fallback view, and materialization sync before lowering.
Blocker: DIRECT-SLOT-MATERIALIZATION-FALLBACK-SYNC-SSOT-296X-001
Related:
  - docs/development/current/main/design/direct-slot-lease-addressable-slot-bridge-ssot.md
  - docs/development/current/main/phases/phase-296x/296x-323-DIRECT-SLOT-OBJECT-LAYOUT-PILOT.md
---

# 296x-324 Direct Slot Materialization Fallback Sync SSOT

## Purpose

Define the storage truth and sync policy before DirectSlot lowering or backend
connection work continues.

The key rule from row323 is that `DirectSlotCellV0` must not become a cache next
to a separate `TypedSlot` truth. For DirectSlot backend objects, the direct cell
is the primary storage direction. Existing `TypedSlot` helpers remain the current
compatibility/fallback view until a materialization path is implemented.

## Contract

```text
output_contract=direct-slot-materialization-fallback-sync-ssot-v0
input_contract=direct-slot-object-layout-pilot-v0
selected_owner=direct_slot_materialization_fallback_sync_policy
direct_backend_primary_storage=DirectSlotCellV0
direct_object_layout=DirectSlotObjectV0
typed_slot_role=fallback_materialization_debug_view
typed_slot_primary_storage_in_direct_backend=0
direct_cell_cache_only_policy=0
dual_truth_allowed=0
sync_direction_for_current_pilot=direct_cell_to_typed_slot_on_explicit_materialization
implicit_sync_on_every_direct_write=0
existing_helper_path=typed_slot_compatibility_until_materialization_bridge
fallback_helper_reads_direct_cell_before_lowering=0
fallback_helper_reads_typed_slot_view_before_bridge=1
materialization_required_before_public_observer=1
materialization_required_before_existing_helper_fallback=1
materialization_required_before_unknown_escape=1
materialization_required_before_debug_view=1
materialization_policy_implementation_open=0
fallback_bridge_implementation_open=0
llvm_lowering_open=0
native_direct_open=0
direct_load_store_open=0
existing_helper_abi_unchanged=1
default_backend_direct_handle_emission=0
raw_runtime_vec_pointer_exposure=0
thread_local_refcell_pointer_exposure=0
typed_slot_enum_layout_exposure=0
by_name_hako_alloc_special_case=0
selected_next=direct_slot_object_backend_connection_selection
winner_claim=0
replacement_active=0
hook_installed=0
global_allocator=0
summary=ok
```

## Policy

For DirectSlot backend objects:

```text
primary truth:
  DirectSlotObjectV0 / DirectSlotCellV0

compatibility view:
  TypedSlot materialized view

allowed sync:
  DirectSlotCellV0 -> TypedSlot view at explicit materialization boundaries

forbidden sync:
  implicit dual-write truth as a long-term policy
```

The current `PinnedTypedObjectArena` still maintains `TypedSlot` and
`DirectSlotCellV0` side by side for pilot compatibility. That is a temporary
bridge, not the final DirectSlot backend truth model.

## Next Row

The next row may select how to connect `DirectSlotObjectV0` into the typed-object
backend. It must not open LLVM lowering.

## Guard

```bash
bash tools/checks/k2_wide_phase296x_direct_slot_materialization_fallback_sync_ssot_guard.sh
```
