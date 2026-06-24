---
Status: Landed
Date: 2026-05-29
Scope: let DirectSlot positive handles participate in bootstrap/materialization helper paths while preserving selected hot NativeDirect lowering.
Blocker: DIRECT-SLOT-BOOTSTRAP-MATERIALIZATION-COMPATIBILITY-296X-001
Related:
  - docs/development/current/main/phases/phase-296x/296x-340-BOUNDARY-ROUTE-DIRECT-SLOT-NATIVEDIRECT-LOWERING-SELECTED-METHOD-PILOT.md
  - crates/nyash_kernel/src/exports/typed_object_store.rs
  - crates/nyash_kernel/src/exports/typed_object_pinned_arena.rs
---

# 296x-341 DirectSlot Bootstrap Materialization Compatibility

## Purpose

Make `direct_slot_exact` usable by exact-EXE semantic execution after row340.

Row340 proved the `ny-llvmc` boundary route can emit selected-method direct
payload load/store IR for `HakoAllocPageModel.acquire_usize/1`. The first
semantic run then failed before that method ran: constructors such as
`HakoAllocPageModel.birth/4` still use existing field helpers to initialize the
new object, but `direct_slot_exact` returns a positive tagged
`DirectSlotObjectV0` handle and those helpers only accepted negative
materialized view handles.

This row makes the existing helper path an explicit bootstrap/materialization
compatibility path for positive DirectSlot handles. It is not the hot
NativeDirect owner.

## Contract

```text
output_contract=direct-slot-bootstrap-materialization-compatibility-v0
input_contract=boundary-route-direct-slot-nativedirect-lowering-selected-method-pilot-v0
implemented_owner=typed_object_store_direct_slot_positive_handle_compatibility
implemented_owner_files=crates/nyash_kernel/src/exports/typed_object_store.rs,crates/nyash_kernel/src/exports/typed_object_pinned_arena.rs
selected_backend=direct_slot_exact
selected_hot_method=HakoAllocPageModel.acquire_usize/1
hot_selected_method_native_direct=preserved
compatibility_scope=bootstrap_materialization_and_non_native_regions
direct_slot_cell_primary_storage=1
typed_slot_materialized_view_policy=fallback_debug_compatibility_view
positive_direct_handle_generic_helper_get_supported=1
positive_direct_handle_generic_helper_set_supported=1
positive_direct_handle_exact_slot_helper_get_supported=1
positive_direct_handle_exact_slot_helper_set_supported=1
helper_fallback_hot_path_owner=0
generic_direct_slot_lowering_allowed=0
mirbuilder_changes_allowed=0
hako_source_changes_allowed=0
runtime_helper_compatibility_change=explicit
silent_fallback_allowed=0
materialized_snapshot_reads_primary_direct_cells=1
typed_slot_enum_layout_exposure=0
raw_runtime_vec_pointer_exposure=0
thread_local_refcell_pointer_exposure=0
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

## Compatibility Rule

For `direct_slot_exact`, a positive DirectSlot handle may enter existing
bootstrap or non-native helper paths. Those helpers must read and write
`DirectSlotCellV0` raw cells as the primary storage. Materialized `TypedSlot`
objects are compatibility snapshots/views only.

The selected hot method remains helper-free through row340 direct payload
lowering. This row only prevents setup and fallback paths from trapping before
or after that hot region.

## Non-Goals

```text
rejected=generic_native_direct_lowering
reason=row341 is runtime compatibility, not a broad compiler transform

rejected=helper_load_writeback_substitution_for_selected_hot_method
reason=row340 already emits direct payload load/store for the selected method

rejected=materialized_view_as_primary_storage
reason=would leave direct payload stores and helper snapshots inconsistent
```

## Guard

```bash
bash tools/checks/k2_wide_phase296x_direct_slot_bootstrap_materialization_compatibility_guard.sh
```

## Evidence

```text
semantic_proof_summary=ok
representative_sample_body_elapsed_ns=122000000
representative_sample_external_elapsed_ms=120
allocation_count=524288
free_count=524288
select_page_single_fast_path_count=524288
select_page_single_fallback_count=0
release_known_page_fast_path_count=524288
release_known_page_fallback_count=0
winner_claim=0
summary=ok
```
