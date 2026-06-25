---
Status: Landed
Date: 2026-06-25
Card: MIRBUILDER-CONDITION-FN-INJECTION-001
---

# MIRBUILDER-CONDITION-FN-INJECTION-001

## Summary

`ConditionFnInjection` is now a source-derived PlanOnly capability for the
prepared-state minimal MirBuilder path. The slice fixes only the source-required
`condition_fn` stub injection when missing. It does not claim region cleanup,
metadata publication, semantic refresh, all-functions PHI materialization, full
finalize, generated Hako, backend routes, ABI changes, runtime fallback, or
source selfhost.

## Source Authority

- `src/mir/builder/module_lifecycle.rs::MirBuilder::finalize_module`
- Predecessor plan:
  `docs/development/current/main/design/fixtures/rust-lifecycle/mirbuilder-module-function-insertion-plan-v0.json`

## Injection Contract

```text
predicate = module.functions.get("condition_fn").is_none()
function_name = condition_fn
params = [MirType::Integer]
return_type = MirType::Integer
effects = EffectMask::PURE
entry_block = BasicBlockId(0)
body = ConstInteger(1), ReturnValue(one)
insert_operation = module.add_function(f)
required_by_source = true
```

## Artifacts

- `tools/rust_lifecycle/mirbuilder_condition_fn_injection.py`
- `docs/development/current/main/design/fixtures/rust-lifecycle/mirbuilder-condition-fn-injection-plan-v0.json`
- `tools/checks/rust_lifecycle_mirbuilder_condition_fn_injection_guard.sh`

## Derived Frontier Result

The minimal execution path analyzer now marks
`finalize_module.condition_fn_injection` as `Available`.

The next derived unsupported edge is:

```text
edge_id: finalize_module.region_stack_pop
callsite: MirBuilder::finalize_module -> region::observer::pop_function_region
deny_reason: UnsupportedDirectShape
deny_detail: FunctionRegionStackPopRequired
semantic_owner: MirBuilder::finalize_module function region cleanup
next_slice_token: MIRBUILDER-FUNCTION-REGION-STACK-POP-001
```

## Non-Claims

```text
condition_fn_policy_generalization = 0
region_stack_pop = 0
slot_registry_release = 0
metadata_publication = 0
semantic_refresh = 0
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
python3 -m py_compile tools/rust_lifecycle/mirbuilder_condition_fn_injection.py tools/rust_lifecycle/mirbuilder_minimal_execution_path_selection.py
bash tools/checks/rust_lifecycle_mirbuilder_condition_fn_injection_guard.sh
bash tools/checks/rust_lifecycle_mirbuilder_minimal_execution_path_selection_guard.sh
bash tools/checks/current_state_pointer_guard.sh
bash tools/checks/rust_lifecycle_no_silent_hardcode_guard.sh
cargo check --release
bash tools/checks/rust_mirbuilder_converter_matrix_guard.sh
```
