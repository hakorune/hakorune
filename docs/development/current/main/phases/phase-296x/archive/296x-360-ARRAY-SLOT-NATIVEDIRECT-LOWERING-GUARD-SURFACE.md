---
Status: Landed
Date: 2026-05-30
Scope: define the selected-method ArraySlot NativeDirect lowering guard surface before implementation.
Blocker: ARRAY-SLOT-NATIVEDIRECT-LOWERING-GUARD-SURFACE-296X-001
Related:
  - docs/development/current/main/phases/phase-296x/296x-359-ARRAY-SLOT-NATIVEDIRECT-LOWERING-READINESS-INVENTORY.md
  - docs/development/current/main/phases/phase-296x/296x-350-DIRECT-ARRAY-I64-BUFFER-V0-STORAGE-PILOT.md
  - docs/development/current/main/phases/phase-296x/296x-357-DIRECT-ARRAY-I64-MATERIALIZED-VIEW-HANDLE-PILOT.md
---

# 296x-360 Array Slot NativeDirect Lowering Guard Surface

## Purpose

Define the first guard surface for ArraySlot NativeDirect lowering.

This row authorizes only a future selected-method implementation attempt. It
does not implement lowering.

## Contract

```text
output_contract=array-slot-nativedirect-lowering-guard-surface-v0
input_contract=array-slot-nativedirect-lowering-readiness-inventory-v0
selected_method=HakoAllocPageModel.acquire_usize/1
selected_backend=direct_array_i64_exact
selected_representation=NativeDirect
selected_storage_substrate=DirectArrayI64BufferV0
selected_buffer_layout=repr_c_header_trailing_i64
selected_lowering_owner=src/llvm_py/instructions/mir_call/collection_method_call.py
runtime_layout_owner=crates/nyash_kernel/src/plugin/array_direct_i64_buffer.rs
runtime_backend_owner=crates/nyash_kernel/src/plugin/array_slot_backend.rs
selected_method_only=1
selected_backend_required=1
default_backend_emission=0
direct_array_buffer_required=1
receiver_array_exact_required=1
index_i64_required=1
element_storage_i64_required=1
same_block_get_set_pair_required=1
set_uses_get_result_required=1
field_address_formula=buffer_base_plus_header_offset_plus_index_times_8
append_policy=known_but_selected_lowering_initially_requires_in_bounds_or_explicit_append_plan
oob_policy=preserve_or_reject_plan
unsupported_storage_policy=reject_plan_not_silent_fallback
unknown_receiver_policy=reject_plan
unknown_call_barrier_policy=materialize_or_reject_before_crossing
phi_barrier_policy=reject_unless_plan_has_explicit_merge_materialization
fallback_boundary=explicit_public_arraybox_snapshot_handle
fallback_boundary_required=1
silent_fallback_allowed=0
helper_load_writeback_substitution_allowed=0
arraybox_items_rwlock_exposure=0
array_slot_cache_vec_exposure=0
direct_array_helper_route_reuse_allowed=0
by_name_hako_alloc_special_case=0
planned_erased_helper_ops=2
planned_added_helper_ops=0
planned_net_helper_delta=2
planned_net_helper_delta_positive=1
implementation_open=0
optimization_open=0
llvm_lowering_open=0
native_direct_open=0
direct_load_store_open=0
selected_next=array_slot_nativedirect_lowering_owner_selection
winner_claim=0
replacement_active=0
hook_installed=0
global_allocator=0
summary=ok
```

## Implementation Constraints

The future implementation row must not introduce a generic ArrayBox rewrite. It
may only target the selected method and selected backend. If any required fact
is missing, the selected method must remain on the existing helper path and the
row must report no implementation.

The implementation must also preserve the fallback boundary:

```text
hot path:
  DirectArrayI64BufferV0 NativeDirect access

compatibility boundary:
  explicit public ArrayBox snapshot handle

forbidden:
  hidden helper load/writeback substitution
  per-helper snapshot fallback
  direct use of ArrayBox.items / RwLock / ArrayStorage internals in LLVM lowering
  direct use of diagnostic ArraySlotCacheEntry.values Vec in LLVM lowering
```

## Guard

```bash
bash tools/checks/k2_wide_phase296x_array_slot_nativedirect_lowering_guard_surface_guard.sh
```
