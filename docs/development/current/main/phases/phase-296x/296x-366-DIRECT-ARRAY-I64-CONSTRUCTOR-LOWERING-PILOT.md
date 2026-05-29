---
Status: Landed
Date: 2026-05-30
Scope: implement exact-lane DirectArrayI64 constructor lowering without changing default public ArrayBox birth.
Blocker: DIRECT-ARRAY-I64-CONSTRUCTOR-LOWERING-PILOT-296X-001
Related:
  - docs/development/current/main/phases/phase-296x/296x-365-DIRECT-ARRAY-I64-CONSTRUCTOR-LOWERING-SELECTION.md
  - src/llvm_py/instructions/newbox.py
  - src/llvm_py/instructions/mir_call/constructor_call.py
---

# 296x-366 Direct Array I64 Constructor Lowering Pilot

## Purpose

Implement the exact-lane constructor hook that emits DirectArrayI64 birth.

Default `ArrayBox` construction still emits `nyash.array.birth_h`. Exact lane
construction emits `nyash.array.direct_i64.birth_h` only when
`HAKO_ARRAY_SLOT_STORE=direct_array_i64_exact`.

## Contract

```text
output_contract=direct-array-i64-constructor-lowering-pilot-v0
input_contract=direct-array-i64-constructor-lowering-selection-v0
implemented_owner=llvm_arraybox_constructor_direct_array_birth_hook
implemented_owner_file_0=src/llvm_py/instructions/newbox.py
implemented_owner_file_1=src/llvm_py/instructions/mir_call/constructor_call.py
selected_backend=direct_array_i64_exact
selected_direct_birth_symbol=nyash.array.direct_i64.birth_h
default_public_birth_symbol=nyash.array.birth_h
default_public_birth_unchanged=1
direct_array_birth_requires_env=HAKO_ARRAY_SLOT_STORE=direct_array_i64_exact
direct_array_birth_default_emission=0
direct_array_origin_fact=resolver.direct_array_i64_ids
direct_array_origin_fact_recorded=1
public_arraybox_handle_as_direct_buffer_allowed=0
generic_arraybox_rewrite_allowed=0
runtime_helper_semantics_changes_allowed=0
selected_method_lowering_open=0
llvm_direct_load_store_open=0
native_direct_open=0
direct_load_store_open=0
silent_fallback_allowed=0
by_name_hako_alloc_special_case=0
python_unit_smoke=ok
selected_next=array_slot_nativedirect_lowering_readiness_refresh_after_constructor
implementation_open=0
optimization_open=0
winner_claim=0
replacement_active=0
hook_installed=0
global_allocator=0
summary=ok
```

## Guard

```bash
bash tools/checks/k2_wide_phase296x_direct_array_i64_constructor_lowering_pilot_guard.sh
```
