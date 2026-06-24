---
Status: Landed
Date: 2026-05-29
Scope: implement the first selected-method DirectSlot NativeDirect lowering pilot.
Blocker: DIRECT-SLOT-NATIVEDIRECT-LOWERING-SELECTED-METHOD-PILOT-296X-001
Related:
  - docs/development/current/main/phases/phase-296x/296x-334-DIRECT-SLOT-NATIVEDIRECT-LOWERING-GUARD-SURFACE.md
  - docs/development/current/main/phases/phase-296x/296x-335-DIRECT-SLOT-NATIVEDIRECT-LOWERING-OWNER-SELECTION.md
  - docs/development/current/main/phases/phase-296x/296x-337-DIRECT-SLOT-USIZE-CELL-STORAGE-COMPATIBILITY-PILOT.md
  - src/llvm_py/instructions/field_access.py
---

# 296x-338 Direct Slot NativeDirect Lowering Selected-Method Pilot

## Purpose

Implement the first helper-free DirectSlot NativeDirect lowering pilot for one
selected method:

```text
HakoAllocPageModel.acquire_usize/1
```

The pilot is lowerer-only. It does not change MIRBuilder, `.hako` source,
runtime helper semantics, or public helper ABI.

## Contract

```text
output_contract=direct-slot-nativedirect-lowering-selected-method-pilot-v0
input_contract=direct-slot-usize-cell-storage-compatibility-pilot-v0
implemented_owner=llvm_field_access_direct_slot_nativedirect_selected_method_hook
implemented_owner_file=src/llvm_py/instructions/field_access.py
selected_method=HakoAllocPageModel.acquire_usize/1
selected_backend=direct_slot_exact
selected_method_only=1
direct_slot_exact_only=1
default_backend_emission=0
generic_direct_slot_rewrite_allowed=0
mirbuilder_changes_allowed=0
hako_source_changes_allowed=0
runtime_helper_semantics_changes_allowed=0
direct_slot_payload_formula=object_base_plus_24_plus_slot_times_16_plus_8
direct_slot_handle_decode=clear_low_tag_bit
implemented_get_lowering=payload_load_i64
implemented_set_lowering=payload_store_i64
supported_storage=i64,u64,usize,handle
unsupported_storage_policy=fail_fast_in_selected_method
non_selected_method_policy=existing_helper_path
silent_fallback_allowed=0
helper_load_writeback_substitution_allowed=0
typed_slot_enum_layout_exposure=0
raw_runtime_vec_pointer_exposure=0
thread_local_refcell_pointer_exposure=0
selected_method_get_smoke=ok
selected_method_set_smoke=ok
non_selected_method_helper_smoke=ok
winner_claim=0
replacement_active=0
hook_installed=0
global_allocator=0
summary=ok
```

## Lowering Shape

For the selected method and `direct_slot_exact` backend only:

```text
object_base = receiver_handle & -2
payload_addr = object_base + 24 + slot * 16 + 8
field_get = load i64, payload_addr
field_set = store i64 value, payload_addr
```

The `storage_tag` field is not loaded in the hot path. The compiler-side exact
field plan is the guard that proves slot and storage shape. Explicit
materialized view handles remain the fallback/observer boundary.

## Rejected Options

```text
rejected=generic_direct_slot_field_rewrite
reason=first pilot must be selected-method only

rejected=helper_load_writeback_substitution
reason=prior ResidentScalar attempts proved zero net helper delta

rejected=runtime_helper_semantics_change
reason=NativeDirect must remove helper calls, not make another helper fast lane
```

## Guard

```bash
bash tools/checks/k2_wide_phase296x_direct_slot_nativedirect_lowering_selected_method_pilot_guard.sh
```
