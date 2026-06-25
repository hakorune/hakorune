---
Status: Landed
Date: 2026-06-26
Card: MIRBUILDER-CONDITION-FN-INJECTION-DERIVED-HAKO-ARTIFACT-001
---

# MIRBUILDER-CONDITION-FN-INJECTION-DERIVED-HAKO-ARTIFACT-001

## Summary

`MirBuilder::finalize_module` condition function injection now has a focused
DerivedShadow Hako artifact for the prepared minimal profile. The artifact
materializes the source-required `condition_fn` stub insertion boundary:

```text
MirBuilder::finalize_module
  -> inject condition_fn when missing
  -> module.functions.has("condition_fn")
  -> module.functions.set("condition_fn", condition_fn_stub)
```

The executable surface is intentionally narrow: prepared module shell,
condition_fn function shell, `OrderedMapBox.has`, `OrderedMapBox.set`, and a
result box that records whether the stub was inserted. It also verifies that a
second injection attempt does not duplicate the function.

## Authority

Semantic source:

```text
MirBuilderConditionFnInjectionPlanV1
  -> ConditionFnInjection DerivedShadow artifact
```

Generated artifact:

```text
lang/generated/rust_derived/hakorune_mir_builder/mirbuilder_condition_fn_injection.hako
lang/generated/rust_derived/hakorune_mir_builder/mirbuilder_condition_fn_injection.artifact.json
```

The shared emitter change adds generic `StaticCall` rendering and literal
expression support. It does not branch on `condition_fn`, `MirBuilder`, or any
family name.

## Acceptance

```text
deterministic regeneration = green
generated Hako parse/MIR = green
generated Hako EXE/AOT = green
ConditionFnInjectionApi.inject_if_missing/1 direct same-module route = green
proof = typed_global_call_same_module_object_handle
definition_owner = uniform_mir
result box = ConditionFnInjectionResultBox
module_transport = MirModuleMinimalShell
predicate = module.functions.get("condition_fn").is_none()
hako_operation = OrderedMapBox.has + OrderedMapBox.set
function_name = condition_fn
param_count = 1
return_type = MirType::Integer
effects = EffectMask::PURE
entry_block = 0
body = ConstInteger(1), ReturnValue(one)
condition_fn_injection = 1
region_stack_pop = 0
slot_registry_release = 0
full_finalize_module = 0
runtime_fallback = 0
```

The semantic closure report now consumes this artifact and advances the first
executable materialization gap:

```text
first_executable_materialization_gap = finalize_module.region_stack_pop
next_slice = MIRBUILDER-FUNCTION-REGION-STACK-POP-DERIVED-HAKO-ARTIFACT-001
```

Task-order intentionally does not continue directly to that materialization
gap. The next blocker is the Python semantic projector growth freeze checkpoint
before widening derived artifact expansion further.

## Artifacts

- `tools/rust_lifecycle/mirbuilder_condition_fn_injection_artifacts.py`
- `tools/checks/rust_lifecycle_mirbuilder_condition_fn_injection_derived_artifact_guard.sh`
- `docs/development/current/main/design/fixtures/rust-lifecycle/mirbuilder-condition-fn-injection-derived-hako-oracle-v0.json`
- `docs/development/current/main/design/fixtures/rust-lifecycle/mirbuilder-condition-fn-injection-derived-hako-recipe-v0.json`
- `docs/development/current/main/design/fixtures/rust-lifecycle/mirbuilder-condition-fn-injection-derived-hako-verifier-result-v0.json`
- `lang/generated/rust_derived/hakorune_mir_builder/mirbuilder_condition_fn_injection.hako`
- `lang/generated/rust_derived/hakorune_mir_builder/mirbuilder_condition_fn_injection.artifact.json`

## Non-Claims

```text
condition_fn_policy_generalization = 0
region_stack_pop = 0
slot_registry_release = 0
metadata_publication = 0
semantic_refresh = 0
all_functions_phi_materialization = 0
full_finalize_module = 0
mainline_selected = 0
source_selfhost_claim = 0
runtime_fallback = 0
new_backend_route = 0
new_abi = 0
new_canonical_mir_instruction = 0
```

## Gates

```text
python3 -m py_compile tools/rust_lifecycle/mirbuilder_condition_fn_injection_artifacts.py
python3 tools/rust_lifecycle/convert_mirbuilder_lightweight_facts.py --family mirbuilder-condition-fn-injection --check
bash tools/checks/rust_lifecycle_mirbuilder_condition_fn_injection_derived_artifact_guard.sh
bash tools/checks/rust_lifecycle_mirbuilder_minimal_execution_path_semantic_closure_report_guard.sh
bash tools/checks/current_state_pointer_guard.sh
bash tools/checks/rust_lifecycle_no_silent_hardcode_guard.sh
cargo check --release
bash tools/checks/rust_mirbuilder_converter_matrix_guard.sh
```
