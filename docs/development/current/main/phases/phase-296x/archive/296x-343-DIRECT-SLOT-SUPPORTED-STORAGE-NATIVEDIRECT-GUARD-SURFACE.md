---
Status: Landed
Date: 2026-05-29
Scope: define the fact-driven DirectSlot NativeDirect guard surface for supported typed-object storage.
Blocker: DIRECT-SLOT-SUPPORTED-STORAGE-NATIVEDIRECT-GUARD-SURFACE-296X-001
Related:
  - docs/development/current/main/phases/phase-296x/296x-342-POST-DIRECT-SLOT-BOOTSTRAP-OWNER-REFRESH.md
  - lang/c-abi/shims/hako_llvmc_ffi_same_module_typed_object_emit.inc
---

# 296x-343 DirectSlot Supported Storage NativeDirect Guard Surface

## Purpose

Close the row340 selected-method pilot boundary and define the next
fact-driven lowering surface.

Row342 showed that after DirectSlot bootstrap/materialization compatibility,
legacy typed-object field helpers dominate the exact-EXE sample again. The next
row should not add another helper fast lane. It should broaden DirectSlot
NativeDirect lowering for fields whose receiver type, runtime slot, and storage
class are already proven by `TypedObjectPlan`.

This row is docs/guard only. It opens implementation only for a follow-up row.

## Contract

```text
output_contract=direct-slot-supported-storage-nativedirect-guard-surface-v0
input_contract=direct-slot-post-bootstrap-owner-refresh-v0
selected_owner=ny_llvmc_boundary_same_module_typed_object_emit
selected_owner_file=lang/c-abi/shims/hako_llvmc_ffi_same_module_typed_object_emit.inc
selected_backend=direct_slot_exact
selection_kind=fact_driven_supported_storage
selected_method_only=0
by_name_hako_alloc_special_case=0
required_receiver_fact=typed_object_binding
required_slot_fact=typed_object_plan_runtime_slot
required_storage_fact=typed_object_plan_storage
supported_storage=i64,u64,usize,handle
unsupported_storage_policy=existing_helper_route
unsupported_narrow_integer_direct_store=0
legacy_field_helper_internal_fast_lane=0
runtime_helper_semantics_change=0
mirbuilder_changes_allowed=0
hako_source_changes_allowed=0
direct_slot_payload_formula=object_base_plus_24_plus_slot_times_16_plus_8
unsigned_set_nonnegative_trap_required=1
exact_status_continue_label_required=1
silent_fallback_allowed=0
ffi_shim_rebuild_required=1
implementation_open=0
optimization_open=0
provider_activation=0
host_replacement=0
replacement_active=0
hook_installed=0
global_allocator=0
winner_claim=0
summary=ok
```

## Guard Surface

The follow-up implementation may lower typed-object field get/set to direct
payload load/store only when all of these facts are present:

```text
HAKO_TYPED_OBJECT_STORE=direct_slot_exact
get_typed_object_binding(receiver)=present
typed_object_plan_field_runtime_slot_with_storage(...)=present
storage in {i64,u64,usize,handle}
slot is a constant runtime slot from TypedObjectPlan
```

The implementation must not infer layout from `.hako` class names, method
names, or field names. `HakoAlloc*` is just a workload user of the generic
DirectSlot facts.

Unsupported storage stays on the existing helper route. That keeps narrow
integer range checks, non-direct storage, weak fields, and any unknown storage
under the current helper semantics.

## Non-Goals

```text
rejected=helper_internal_fast_lane
reason=helper internals are no longer the selected owner; direct storage facts are
the selected seam

rejected=by_name_hako_alloc_special_case
reason=the compiler must consume typed-object facts, not allocator-specific names

rejected=unsupported_narrow_integer_direct_store
reason=narrow signed/unsigned storage still needs the existing range semantics

rejected=mirbuilder_transform
reason=this is a ny-llvmc boundary lowering surface, not a MIR semantics change
```

## Guard

```bash
bash tools/checks/k2_wide_phase296x_direct_slot_supported_storage_nativedirect_guard_surface_guard.sh
```
