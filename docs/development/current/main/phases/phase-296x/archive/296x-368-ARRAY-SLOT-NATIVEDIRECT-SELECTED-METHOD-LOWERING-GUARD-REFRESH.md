---
Status: Landed
Date: 2026-05-30
Scope: refresh the selected-method ArraySlot NativeDirect lowering guard with DirectArray origin facts before implementation.
Blocker: ARRAY-SLOT-NATIVEDIRECT-SELECTED-METHOD-LOWERING-GUARD-REFRESH-296X-001
Related:
  - docs/development/current/main/phases/phase-296x/296x-360-ARRAY-SLOT-NATIVEDIRECT-LOWERING-GUARD-SURFACE.md
  - docs/development/current/main/phases/phase-296x/296x-361-ARRAY-SLOT-NATIVEDIRECT-LOWERING-OWNER-SELECTION.md
  - docs/development/current/main/phases/phase-296x/296x-367-ARRAY-SLOT-NATIVEDIRECT-LOWERING-READINESS-REFRESH-AFTER-CONSTRUCTOR.md
---

# 296x-368 ArraySlot NativeDirect Selected-Method Lowering Guard Refresh

## Purpose

Refresh the implementation guard for the first selected-method ArraySlot
NativeDirect lowering attempt.

The implementation owner remains `collection_method_call.py`. The new fact from
row366 is `resolver.direct_array_i64_ids`: only receivers in that set may be
treated as DirectArrayI64 handles. Public `ArrayBox` handles must keep the
existing helper path.

This row is docs/guard only. It does not implement direct load/store lowering.

## Contract

```text
output_contract=array-slot-nativedirect-selected-method-lowering-guard-refresh-v0
input_contract=array-slot-nativedirect-lowering-readiness-refresh-after-constructor-v0
selected_method=HakoAllocPageModel.acquire_usize/1
selected_owner=llvm_collection_method_call_direct_array_nativedirect_selected_method_hook
selected_owner_file=src/llvm_py/instructions/mir_call/collection_method_call.py
selected_backend=direct_array_i64_exact
selected_representation=NativeDirect
selected_storage_substrate=DirectArrayI64BufferV0
selected_direct_birth_symbol=nyash.array.direct_i64.birth_h
default_public_birth_symbol=nyash.array.birth_h
receiver_origin_fact=resolver.direct_array_i64_ids
receiver_origin_fact_required=1
receiver_origin_must_be_direct_array=1
public_arraybox_handle_as_direct_buffer_allowed=0
selected_method_only=1
direct_array_i64_exact_only=1
default_backend_emission=0
arraybox_get_selected_method_direct_load_allowed=1
arraybox_set_selected_method_direct_store_allowed=1
arraybox_push_direct_lowering_allowed=0
arraybox_len_direct_lowering_allowed=0
generic_arraybox_rewrite_allowed=0
runtime_helper_semantics_changes_allowed=0
mirbuilder_changes_allowed=0
hako_source_changes_allowed=0
direct_array_helper_route_reuse_allowed=0
helper_load_writeback_substitution_allowed=0
arraybox_items_rwlock_exposure=0
array_slot_cache_vec_exposure=0
by_name_hako_alloc_special_case=0
field_address_formula=buffer_base_plus_header_offset_plus_index_times_8
header_offset_bytes=32
element_size_bytes=8
element_storage_i64_required=1
index_i64_required=1
oob_policy=selected_method_plan_must_preserve_or_reject
append_policy=selected_method_plan_must_preserve_or_reject
fallback_boundary=explicit_public_arraybox_snapshot_handle
fallback_boundary_required=1
silent_fallback_allowed=0
planned_erased_helper_ops=2
planned_added_helper_ops=0
planned_net_helper_delta=2
planned_net_helper_delta_positive=1
implementation_open=0
optimization_open=0
llvm_lowering_open=0
native_direct_open=0
direct_load_store_open=0
selected_next=array_slot_nativedirect_selected_method_lowering_implementation
winner_claim=0
replacement_active=0
hook_installed=0
global_allocator=0
summary=ok
```

## Implementation Boundary

The next row may add a narrow branch in
`src/llvm_py/instructions/mir_call/collection_method_call.py`.

Allowed:

```text
if selected backend is direct_array_i64_exact
and receiver_vid is in resolver.direct_array_i64_ids
and method is get or set
and the selected-method guard facts hold:
  emit DirectArrayI64BufferV0 load/store shape
```

Forbidden:

```text
public ArrayBox host handle reinterpretation
generic ArrayBox rewrite
helper load/writeback substitution
runtime helper semantic changes
MIRBuilder changes
.hako source changes
silent fallback from selected direct plan
legacy helper/cache deletion
```

Legacy helper/cache retirement remains deferred until a DirectArray semantic
smoke and post-keeper perf owner refresh prove those paths are obsolete.

## Guard

```bash
bash tools/checks/k2_wide_phase296x_array_slot_nativedirect_selected_method_lowering_guard_refresh_guard.sh
```
