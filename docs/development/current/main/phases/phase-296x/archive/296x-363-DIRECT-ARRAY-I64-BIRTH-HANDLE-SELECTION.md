---
Status: Landed
Date: 2026-05-30
Scope: select the DirectArrayI64 birth-handle producer required before ArraySlot NativeDirect lowering can open.
Blocker: DIRECT-ARRAY-I64-BIRTH-HANDLE-SELECTION-296X-001
Related:
  - docs/development/current/main/phases/phase-296x/296x-362-ARRAY-SLOT-NATIVEDIRECT-LOWERING-SELECTED-METHOD-PILOT-PREFLIGHT.md
  - crates/nyash_kernel/src/plugin/array_direct_i64_buffer.rs
  - src/llvm_py/instructions/mir_call/constructor_call.py
  - src/llvm_py/instructions/newbox.py
---

# 296x-363 Direct Array I64 Birth Handle Selection

## Purpose

Select the missing DirectArrayI64 birth-handle producer before reopening
ArraySlot NativeDirect lowering.

The producer must create a `DirectArrayI64BufferV0` handle for the exact lane
without changing default public ArrayBox construction. Public ArrayBox semantics
remain on `nyash.array.birth_h`.

## Contract

```text
output_contract=direct-array-i64-birth-handle-selection-v0
input_contract=array-slot-nativedirect-lowering-selected-method-pilot-preflight-v0
selected_owner=direct_array_i64_birth_handle_producer
selected_runtime_owner=crates/nyash_kernel/src/plugin/array_direct_i64_buffer.rs
selected_lowering_owner_0=src/llvm_py/instructions/mir_call/constructor_call.py
selected_lowering_owner_1=src/llvm_py/instructions/newbox.py
selected_backend=direct_array_i64_exact
default_arraybox_constructor_symbol=nyash.array.birth_h
default_arraybox_constructor_unchanged=1
new_direct_array_birth_symbol_required=1
proposed_direct_array_birth_symbol=nyash.array.direct_i64.birth_h
direct_array_handle_kind=tagged_or_positive_direct_array_i64_buffer_handle
public_arraybox_handle_kind=public_arraybox_host_handle
handle_kinds_must_not_alias=1
selected_method_lowering_open=0
constructor_lowering_open_next=1
generic_arraybox_rewrite_allowed=0
runtime_helper_semantics_changes_allowed=0
public_arraybox_semantics_unchanged=1
materialized_view_boundary=explicit_public_arraybox_snapshot_handle
silent_fallback_allowed=0
by_name_hako_alloc_special_case=0
selected_next=direct_array_i64_birth_handle_pilot
implementation_open=0
optimization_open=0
llvm_lowering_open=0
native_direct_open=0
direct_load_store_open=0
winner_claim=0
replacement_active=0
hook_installed=0
global_allocator=0
summary=ok
```

## Rejected Designs

```text
rejected_design=reinterpret_public_arraybox_host_handle_as_direct_array_pointer
reason=would turn a host handle into an unsafe pointer and break public ArrayBox semantics

rejected_design=replace_nyash_array_birth_h_default
reason=default ArrayBox construction must remain public ArrayBox semantics

rejected_design=route_existing_helpers_through_direct_array_snapshot
reason=per-helper snapshot routing is closed and would reintroduce helper-boundary cost
```

## Guard

```bash
bash tools/checks/k2_wide_phase296x_direct_array_i64_birth_handle_selection_guard.sh
```
