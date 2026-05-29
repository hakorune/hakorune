---
Status: Landed
Date: 2026-05-30
Scope: select the constructor lowering seam that may emit DirectArrayI64 birth for the exact lane.
Blocker: DIRECT-ARRAY-I64-CONSTRUCTOR-LOWERING-SELECTION-296X-001
Related:
  - docs/development/current/main/phases/phase-296x/296x-364-DIRECT-ARRAY-I64-BIRTH-HANDLE-PILOT.md
  - src/llvm_py/instructions/mir_call/constructor_call.py
  - src/llvm_py/instructions/newbox.py
---

# 296x-365 Direct Array I64 Constructor Lowering Selection

## Purpose

Select the constructor lowering seam for exact-lane DirectArray birth.

Default `new ArrayBox` / `ArrayBox.birth()` behavior must stay public ArrayBox.
The future implementation may only emit `nyash.array.direct_i64.birth_h` when
an explicit DirectArray exact-lane gate is present.

## Contract

```text
output_contract=direct-array-i64-constructor-lowering-selection-v0
input_contract=direct-array-i64-birth-handle-pilot-v0
selected_owner=llvm_arraybox_constructor_direct_array_birth_hook
selected_owner_file_0=src/llvm_py/instructions/newbox.py
selected_owner_file_1=src/llvm_py/instructions/mir_call/constructor_call.py
selected_backend=direct_array_i64_exact
selected_direct_birth_symbol=nyash.array.direct_i64.birth_h
default_public_birth_symbol=nyash.array.birth_h
default_public_birth_unchanged=1
direct_array_birth_requires_env=HAKO_ARRAY_SLOT_STORE=direct_array_i64_exact
direct_array_birth_requires_exact_lane=1
direct_array_birth_requires_arraybox_newbox=1
direct_array_birth_default_emission=0
public_arraybox_handle_as_direct_buffer_allowed=0
generic_arraybox_rewrite_allowed=0
runtime_helper_semantics_changes_allowed=0
selected_method_lowering_open=0
llvm_lowering_open=0
native_direct_open=0
direct_load_store_open=0
silent_fallback_allowed=0
by_name_hako_alloc_special_case=0
selected_next=direct_array_i64_constructor_lowering_pilot
implementation_open=0
optimization_open=0
winner_claim=0
replacement_active=0
hook_installed=0
global_allocator=0
summary=ok
```

## Legacy Removal Notes

This selection creates a future off-ramp for diagnostic `single_thread_exact`
Array helper cache lanes. Removal is only allowed after DirectArray constructor
lowering plus selected NativeDirect lowering pass semantic proof and perf owner
refresh shows the helper cache is no longer a hot owner.

## Guard

```bash
bash tools/checks/k2_wide_phase296x_direct_array_i64_constructor_lowering_selection_guard.sh
```
