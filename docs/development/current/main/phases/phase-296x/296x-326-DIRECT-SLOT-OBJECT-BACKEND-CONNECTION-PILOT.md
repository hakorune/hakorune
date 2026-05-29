---
Status: Landed
Date: 2026-05-29
Scope: add the direct_slot_exact backend selection point without routing helpers or lowering.
Blocker: DIRECT-SLOT-OBJECT-BACKEND-CONNECTION-PILOT-296X-001
Related:
  - docs/development/current/main/phases/phase-296x/296x-325-DIRECT-SLOT-OBJECT-BACKEND-CONNECTION-SELECTION.md
  - crates/nyash_kernel/src/exports/typed_object_store.rs
  - crates/nyash_kernel/src/exports/typed_object_pinned_arena.rs
---

# 296x-326 Direct Slot Object Backend Connection Pilot

## Purpose

Add the `direct_slot_exact` backend selection point and prove it can allocate a
`DirectSlotObjectV0` handle.

This row intentionally does not route existing helpers to the DirectSlot backend.
Materialization/fallback sync is still not implemented, so helper fallback must
remain closed for this backend.

## Evidence

```text
output_contract=direct-slot-object-backend-connection-pilot-v0
input_contract=direct-slot-object-backend-connection-selection-v0
implemented_backend=direct_slot_exact
implemented_owner=crates/nyash_kernel/src/exports/typed_object_store.rs
storage_owner=crates/nyash_kernel/src/exports/typed_object_pinned_arena.rs
direct_slot_object_allocation_smoke=ok
tagged_pointer_handle_smoke=ok
generic_helper_route_to_direct_backend=0
exact_slot_helper_route_to_direct_backend=0
materialization_bridge_implemented=0
fallback_bridge_implemented=0
existing_helper_abi_unchanged=1
default_backend_unchanged=1
pinned_arena_exact_backend_unchanged=1
llvm_lowering_open=0
native_direct_open=0
direct_load_store_open=0
new_c_abi_helper_symbols=0
winner_claim=0
replacement_active=0
hook_installed=0
global_allocator=0
summary=ok
```

## Guard

```bash
bash tools/checks/k2_wide_phase296x_direct_slot_object_backend_connection_pilot_guard.sh
```
