---
Status: Landed
Date: 2026-05-29
Scope: define how a DirectSlot handle resolves to a compiler-consumable cell address.
Blocker: DIRECT-SLOT-HANDLE-RESOLUTION-CONTRACT-296X-001
Related:
  - docs/development/current/main/design/direct-slot-lease-addressable-slot-bridge-ssot.md
  - docs/development/current/main/phases/phase-296x/296x-321-DIRECT-SLOT-CELL-STORAGE-PILOT.md
---

# 296x-322 Direct Slot Handle Resolution Contract

## Purpose

Define the handle-resolution contract for future direct-slot lowering.

`DirectSlotCellV0` gives a stable cell layout, but LLVM still needs a contracted
way to resolve a selected receiver handle to an object header and field-cell
address without C ABI load/writeback helpers.

## Contract

```text
output_contract=direct-slot-handle-resolution-contract-v0
input_contract=direct-slot-cell-storage-pilot-v0
selected_owner=direct_slot_handle_resolution_contract
selected_handle_kind=tagged_stable_object_pointer
selected_reason=avoid_tls_refcell_arena_lookup_and_helper_load_writeback_zero_net
object_layout=DirectSlotObjectV0
object_layout_repr=repr_c
object_header_type_id=i64
object_header_generation=u32
object_header_field_count=u32
object_fields_layout=trailing_direct_slot_cell_v0_slice
handle_payload=stable_object_pointer
handle_tag_bits_required=1
handle_alignment_required=8
handle_resolution_in_llvm_allowed_after_layout_pilot=1
handle_points_to_vec_storage=0
handle_points_to_refcell_storage=0
handle_points_to_rust_enum_typed_slot=0
raw_runtime_vec_pointer_exposure=0
generation_validation_policy=validate_at_materialization_or_debug_boundary
slot_bounds_policy=static_slot_constant_plus_field_count_guard
storage_tag_policy=static_plan_plus_optional_debug_check
c_abi_load_writeback_helper_count=0
existing_helper_abi_unchanged=1
default_backend_direct_handle_emission=0
by_name_hako_alloc_special_case=0
lowering_open=0
native_direct_open=0
direct_load_store_open=0
implementation_open=0
optimization_open=0
selected_next=direct_slot_object_layout_pilot
winner_claim=0
replacement_active=0
hook_installed=0
global_allocator=0
summary=ok
```

## Decision

Use a tagged stable object pointer handle for the future DirectSlot object path.

This is not permission to expose a `Vec` element or `RefCell` borrow. The handle
must point to a stable `DirectSlotObjectV0` allocation whose layout is an
explicit ABI contract.

The next row may implement only the object layout pilot. LLVM lowering remains
closed until object layout, handle encode/decode, and fallback/materialization
behavior are all proven.

## Guard

```bash
bash tools/checks/k2_wide_phase296x_direct_slot_handle_resolution_contract_guard.sh
```
