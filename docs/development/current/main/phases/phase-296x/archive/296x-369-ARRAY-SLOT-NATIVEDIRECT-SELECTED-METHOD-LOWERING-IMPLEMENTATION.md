---
Status: Landed
Date: 2026-05-30
Scope: implement selected-method ArraySlot NativeDirect lowering for DirectArrayI64 get/set in collection_method_call.py.
Blocker: ARRAY-SLOT-NATIVEDIRECT-SELECTED-METHOD-LOWERING-IMPLEMENTATION-296X-001
Related:
  - docs/development/current/main/phases/phase-296x/296x-368-ARRAY-SLOT-NATIVEDIRECT-SELECTED-METHOD-LOWERING-GUARD-REFRESH.md
  - src/llvm_py/instructions/mir_call/collection_method_call.py
---

# 296x-369 ArraySlot NativeDirect Selected-Method Lowering Implementation

## Purpose

Implement the first selected-method ArraySlot NativeDirect lowering path.

Only `HakoAllocPageModel.acquire_usize/1` may use this lowering. Only receivers
recorded in `resolver.direct_array_i64_ids` may be treated as DirectArrayI64
handles. Public `ArrayBox` handles remain on the existing helper path.

## Contract

```text
output_contract=array-slot-nativedirect-selected-method-lowering-implementation-v0
input_contract=array-slot-nativedirect-selected-method-lowering-guard-refresh-v0
implemented_owner=llvm_collection_method_call_direct_array_nativedirect_selected_method_hook
implemented_owner_file=src/llvm_py/instructions/mir_call/collection_method_call.py
selected_method=HakoAllocPageModel.acquire_usize/1
selected_backend=direct_array_i64_exact
receiver_origin_fact=resolver.direct_array_i64_ids
receiver_origin_fact_consumed=1
public_arraybox_handle_as_direct_buffer_allowed=0
default_helper_path_preserved=1
default_backend_emission=0
generic_arraybox_rewrite_allowed=0
runtime_helper_semantics_changes_allowed=0
mirbuilder_changes_allowed=0
hako_source_changes_allowed=0
direct_array_data_offset_bytes=32
direct_array_element_size_bytes=8
direct_array_get_lowering=direct_i64_load_with_oob_zero
direct_array_set_lowering=direct_i64_store_with_append_len_update_and_oob_zero
append_policy_preserved_in_direct_path=1
oob_policy_preserved_in_direct_path=1
silent_fallback_allowed=0
legacy_retirement_now=0
legacy_retirement_policy=defer_until_direct_array_semantic_smoke_and_perf_owner_refresh
python_unit_smoke=ok
selected_method_direct_load_store_open=1
generic_direct_load_store_open=0
native_direct_open=selected_method_only
winner_claim=0
replacement_active=0
hook_installed=0
global_allocator=0
selected_next=array_slot_nativedirect_selected_method_semantic_smoke
summary=ok
```

## Notes

The implementation preserves Array semantics inside the direct path:

```text
get:
  idx < 0 or idx >= len -> 0
  otherwise direct load from data[idx]

set:
  idx < 0 -> 0
  idx > len -> 0
  idx >= capacity -> 0
  idx < len -> direct store, len unchanged
  idx == len -> direct store, len += 1
```

This row does not delete legacy helper/cache code. Those paths remain fallback,
debug, materialization, and public ArrayBox semantics until a DirectArray
semantic smoke and perf owner refresh prove they can be retired.

## Guard

```bash
bash tools/checks/k2_wide_phase296x_array_slot_nativedirect_selected_method_lowering_implementation_guard.sh
```
