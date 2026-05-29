---
Status: Landed
Date: 2026-05-29
Scope: implement DirectSlotCellV0 usize storage compatibility before selected-method NativeDirect lowering.
Blocker: DIRECT-SLOT-USIZE-CELL-STORAGE-COMPATIBILITY-PILOT-296X-001
Related:
  - docs/development/current/main/phases/phase-296x/296x-336-DIRECT-SLOT-USIZE-CELL-STORAGE-COMPATIBILITY-SELECTION.md
  - crates/nyash_kernel/src/exports/typed_object_pinned_arena.rs
---

# 296x-337 Direct Slot USize Cell Storage Compatibility Pilot

## Purpose

Implement the row336 storage substrate fix for `usize` DirectSlot cells.

This row keeps LLVM lowering closed. It only makes `DirectSlotCellV0` preserve
`usize` as a distinct storage tag and verifies that materialized views keep the
original `TypedSlotStorage::USize` shape.

## Contract

```text
output_contract=direct-slot-usize-cell-storage-compatibility-pilot-v0
input_contract=direct-slot-usize-cell-storage-compatibility-selection-v0
implemented_owner=direct_slot_cell_v0_usize_storage_tag
implemented_owner_file=crates/nyash_kernel/src/exports/typed_object_pinned_arena.rs
implemented_storage_tag=DirectSlotCellV0::USize
implemented_storage_tag_value=4
cell_layout_size_bytes=16
cell_layout_alignment_bytes=8
target_pointer_width_required=64
usize_payload_representation=u64_payload
usize_materialization_storage=TypedSlotStorage::USize
usize_materialization_value=TypedSlotValue::Unsigned
u64_lease_storage_accepts_usize=1
direct_slot_lease_usize_read_write_smoke=ok
direct_slot_cell_usize_tag_smoke=ok
direct_slot_object_usize_snapshot_smoke=ok
direct_slot_exact_only=1
default_backend_emission=0
typed_slot_enum_layout_exposure=0
raw_runtime_vec_pointer_exposure=0
thread_local_refcell_pointer_exposure=0
llvm_lowering_open=0
native_direct_open=0
direct_load_store_open=0
selected_next=direct_slot_nativedirect_lowering_selected_method_pilot
winner_claim=0
replacement_active=0
hook_installed=0
global_allocator=0
summary=ok
```

## Notes

`usize` remains direct-compatible with the `u64` payload path only on 64-bit
targets. The cell tag is separate from `U64` so explicit materialized views can
reconstruct the original typed-slot storage.

This keeps the future selected-method lowering path simple:

```text
load/store DirectSlotCellV0.payload as i64
preserve storage truth through DirectSlotCellV0.storage_tag
```

No runtime helper semantics, public helper ABI, MIRBuilder rule, or `.hako`
source shape changes in this row.

## Guard

```bash
bash tools/checks/k2_wide_phase296x_direct_slot_usize_cell_storage_compatibility_pilot_guard.sh
```
