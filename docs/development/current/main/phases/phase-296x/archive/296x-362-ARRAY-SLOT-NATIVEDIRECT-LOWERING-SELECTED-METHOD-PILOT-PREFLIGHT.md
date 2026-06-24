---
Status: Landed
Date: 2026-05-30
Scope: preflight the selected-method ArraySlot NativeDirect lowering pilot and stop before unsafe lowering because DirectArray handle production is not yet available.
Blocker: ARRAY-SLOT-NATIVEDIRECT-LOWERING-SELECTED-METHOD-PILOT-296X-001
Related:
  - docs/development/current/main/phases/phase-296x/296x-361-ARRAY-SLOT-NATIVEDIRECT-LOWERING-OWNER-SELECTION.md
  - src/llvm_py/instructions/mir_call/constructor_call.py
  - src/llvm_py/instructions/newbox.py
---

# 296x-362 Array Slot NativeDirect Lowering Selected Method Pilot Preflight

## Purpose

Preflight the first selected-method ArraySlot NativeDirect lowering pilot before
editing `collection_method_call.py`.

The pilot must not treat a public ArrayBox host handle as a DirectArray pointer.
Current ArrayBox construction still lowers to `nyash.array.birth_h`, which
returns the public ArrayBox handle path. Therefore this row closes the selected
method lowering pilot without implementation and selects the missing DirectArray
birth-handle producer as the next row.

## Contract

```text
output_contract=array-slot-nativedirect-lowering-selected-method-pilot-preflight-v0
input_contract=array-slot-nativedirect-lowering-owner-selection-v0
selected_method=HakoAllocPageModel.acquire_usize/1
selected_owner_file=src/llvm_py/instructions/mir_call/collection_method_call.py
constructor_owner_file_0=src/llvm_py/instructions/mir_call/constructor_call.py
constructor_owner_file_1=src/llvm_py/instructions/newbox.py
arraybox_constructor_symbol=nyash.array.birth_h
current_arraybox_handle_kind=public_arraybox_host_handle
required_arraybox_handle_kind=direct_array_i64_buffer_pointer_or_tagged_handle
direct_array_birth_handle_producer_available=0
unsafe_pointer_reinterpretation_risk=1
selected_method_lowering_implemented=0
selected_method_lowering_blocked=1
blocked_reason=direct_array_handle_producer_missing
direct_load_store_open=0
native_direct_open=0
llvm_lowering_open=0
helper_route_reuse_allowed=0
public_arraybox_handle_as_direct_buffer_allowed=0
silent_fallback_allowed=0
by_name_hako_alloc_special_case=0
selected_next=direct_array_i64_birth_handle_selection
implementation_open=0
optimization_open=0
winner_claim=0
replacement_active=0
hook_installed=0
global_allocator=0
summary=ok
```

## Decision

Do not open ArraySlot NativeDirect lowering yet.

The correct next row is a constructor/birth-handle selection row that decides
how exact-EXE DirectArray objects are born without changing public ArrayBox
semantics by default.

## Guard

```bash
bash tools/checks/k2_wide_phase296x_array_slot_nativedirect_lowering_selected_method_pilot_preflight_guard.sh
```
