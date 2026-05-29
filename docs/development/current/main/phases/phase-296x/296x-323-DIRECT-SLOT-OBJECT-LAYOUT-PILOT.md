---
Status: Landed
Date: 2026-05-29
Scope: implement a stable DirectSlotObjectV0 layout pilot without opening LLVM lowering.
Blocker: DIRECT-SLOT-OBJECT-LAYOUT-PILOT-296X-001
Related:
  - docs/development/current/main/phases/phase-296x/296x-322-DIRECT-SLOT-HANDLE-RESOLUTION-CONTRACT.md
  - crates/nyash_kernel/src/exports/typed_object_pinned_arena.rs
---

# 296x-323 Direct Slot Object Layout Pilot

## Purpose

Implement the storage-only `DirectSlotObjectV0` layout pilot.

This row proves that a stable object pointer can address a trailing
`DirectSlotCellV0` slice with predictable offsets. It keeps LLVM lowering,
NativeDirect emission, and materialization/fallback sync closed.

## Evidence

```text
output_contract=direct-slot-object-layout-pilot-v0
input_contract=direct-slot-handle-resolution-contract-v0
implemented_owner=crates/nyash_kernel/src/exports/typed_object_pinned_arena.rs
implemented_layout=DirectSlotObjectV0
object_repr=repr_c
object_header_type_id=i64
object_header_generation=u32
object_header_field_count=u32
object_header_flags=u32
object_header_reserved=u32
object_header_size_bytes=24
object_alignment_bytes=8
fields_layout=trailing_direct_slot_cell_v0_slice
cell_layout=DirectSlotCellV0
cell_size_bytes=16
cell_alignment_bytes=8
field0_offset_bytes=24
field_address_calculation_smoke=ok
handle_payload=tagged_stable_object_pointer
handle_roundtrip_smoke=ok
direct_cell_primary_storage_policy=selected_for_direct_backend
typed_slot_fallback_view_policy=preserved_for_current_helpers
materialization_policy=deferred_required_before_lowering
fallback_sync_policy=deferred_required_before_lowering
typed_slot_enum_layout_exposure=0
raw_runtime_vec_pointer_exposure=0
thread_local_refcell_pointer_exposure=0
existing_helper_abi_unchanged=1
default_backend_direct_handle_emission=0
llvm_lowering_open=0
native_direct_open=0
direct_load_store_open=0
implementation_open=0
optimization_open=0
selected_next=direct_slot_materialization_fallback_sync_ssot
winner_claim=0
replacement_active=0
hook_installed=0
global_allocator=0
summary=ok
```

## Decision

`DirectSlotObjectV0` is a direct backend storage pilot, not a public object
replacement. `DirectSlotCellV0` is selected as the primary storage direction for
the DirectSlot backend, while the existing `TypedSlot` helper view remains only
the current compatibility/fallback view.

The next row must define materialization/fallback sync before any LLVM lowering
is opened.

## Guard

```bash
bash tools/checks/k2_wide_phase296x_direct_slot_object_layout_pilot_guard.sh
```
