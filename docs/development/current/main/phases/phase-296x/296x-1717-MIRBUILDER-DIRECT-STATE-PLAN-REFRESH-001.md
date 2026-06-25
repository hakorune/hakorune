---
Status: Landed
Date: 2026-06-25
Card: MIRBUILDER-DIRECT-STATE-PLAN-REFRESH-001
---

# MIRBUILDER-DIRECT-STATE-PLAN-REFRESH-001

## Summary

`DirectStatePlanRefresh` is now a source-derived PlanOnly capability for the
prepared-state minimal MirBuilder path. The slice owns only
`refresh_module_direct_state_plans(&mut module)` and its assignment of
`module.metadata.direct_state_plans`. It does not claim all-functions PHI
materialization, direct-state lowering, route selection, NativeDirect guards,
full finalize, generated Hako, backend routes, ABI changes, runtime fallback,
or source selfhost.

## Source Authority

- `src/mir/builder/module_lifecycle.rs::MirBuilder::finalize_module`
- `src/mir/direct_state_plan.rs::refresh_module_direct_state_plans`
- Predecessor plan:
  `docs/development/current/main/design/fixtures/rust-lifecycle/mirbuilder-typed-object-plan-refresh-plan-v0.json`

## Refresh Contract

```text
entrypoint = refresh_module_direct_state_plans
timing = AfterTypedObjectPlanRefreshBeforeAllFunctionsPhiMaterialization
operation = AssignDirectStatePlans
source = build_direct_state_plans(module)
build_provider = direct_state_plan::build_direct_state_plans
target = module.metadata.direct_state_plans
```

## Builder Contract

```text
input_authority = module.metadata.user_box_field_decls
ordering = SortBoxNames
field_selection = TypedObjectFieldStorageUsesIntegerLaneAndNotWeak
state_repr = direct_v0
runtime_layout_created = 0
lowering_enabled = 0
```

## Artifacts

- `tools/rust_lifecycle/mirbuilder_direct_state_plan_refresh.py`
- `docs/development/current/main/design/fixtures/rust-lifecycle/mirbuilder-direct-state-plan-refresh-plan-v0.json`
- `tools/checks/rust_lifecycle_mirbuilder_direct_state_plan_refresh_guard.sh`

## Derived Frontier Result

The minimal execution path analyzer now marks
`finalize_module.direct_state_plan_refresh` as `Available`.

The next derived unsupported edge is:

```text
edge_id: finalize_module.all_functions_phi_materialization
callsite: MirBuilder::finalize_module -> materialize_all_phi_inputs for all functions
deny_reason: UnsupportedDirectShape
deny_detail: AllFunctionsPhiMaterializationRequired
semantic_owner: MirBuilder::finalize_module all-functions PHI materialization
next_slice_token: MIRBUILDER-ALL-FUNCTIONS-PHI-MATERIALIZATION-001
```

## Non-Claims

```text
all_functions_phi_materialization = 0
direct_state_lowering = 0
route_selection = 0
native_direct_guard = 0
full_semantic_refresh = 0
full_finalize_module = 0
generated_hako_artifact = 0
backend_route_changed = 0
abi_changed = 0
runtime_fallback = 0
mainline_selected = 0
```

## Acceptance

```text
python3 -m py_compile tools/rust_lifecycle/mirbuilder_direct_state_plan_refresh.py tools/rust_lifecycle/mirbuilder_minimal_execution_path_selection.py
bash tools/checks/rust_lifecycle_mirbuilder_direct_state_plan_refresh_guard.sh
bash tools/checks/rust_lifecycle_mirbuilder_minimal_execution_path_selection_guard.sh
bash tools/checks/current_state_pointer_guard.sh
bash tools/checks/rust_lifecycle_no_silent_hardcode_guard.sh
cargo check --release
bash tools/checks/rust_mirbuilder_converter_matrix_guard.sh
```
