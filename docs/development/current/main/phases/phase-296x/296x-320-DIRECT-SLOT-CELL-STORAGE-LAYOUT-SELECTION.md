---
Status: Landed
Date: 2026-05-29
Scope: select the minimum stable direct-slot cell layout and storage owner before implementation.
Blocker: DIRECT-SLOT-CELL-STORAGE-LAYOUT-SELECTION-296X-001
Related:
  - docs/development/current/main/design/direct-slot-lease-addressable-slot-bridge-ssot.md
  - docs/development/current/main/phases/phase-296x/296x-319-DIRECT-SLOT-LEASE-ADDRESSABLE-SLOT-BRIDGE-SSOT.md
---

# 296x-320 Direct Slot Cell Storage Layout Selection

## Purpose

Select the first stable direct-slot cell layout before adding a storage
implementation.

The layout must be simple enough for LLVM to consume later, but this row does
not open LLVM lowering or expose slot addresses.

## Contract

```text
output_contract=direct-slot-cell-storage-layout-selection-v0
input_contract=direct-slot-lease-addressable-slot-bridge-ssot-v0
selected_owner=typed_object_direct_slot_cell_storage_layout
selected_layout=DirectSlotCellV0
selected_reason=need_stable_abi_cell_instead_of_private_typed_slot_rust_enum
cell_repr=repr_c
cell_storage_tag_type=u32
cell_flags_type=u32
cell_payload_type=u64
cell_size_bytes=16
cell_alignment_bytes=8
cell_payload_i64_encoding=two_complement_bits
cell_payload_u64_encoding=raw_u64_bits
cell_payload_handle_encoding=i64_bits
storage_tag_i64=1
storage_tag_u64=2
storage_tag_handle=3
unsupported_storage_policy=no_direct_cell_plan
object_header_repr=repr_c
object_header_type_id=i64
object_header_generation=u32
object_header_field_count=u32
fields_storage=pinned_boxed_slice
handle_resolution_contract=separate_next_row
llvm_consumable_slot_address_open=0
runtime_typed_slot_enum_layout_exposure=0
raw_runtime_vec_pointer_exposure=0
thread_local_refcell_pointer_exposure=0
existing_helper_abi_unchanged=1
lowering_open=0
native_direct_open=0
direct_load_store_open=0
implementation_open=0
optimization_open=0
selected_next=direct_slot_cell_storage_pilot
winner_claim=0
replacement_active=0
hook_installed=0
global_allocator=0
summary=ok
```

## Decision

Use a dedicated `DirectSlotCellV0` storage cell:

```text
repr(C) DirectSlotCellV0 {
  storage_tag: u32
  flags: u32
  payload: u64
}
```

This is not the public `TypedSlot` layout. It is a lowerable substrate for
selected direct-slot regions and fallback/materialization paths.

Handle-to-address resolution is deliberately left to the next design/implementation
row. This row only fixes the stable cell and object storage vocabulary.

## Guard

```bash
bash tools/checks/k2_wide_phase296x_direct_slot_cell_storage_layout_selection_guard.sh
```
