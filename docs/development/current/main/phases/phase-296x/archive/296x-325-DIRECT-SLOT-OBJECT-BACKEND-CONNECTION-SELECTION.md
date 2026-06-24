---
Status: Landed
Date: 2026-05-29
Scope: select how DirectSlotObjectV0 connects to the typed-object backend without opening lowering.
Blocker: DIRECT-SLOT-OBJECT-BACKEND-CONNECTION-SELECTION-296X-001
Related:
  - docs/development/current/main/phases/phase-296x/296x-324-DIRECT-SLOT-MATERIALIZATION-FALLBACK-SYNC-SSOT.md
  - crates/nyash_kernel/src/exports/typed_object_store.rs
  - crates/nyash_kernel/src/exports/typed_object_pinned_arena.rs
---

# 296x-325 Direct Slot Object Backend Connection Selection

## Purpose

Select the next implementation owner for connecting `DirectSlotObjectV0` to the
typed-object runtime backend.

The connection must stay runtime/storage-only. It must not emit DirectSlot
handles by default, and it must not open LLVM lowering.

## Contract

```text
output_contract=direct-slot-object-backend-connection-selection-v0
input_contract=direct-slot-materialization-fallback-sync-ssot-v0
selected_owner=typed_object_store_direct_slot_backend_connection
selected_backend_name=direct_slot_exact
selected_reason=connect_stable_direct_slot_object_storage_before_lowering_retry
new_backend_allowed=1
default_backend_unchanged=1
pinned_arena_exact_backend_unchanged=1
direct_slot_object_storage_owner=crates/nyash_kernel/src/exports/typed_object_pinned_arena.rs
typed_object_backend_selector_owner=crates/nyash_kernel/src/exports/typed_object_store.rs
direct_slot_primary_storage=DirectSlotCellV0
typed_slot_role=fallback_materialization_debug_view
direct_handle_emission_default=0
generic_helper_route_to_direct_backend=0
exact_slot_helper_route_to_direct_backend=0
materialization_bridge_required_before_helper_route=1
new_c_abi_helper_symbols=0
llvm_lowering_open=0
native_direct_open=0
direct_load_store_open=0
implementation_open=0
optimization_open=0
selected_next=direct_slot_object_backend_connection_pilot
winner_claim=0
replacement_active=0
hook_installed=0
global_allocator=0
summary=ok
```

## Decision

Add a distinct backend selection point named `direct_slot_exact`.

This backend is not the same as `pinned_arena_exact`:

- `pinned_arena_exact` preserves existing helper compatibility;
- `direct_slot_exact` is the future DirectSlot primary-storage backend.

The pilot may add backend selection and storage construction smoke only. Helper
fallback routing and LLVM lowering remain closed until materialization/fallback
sync has an implementation.

## Guard

```bash
bash tools/checks/k2_wide_phase296x_direct_slot_object_backend_connection_selection_guard.sh
```
