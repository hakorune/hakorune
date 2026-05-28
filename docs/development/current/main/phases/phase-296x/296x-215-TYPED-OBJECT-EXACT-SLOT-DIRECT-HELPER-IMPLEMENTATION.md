---
Status: Landed
Date: 2026-05-29
Scope: implement the selected exact-lane typed-object slot direct helper seam.
Blocker: TYPED-OBJECT-EXACT-SLOT-DIRECT-HELPER-IMPLEMENTATION-296X-001
Related:
  - docs/development/current/main/phases/phase-296x/296x-214-TYPED-OBJECT-EXACT-SLOT-DIRECT-HELPER-SELECTION.md
  - docs/development/current/main/phases/phase-296x/296x-213-TYPED-OBJECT-FIELD-HELPER-SUBOWNER-REFRESH.md
---

# 296x-215 Typed Object Exact Slot Direct Helper Implementation

## Purpose

Implement the row214 selected seam without changing the default typed-object
helper ABI.

This row adds separate exact-lane symbols for direct-compatible i64/u64/handle
slots and teaches exact-EXE lowering to select those symbols only under the
explicit exact helper gate.

## Boundary

```text
output_contract=typed-object-exact-slot-direct-helper-implementation-v0
input_contract=typed-object-exact-slot-direct-helper-selection-v0
selected_owner_family=typed_object_exact_slot_direct_helper

default_helper_abi_unchanged=1
generic_helper_codepath_unchanged=1
new_helper_symbols=separate
new_symbol_count=6

runtime_helper_env_check=0
runtime_helper_safe_mutex_fallback=0
runtime_helper_memory_safety_bounds=preserved

lowering_gate_0=HAKO_TYPED_OBJECT_STORE_single_thread_exact
lowering_gate_1=HAKO_TYPED_OBJECT_EXACT_SLOT_HELPER_1
lowering_gate_2=exact_field_plan_for_receiver_present
lowering_gate_3=slot_constant

hako_alloc_by_name_special_case=0
mir_residence_transform=0
generic_cse=0
provider_activation=0
allocator_replacement=0
hook_installed=0
global_allocator=0
winner_claim=0
```

## Symbols

```text
implemented_symbol_0=nyash.object.exact_slot_get_i64_hii
implemented_symbol_1=nyash.object.exact_slot_set_i64_hii
implemented_symbol_2=nyash.object.exact_slot_get_u64_hii
implemented_symbol_3=nyash.object.exact_slot_set_u64_hiu
implemented_symbol_4=nyash.object.exact_slot_get_handle_hii
implemented_symbol_5=nyash.object.exact_slot_set_handle_hii
```

## Runtime Contract

The exact helpers directly access the single-thread typed-object TLS store.
They must not select a backend, lock the SafeMutex store, or silently fall back
to the generic helper.

```text
single_thread_tls_store_direct=1
safe_mutex_store_direct=0
unsupported_storage_returns_failure=1
wrong_handle_or_slot_returns_failure_or_zero=1
```

## Lowering Contract

```text
exact_lane_helper_emission_requires_env=1
default_exact_helper_emission=0
unsupported_storage_fallback_reported=1
direct_storage_allowed_0=i64
direct_storage_allowed_1=u64
direct_storage_allowed_2=usize_if_target_pointer_width_64
direct_storage_allowed_3=handle
```

The first implementation updates C-ABI exact lowering. Python LLVM compat may
select the same symbols when the same gate is present, but default Python
lowering must keep existing helpers.

## Closeout Target

```text
safe_mutex_default_smoke=ok
single_thread_exact_existing_helper_smoke=ok
single_thread_exact_direct_helper_smoke=ok
exact_lane_helper_emission_count_positive=1
default_exact_helper_emission=0
summary=ok
```

Guard:

```bash
bash tools/checks/k2_wide_phase296x_typed_object_exact_slot_direct_helper_implementation_guard.sh
```
