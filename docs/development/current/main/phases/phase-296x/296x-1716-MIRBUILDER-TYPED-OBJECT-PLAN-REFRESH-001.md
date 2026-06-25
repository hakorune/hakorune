---
Status: Landed
Date: 2026-06-25
Card: MIRBUILDER-TYPED-OBJECT-PLAN-REFRESH-001
---

# MIRBUILDER-TYPED-OBJECT-PLAN-REFRESH-001

## Summary

`TypedObjectPlanRefresh` is now a source-derived PlanOnly capability for the
prepared-state minimal MirBuilder path. The slice owns only
`refresh_module_typed_object_plans(&mut module)` and its assignment of
`module.metadata.typed_object_plans`. It does not claim direct-state refresh,
typed-object field value refresh, collection field element refresh,
all-functions PHI materialization, full finalize, generated Hako, backend
routes, ABI changes, runtime fallback, or source selfhost.

## Source Authority

- `src/mir/builder/module_lifecycle.rs::MirBuilder::finalize_module`
- `src/mir/typed_object_plan.rs::refresh_module_typed_object_plans`
- Predecessor plan:
  `docs/development/current/main/design/fixtures/rust-lifecycle/mirbuilder-record-packed-layout-refresh-plan-v0.json`

## Refresh Contract

```text
entrypoint = refresh_module_typed_object_plans
timing = AfterRecordPackedLayoutRefreshBeforeDirectStateRefresh
operation = AssignTypedObjectPlans
source = build_typed_object_plans(module)
build_provider = storage_inference::build_typed_object_plans
target = module.metadata.typed_object_plans
```

## Artifacts

- `tools/rust_lifecycle/mirbuilder_typed_object_plan_refresh.py`
- `docs/development/current/main/design/fixtures/rust-lifecycle/mirbuilder-typed-object-plan-refresh-plan-v0.json`
- `tools/checks/rust_lifecycle_mirbuilder_typed_object_plan_refresh_guard.sh`

## Derived Frontier Result

The minimal execution path analyzer now marks
`finalize_module.typed_object_plan_refresh` as `Available`.

The next derived unsupported edge is:

```text
edge_id: finalize_module.direct_state_plan_refresh
callsite: MirBuilder::finalize_module -> refresh_module_direct_state_plans
deny_reason: UnsupportedDirectShape
deny_detail: DirectStatePlanRefreshRequired
semantic_owner: MirBuilder::finalize_module direct state plan refresh
next_slice_token: MIRBUILDER-DIRECT-STATE-PLAN-REFRESH-001
```

## Non-Claims

```text
typed_object_field_value_type_refresh = 0
typed_object_collection_field_element_refresh = 0
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
python3 -m py_compile tools/rust_lifecycle/mirbuilder_typed_object_plan_refresh.py tools/rust_lifecycle/mirbuilder_minimal_execution_path_selection.py
bash tools/checks/rust_lifecycle_mirbuilder_typed_object_plan_refresh_guard.sh
bash tools/checks/rust_lifecycle_mirbuilder_minimal_execution_path_selection_guard.sh
bash tools/checks/current_state_pointer_guard.sh
bash tools/checks/rust_lifecycle_no_silent_hardcode_guard.sh
cargo check --release
bash tools/checks/rust_mirbuilder_converter_matrix_guard.sh
```
