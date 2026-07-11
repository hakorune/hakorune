---
Status: Landed
Date: 2026-06-25
Card: MIRBUILDER-MODULE-FUNCTION-INSERTION-001
---

# MIRBUILDER-MODULE-FUNCTION-INSERTION-001

## Summary

`ModuleFunctionInsertion` is now a source-derived PlanOnly capability for the
prepared-state minimal MirBuilder path. The slice fixes only the first
`module.add_function(function)` call and the `MirModule::add_function`
name-keyed insertion behavior. It does not claim condition_fn injection,
all-functions PHI materialization, region cleanup, metadata publication,
semantic refresh, full finalize, generated Hako, backend routes, ABI changes,
runtime fallback, or source selfhost.

## Source Authority

- `src/mir/builder/module_lifecycle.rs::MirBuilder::finalize_module`
- `src/mir/function/module_impl.rs::MirModule::add_function`
- Predecessor plan:
  `docs/development/current/main/design/fixtures/rust-lifecycle/mirbuilder-dev-birth-verification-plan-v0.json`

## Insertion Contract

```text
callsite = module.add_function(function)
inserted_function = MirFunctionPreparedMain
key_source = function.signature.name.clone()
container = MirModule.functions
container_operation = BTreeMap::insert
collision_policy = ReplaceExistingByName
```

## Artifacts

- `tools/rust_lifecycle/mirbuilder_module_function_insertion.py`
- `docs/development/current/main/design/fixtures/rust-lifecycle/mirbuilder-module-function-insertion-plan-v0.json`
- `tools/checks/rust_lifecycle_mirbuilder_module_function_insertion_guard.sh`

## Derived Frontier Result

The minimal execution path analyzer now marks
`finalize_module.module_function_insertion` as `Available`.

The next derived unsupported edge is:

```text
edge_id: finalize_module.condition_fn_injection
callsite: MirBuilder::finalize_module -> inject condition_fn when missing
deny_reason: UnsupportedDirectShape
deny_detail: ConditionFnInjectionRequired
semantic_owner: MirBuilder::finalize_module condition_fn injection
next_slice_token: MIRBUILDER-CONDITION-FN-INJECTION-001
```

## Non-Claims

```text
condition_fn_injection = 0
all_functions_phi_materialization = 0
region_stack_pop = 0
slot_registry_release = 0
metadata_publication = 0
semantic_refresh = 0
full_finalize_module = 0
generated_hako_artifact = 0
backend_route_changed = 0
abi_changed = 0
runtime_fallback = 0
mainline_selected = 0
```

## Acceptance

```text
python3 -m py_compile tools/rust_lifecycle/mirbuilder_module_function_insertion.py tools/rust_lifecycle/mirbuilder_minimal_execution_path_selection.py
bash tools/checks/rust_lifecycle_mirbuilder_module_function_insertion_guard.sh
bash tools/checks/rust_lifecycle_mirbuilder_minimal_execution_path_selection_guard.sh
bash tools/checks/current_state_pointer_guard.sh
bash tools/checks/rust_lifecycle_no_silent_hardcode_guard.sh
cargo check --release
bash tools/checks/rust_mirbuilder_converter_matrix_guard.sh
```
