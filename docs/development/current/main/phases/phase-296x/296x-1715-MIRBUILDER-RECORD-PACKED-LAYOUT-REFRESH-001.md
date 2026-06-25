---
Status: Landed
Date: 2026-06-25
Card: MIRBUILDER-RECORD-PACKED-LAYOUT-REFRESH-001
---

# MIRBUILDER-RECORD-PACKED-LAYOUT-REFRESH-001

## Summary

`RecordAndPackedLayoutRefresh` is now a source-derived PlanOnly capability for
the prepared-state minimal MirBuilder path. The slice owns only
`refresh_module_record_and_packed_layout_plans(&mut module)` and the ordered
helper chain inside that entry point. It does not claim typed-object refresh,
direct-state refresh, all-functions PHI materialization, full finalize,
generated Hako, backend routes, ABI changes, runtime fallback, or source
selfhost.

## Source Authority

- `src/mir/builder/module_lifecycle.rs::MirBuilder::finalize_module`
- `src/mir/semantic_refresh.rs::refresh_module_record_and_packed_layout_plans`
- Predecessor plan:
  `docs/development/current/main/design/fixtures/rust-lifecycle/mirbuilder-module-metadata-publication-plan-v0.json`

## Refresh Contract

```text
entrypoint = refresh_module_record_and_packed_layout_plans
timing = AfterModuleMetadataPublicationBeforeTypedObjectRefresh
steps =
  refresh_module_record_layout_plans
  refresh_module_array_record_storage_plans
  refresh_module_array_record_autouse_eligibility_plans
  refresh_module_array_record_materialization_boundary_plans
  refresh_module_array_record_packed_autouse_pilot_plans
  refresh_module_source_packed_array_autouse_pilot_plans
  refresh_module_source_packed_array_direct_read_consumption_plans
  refresh_module_hako_alloc_aligned_small_packed_store_pilot_plans
  refresh_module_hako_alloc_huge_page_packed_store_pilot_plans
```

## Artifacts

- `tools/rust_lifecycle/mirbuilder_record_packed_layout_refresh.py`
- `docs/development/current/main/design/fixtures/rust-lifecycle/mirbuilder-record-packed-layout-refresh-plan-v0.json`
- `tools/checks/rust_lifecycle_mirbuilder_record_packed_layout_refresh_guard.sh`

## Derived Frontier Result

The minimal execution path analyzer now marks
`finalize_module.record_packed_layout_refresh` as `Available`.

The next derived unsupported edge is:

```text
edge_id: finalize_module.typed_object_plan_refresh
callsite: MirBuilder::finalize_module -> refresh_module_typed_object_plans
deny_reason: UnsupportedDirectShape
deny_detail: TypedObjectPlanRefreshRequired
semantic_owner: MirBuilder::finalize_module typed object plan refresh
next_slice_token: MIRBUILDER-TYPED-OBJECT-PLAN-REFRESH-001
```

## Non-Claims

```text
typed_object_plan_refresh = 0
direct_state_plan_refresh = 0
full_semantic_refresh = 0
all_functions_phi_materialization = 0
full_finalize_module = 0
generated_hako_artifact = 0
backend_route_changed = 0
abi_changed = 0
runtime_fallback = 0
mainline_selected = 0
```

## Acceptance

```text
python3 -m py_compile tools/rust_lifecycle/mirbuilder_record_packed_layout_refresh.py tools/rust_lifecycle/mirbuilder_minimal_execution_path_selection.py
bash tools/checks/rust_lifecycle_mirbuilder_record_packed_layout_refresh_guard.sh
bash tools/checks/rust_lifecycle_mirbuilder_minimal_execution_path_selection_guard.sh
bash tools/checks/current_state_pointer_guard.sh
bash tools/checks/rust_lifecycle_no_silent_hardcode_guard.sh
cargo check --release
bash tools/checks/rust_mirbuilder_converter_matrix_guard.sh
```
