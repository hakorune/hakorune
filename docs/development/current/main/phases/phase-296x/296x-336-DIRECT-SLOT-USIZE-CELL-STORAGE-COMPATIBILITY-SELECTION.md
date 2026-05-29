---
Status: Landed
Date: 2026-05-29
Scope: select DirectSlotCellV0 usize storage compatibility before selected-method NativeDirect lowering.
Blocker: DIRECT-SLOT-USIZE-CELL-STORAGE-COMPATIBILITY-SELECTION-296X-001
Related:
  - docs/development/current/main/phases/phase-296x/296x-333-DIRECT-SLOT-NATIVEDIRECT-LOWERING-READINESS-INVENTORY.md
  - docs/development/current/main/phases/phase-296x/296x-334-DIRECT-SLOT-NATIVEDIRECT-LOWERING-GUARD-SURFACE.md
  - docs/development/current/main/phases/phase-296x/296x-335-DIRECT-SLOT-NATIVEDIRECT-LOWERING-OWNER-SELECTION.md
  - crates/nyash_kernel/src/exports/typed_object_pinned_arena.rs
---

# 296x-336 Direct Slot USize Cell Storage Compatibility Selection

## Purpose

Close the storage compatibility gap found before opening selected-method
DirectSlot NativeDirect lowering.

`HakoAllocPageModel.acquire_usize/1` is the selected first NativeDirect pilot,
but its hot field plan includes `usize` storage. The existing `DirectSlotCellV0`
pilot only preserves `i64`, `u64`, and `handle` tags. Lowering `usize` through
the `u64` payload path without preserving the storage tag would make explicit
materialized views lose the original `usize` storage shape.

This row selects the narrow substrate fix. It does not implement LLVM lowering.

## Contract

```text
output_contract=direct-slot-usize-cell-storage-compatibility-selection-v0
input_contract=direct-slot-nativedirect-lowering-owner-selection-v0
selected_owner=direct_slot_cell_v0_usize_storage_tag
selected_owner_file=crates/nyash_kernel/src/exports/typed_object_pinned_arena.rs
selected_reason=selected_method_acquire_usize_uses_usize_fields_and_materialized_view_must_preserve_storage_shape
selected_storage_tag=DirectSlotCellV0::USize
selected_storage_tag_value=4
cell_layout_size_bytes_unchanged=16
cell_layout_alignment_bytes_unchanged=8
target_pointer_width_required=64
usize_payload_representation=u64_payload
usize_materialization_storage=TypedSlotStorage::USize
usize_materialization_value=TypedSlotValue::Unsigned
u64_lease_storage_accepts_usize=1
direct_slot_exact_only=1
default_backend_emission=0
typed_slot_enum_layout_exposure=0
raw_runtime_vec_pointer_exposure=0
thread_local_refcell_pointer_exposure=0
llvm_lowering_open=0
native_direct_open=0
direct_load_store_open=0
selected_next=direct_slot_usize_cell_storage_compatibility_pilot
winner_claim=0
replacement_active=0
hook_installed=0
global_allocator=0
summary=ok
```

## Decision

Add a dedicated `USize` DirectSlot cell tag instead of treating `usize` as
`U64` during materialization.

The payload remains a 64-bit scalar on 64-bit targets, so future selected-method
NativeDirect lowering can still use the same payload address formula. The tag
keeps the materialized view honest:

```text
DirectSlotCellV0(USize, payload)
  -> TypedSlot { storage: USize, value: Unsigned(payload) }
```

`DirectSlotLeaseStorage::U64` may read/write a `USize` direct cell only on
64-bit targets. That mirrors the existing exact-slot helper policy where
`usize` is direct-compatible with `u64` under `target_pointer_width=64`.

## Rejected Options

```text
rejected=lower_usize_as_u64_without_new_tag
reason=explicit_materialized_view_would_lose_usize_storage_shape

rejected=block_acquire_usize_from_first_nativedirect_pilot
reason=row333_and_row334_already_selected_acquire_usize_with_positive_helper_delta

rejected=expose_typed_slot_enum_layout_to_lowering
reason=DirectSlotCellV0_remains_the_stable_direct_lowering_cell_ABI
```

## Guard

```bash
bash tools/checks/k2_wide_phase296x_direct_slot_usize_cell_storage_compatibility_selection_guard.sh
```
