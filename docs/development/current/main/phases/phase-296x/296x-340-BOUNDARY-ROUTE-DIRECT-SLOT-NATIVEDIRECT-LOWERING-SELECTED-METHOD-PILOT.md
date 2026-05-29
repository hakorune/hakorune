---
Status: Landed
Date: 2026-05-29
Scope: implement selected-method DirectSlot NativeDirect IR shape in the ny-llvmc boundary route.
Blocker: BOUNDARY-ROUTE-DIRECT-SLOT-NATIVEDIRECT-LOWERING-SELECTED-METHOD-PILOT-296X-001
Related:
  - docs/development/current/main/phases/phase-296x/296x-339-DIRECT-SLOT-NATIVEDIRECT-LOWERING-DAILY-OWNER-GAP-DIAGNOSTIC.md
  - lang/c-abi/shims/hako_llvmc_ffi_same_module_typed_object_emit.inc
---

# 296x-340 Boundary Route Direct Slot NativeDirect Lowering Selected-Method Pilot

## Purpose

Implement the first mainline exact-EXE selected-method DirectSlot NativeDirect
IR shape in the `ny-llvmc` boundary route.

This mirrors the row338 payload access shape, but applies it to the actual
daily exact-EXE owner instead of the llvmlite keep lane.

This row intentionally stops at the boundary-route IR shape. The first semantic
execution attempt exposed a separate bootstrap/materialization boundary:
`direct_slot_exact` returns positive tagged `DirectSlotObjectV0` handles, while
existing constructor/init field helpers only routed materialized negative view
handles. That owner is split to row341 so the selected hot NativeDirect lowering
does not silently become another helper fast lane.

## Contract

```text
output_contract=boundary-route-direct-slot-nativedirect-lowering-selected-method-pilot-v0
input_contract=direct-slot-nativedirect-lowering-daily-owner-gap-diagnostic-v0
implemented_owner=ny_llvmc_boundary_same_module_typed_object_emit
implemented_owner_file=lang/c-abi/shims/hako_llvmc_ffi_same_module_typed_object_emit.inc
selected_method=HakoAllocPageModel.acquire_usize/1
selected_backend=direct_slot_exact
initial_selected_method_only=1
selected_method_pilot_superseded_by_supported_storage_nativedirect=1
direct_slot_exact_only=1
llvmlite_keep_lane_changes_allowed=0
generic_direct_slot_rewrite_allowed=0
mirbuilder_changes_allowed=0
hako_source_changes_allowed=0
runtime_helper_semantics_changes_allowed=0
direct_slot_payload_formula=object_base_plus_24_plus_slot_times_16_plus_8
direct_slot_handle_decode=clear_low_tag_bit
implemented_get_lowering=payload_load_i64
implemented_set_lowering=payload_store_i64
supported_storage=i64,u64,usize,handle
unsigned_set_nonnegative_trap_preserved=1
direct_set_status_continue_branch_preserved=1
exact_status_continue_label_preserved=1
unsupported_storage_policy=existing_non_direct_route
non_selected_method_policy=existing_helper_path
silent_fallback_allowed=0
helper_load_writeback_substitution_allowed=0
typed_slot_enum_layout_exposure=0
raw_runtime_vec_pointer_exposure=0
thread_local_refcell_pointer_exposure=0
ffi_shim_rebuild_required=1
direct_slot_ir_shape_smoke=ok
exact_exe_semantic_smoke=blocked_by_direct_slot_bootstrap_materialization_boundary
blocked_owner=direct_slot_positive_handle_bootstrap_materialization
body_elapsed_positive=0
provider_activation=0
host_replacement=0
replacement_active=0
hook_installed=0
global_allocator=0
winner_claim=0
summary=ok
```

## Lowering Shape

For the initial selected-method pilot and `direct_slot_exact` backend:

```text
object_base = receiver_handle & -2
payload_addr = object_base + 24 + slot * 16 + 8
field_get = load i64, payload_addr
field_set = store i64 value, payload_addr
```

The hot path does not load the DirectSlotCell storage tag. The TypedObjectPlan
and selected-method boundary were the proof surface for this pilot. Later rows
supersede the method-name gate with fact-driven supported-storage lowering.

## Rejected Options

```text
rejected_for_row340=direct_slot_exact_helper_fallback_for_positive_handles
reason=would mix selected-method NativeDirect lowering with bootstrap/materialization compatibility

rejected_for_row340=generic_direct_slot_rewrite
reason=first boundary pilot stayed selected-method only; row343/344 supersede this with a fact-driven supported-storage surface

rejected_for_row340=runtime_helper_semantics_change
reason=NativeDirect must remove helper calls, not create another helper fast lane
```

## Follow-Up Boundary

```text
next_row=296x-341-DIRECT-SLOT-BOOTSTRAP-MATERIALIZATION-COMPATIBILITY
next_owner=typed_object_store_direct_slot_positive_handle_materialization
reason=HakoAllocPageModel.birth/4 uses generic field_set_u64_hiu before acquire_usize/1 runs
hot_selected_method_native_direct=preserved
helper_fallback_scope=bootstrap_materialization_and_non_native_regions_only
```

## Guard

```bash
bash tools/checks/k2_wide_phase296x_boundary_route_direct_slot_nativedirect_lowering_selected_method_pilot_guard.sh
```
