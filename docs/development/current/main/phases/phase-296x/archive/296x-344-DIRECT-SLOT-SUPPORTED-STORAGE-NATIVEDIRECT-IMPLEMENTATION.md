---
Status: Landed
Date: 2026-05-29
Scope: implement fact-driven DirectSlot NativeDirect lowering for supported typed-object storage in the ny-llvmc boundary route.
Blocker: DIRECT-SLOT-SUPPORTED-STORAGE-NATIVEDIRECT-IMPLEMENTATION-296X-001
Related:
  - docs/development/current/main/phases/phase-296x/296x-343-DIRECT-SLOT-SUPPORTED-STORAGE-NATIVEDIRECT-GUARD-SURFACE.md
  - lang/c-abi/shims/hako_llvmc_ffi_same_module_typed_object_emit.inc
---

# 296x-344 DirectSlot Supported Storage NativeDirect Implementation

## Purpose

Implement the row343 guard surface by removing the row340 selected-method name
gate and lowering supported typed-object field access to DirectSlot payload
loads/stores whenever `TypedObjectPlan` already proves receiver binding,
constant runtime slot, and supported storage.

This keeps `.hako` allocator semantics unchanged. The compiler only changes the
execution representation for proven DirectSlot storage.

## Contract

```text
output_contract=direct-slot-supported-storage-nativedirect-implementation-v0
input_contract=direct-slot-supported-storage-nativedirect-guard-surface-v0
implemented_owner=ny_llvmc_boundary_same_module_typed_object_emit
implemented_owner_file=lang/c-abi/shims/hako_llvmc_ffi_same_module_typed_object_emit.inc
selected_backend=direct_slot_exact
selection_kind=fact_driven_supported_storage
selected_method_only=0
selected_method_name_gate_removed=1
by_name_hako_alloc_special_case=0
required_receiver_fact=typed_object_binding
required_slot_fact=typed_object_plan_runtime_slot
required_storage_fact=typed_object_plan_storage
supported_storage=i64,u64,usize,handle
implemented_get_lowering=payload_load_i64
implemented_set_lowering=payload_store_i64
unsupported_storage_policy=existing_helper_route
unsupported_narrow_integer_direct_store=0
legacy_field_helper_internal_fast_lane=0
runtime_helper_semantics_change=0
mirbuilder_changes_allowed=0
hako_source_changes_allowed=0
direct_slot_payload_formula=object_base_plus_24_plus_slot_times_16_plus_8
unsigned_set_nonnegative_trap_preserved=1
exact_status_continue_label_preserved=1
silent_fallback_allowed=0
ffi_shim_rebuild_required=1
direct_slot_ir_shape_smoke=ok
exact_exe_semantic_smoke=ok
provider_activation=0
host_replacement=0
replacement_active=0
hook_installed=0
global_allocator=0
winner_claim=0
summary=ok
```

## Implementation Rule

The direct payload path is selected by facts only:

```text
backend=direct_slot_exact
receiver has typed-object binding
field resolves to TypedObjectPlan runtime slot + storage
storage is i64/u64/usize/handle
```

No `.hako` class name, method name, or field name is part of the selection.
Unsupported storage remains on the existing helper path.

## Guard

```bash
bash tools/checks/k2_wide_phase296x_direct_slot_supported_storage_nativedirect_implementation_guard.sh
```
