---
Status: Landed
Date: 2026-05-29
Scope: define the selected-method DirectSlot NativeDirect lowering guard surface before implementation.
Blocker: DIRECT-SLOT-NATIVEDIRECT-LOWERING-GUARD-SURFACE-296X-001
Related:
  - docs/development/current/main/phases/phase-296x/296x-333-DIRECT-SLOT-NATIVEDIRECT-LOWERING-READINESS-INVENTORY.md
  - docs/development/current/main/phases/phase-296x/296x-323-DIRECT-SLOT-OBJECT-LAYOUT-PILOT.md
  - docs/development/current/main/phases/phase-296x/296x-331-DIRECT-SLOT-MATERIALIZED-VIEW-HANDLE-PILOT.md
---

# 296x-334 Direct Slot NativeDirect Lowering Guard Surface

## Purpose

Define the first guard surface for DirectSlot NativeDirect lowering.

This row authorizes only a future selected-method implementation attempt. It
does not implement lowering.

## Contract

```text
output_contract=direct-slot-nativedirect-lowering-guard-surface-v0
input_contract=direct-slot-nativedirect-lowering-readiness-inventory-v0
selected_method=HakoAllocPageModel.acquire_usize/1
selected_backend=direct_slot_exact
selected_representation=NativeDirect
selected_storage_substrate=DirectSlotObjectV0
selected_cell_layout=DirectSlotCellV0
selected_handle_kind=tagged_stable_direct_slot_object_pointer
selected_lowering_owner=src/llvm_py/instructions/field_access.py
runtime_layout_owner=crates/nyash_kernel/src/exports/typed_object_pinned_arena.rs
selected_method_only=1
selected_backend_required=1
default_backend_emission=0
direct_handle_required=1
slot_constant_required=1
storage_tag_known_required=1
field_address_formula=object_base_plus_header_offset_plus_slot_times_16
allowed_storage_tags=i64,u64,handle
unsupported_storage_policy=fail_or_keep_helper_before_plan_not_silent
unknown_receiver_policy=reject_plan
unknown_field_plan_policy=reject_plan
unknown_call_barrier_policy=materialize_or_reject_before_crossing
phi_barrier_policy=reject_unless_plan_has_explicit_merge_materialization
return_barrier_policy=materialize_view_only_if_needed
fallback_boundary=explicit_materialized_view_handle
fallback_boundary_required=1
silent_fallback_allowed=0
helper_load_writeback_substitution_allowed=0
raw_runtime_vec_pointer_exposure=0
thread_local_refcell_pointer_exposure=0
typed_slot_enum_layout_exposure=0
by_name_hako_alloc_special_case=0
planned_erased_helper_ops=21
planned_added_helper_ops=0
planned_net_helper_delta=21
planned_net_helper_delta_positive=1
implementation_open=0
optimization_open=0
llvm_lowering_open=0
native_direct_open=0
direct_load_store_open=0
selected_next=direct_slot_nativedirect_lowering_owner_selection
winner_claim=0
replacement_active=0
hook_installed=0
global_allocator=0
summary=ok
```

## Implementation Constraints

The future implementation row must not introduce a generic DirectSlot rewrite.
It may only target the selected method and selected backend. If any required
fact is missing, the selected method must remain on the existing helper path and
the row must report no implementation.

The implementation must also preserve the fallback boundary:

```text
hot path:
  DirectSlotObjectV0 / DirectSlotCellV0 NativeDirect access

compatibility boundary:
  explicit materialized view handle

forbidden:
  hidden helper load/writeback substitution
  per-helper snapshot fallback
  direct use of TypedSlot enum layout in LLVM lowering
```

## Guard

```bash
bash tools/checks/k2_wide_phase296x_direct_slot_nativedirect_lowering_guard_surface_guard.sh
```
