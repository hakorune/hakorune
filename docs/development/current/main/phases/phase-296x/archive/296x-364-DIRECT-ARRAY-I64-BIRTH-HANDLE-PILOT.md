---
Status: Landed
Date: 2026-05-30
Scope: implement the separate DirectArrayI64 birth-handle producer while keeping public ArrayBox birth unchanged.
Blocker: DIRECT-ARRAY-I64-BIRTH-HANDLE-PILOT-296X-001
Related:
  - docs/development/current/main/phases/phase-296x/296x-363-DIRECT-ARRAY-I64-BIRTH-HANDLE-SELECTION.md
  - crates/nyash_kernel/src/plugin/array_direct_i64_buffer.rs
---

# 296x-364 Direct Array I64 Birth Handle Pilot

## Purpose

Implement a separate DirectArrayI64 birth-handle producer.

This row does not open ArraySlot NativeDirect lowering. It only creates a
distinct DirectArray handle kind so a future lowering row cannot accidentally
reinterpret a public ArrayBox host handle as a DirectArray buffer pointer.

## Contract

```text
output_contract=direct-array-i64-birth-handle-pilot-v0
input_contract=direct-array-i64-birth-handle-selection-v0
implemented_owner=direct_array_i64_birth_handle_producer
implemented_owner_file=crates/nyash_kernel/src/plugin/array_direct_i64_buffer.rs
implemented_symbol=nyash.array.direct_i64.birth_h
default_arraybox_constructor_symbol=nyash.array.birth_h
default_arraybox_constructor_unchanged=1
direct_array_handle_kind=tagged_stable_direct_array_i64_buffer_pointer
direct_array_handle_tag=3
direct_array_default_capacity=64
direct_array_object_storage=thread_local_direct_array_i64_objects
direct_array_buffer_layout=DirectArrayI64BufferV0
public_arraybox_handle_kind=public_arraybox_host_handle
handle_kinds_do_not_alias=1
host_handle_lookup_for_direct_array_handle=none
public_arraybox_handle_as_direct_buffer_allowed=0
constructor_lowering_changed=0
selected_method_lowering_open=0
llvm_lowering_open=0
native_direct_open=0
direct_load_store_open=0
runtime_helper_semantics_change=0
silent_fallback_allowed=0
by_name_hako_alloc_special_case=0
birth_handle_smoke=ok
default_arraybox_birth_smoke=ok
selected_next=direct_array_i64_constructor_lowering_selection
implementation_open=0
optimization_open=0
winner_claim=0
replacement_active=0
hook_installed=0
global_allocator=0
summary=ok
```

## Legacy Removal Notes

This row deliberately keeps `nyash.array.birth_h` as the public ArrayBox birth
route. Later cleanup may remove diagnostic `single_thread_exact` helper-cache
lanes after DirectArray NativeDirect lowering owns the hot path, but that is a
separate closeout row. The legacy removal condition is:

```text
direct_array_native_lowering_semantic_smoke=ok
perf_owner_no_longer_array_slot_helper_cache=1
public_arraybox_semantics_unchanged=1
```

## Guard

```bash
bash tools/checks/k2_wide_phase296x_direct_array_i64_birth_handle_pilot_guard.sh
```
