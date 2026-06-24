---
Status: Landed
Date: 2026-05-29
Scope: implement the stable DirectSlotCellV0 storage substrate without opening LLVM lowering.
Blocker: DIRECT-SLOT-CELL-STORAGE-PILOT-296X-001
Related:
  - docs/development/current/main/phases/phase-296x/296x-320-DIRECT-SLOT-CELL-STORAGE-LAYOUT-SELECTION.md
  - crates/nyash_kernel/src/exports/typed_object_pinned_arena.rs
---

# 296x-321 Direct Slot Cell Storage Pilot

## Purpose

Implement the storage-only `DirectSlotCellV0` substrate in the pinned typed-object
arena.

This row keeps helper ABI and LLVM lowering unchanged. It only proves the stable
cell layout and tagged payload storage needed by the future addressable bridge.

## Evidence

```text
output_contract=direct-slot-cell-storage-pilot-v0
input_contract=direct-slot-cell-storage-layout-selection-v0
implemented_owner=crates/nyash_kernel/src/exports/typed_object_pinned_arena.rs
implemented_layout=DirectSlotCellV0
cell_repr=repr_c
cell_storage_tag_type=u32
cell_flags_type=u32
cell_payload_type=u64
cell_size_bytes=16
cell_alignment_bytes=8
storage_tag_i64=1
storage_tag_u64=2
storage_tag_handle=3
direct_cell_parallel_storage=1
typed_slot_fallback_storage_preserved=1
direct_slot_lease_token_reads_cells=1
direct_slot_lease_token_updates_fallback_field_on_write=1
existing_helper_abi_unchanged=1
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
bash tools/checks/k2_wide_phase296x_direct_slot_cell_storage_pilot_guard.sh
```
