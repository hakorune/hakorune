---
Status: Landed
Date: 2026-06-26
Card: MIRBUILDER-MODULE-FUNCTION-INSERTION-DERIVED-HAKO-ARTIFACT-001
---

# MIRBUILDER-MODULE-FUNCTION-INSERTION-DERIVED-HAKO-ARTIFACT-001

## Summary

`MirBuilder::finalize_module` module function insertion now has a focused
DerivedShadow Hako artifact for the prepared minimal profile. The artifact
materializes the source-derived name-keyed insertion boundary:

```text
MirBuilder::finalize_module
  -> module.add_function(function)
  -> MirModule::add_function
  -> functions.insert(function.signature.name.clone(), function)
```

The executable surface is intentionally narrow: prepared module shell,
prepared main function shell, `OrderedMapBox.set`, and a result box that records
the successful insertion frame. It does not claim condition_fn injection,
all-functions PHI materialization, region cleanup, metadata publication,
semantic refresh, or full `finalize_module`.

## Authority

Semantic source:

```text
MirBuilderModuleFunctionInsertionPlanV1
  -> ModuleFunctionInsertion DerivedShadow artifact
```

Generated artifact:

```text
lang/generated/rust_derived/hakorune_mir_builder/mirbuilder_module_function_insertion.hako
lang/generated/rust_derived/hakorune_mir_builder/mirbuilder_module_function_insertion.artifact.json
```

The shared emitter change is a generic `MethodCall` renderer used to emit the
existing verified operation `OrderedMapBox.set`. It is not a
module-function-specific branch.

## Acceptance

```text
deterministic regeneration = green
generated Hako parse/MIR = green
generated Hako EXE/AOT = green
ModuleFunctionInsertionApi.insert/2 direct same-module route = green
proof = typed_global_call_same_module_object_handle
definition_owner = uniform_mir
result box = ModuleFunctionInsertionResultBox
module_transport = MirModuleMinimalShell
function_transport = MirFunctionPreparedMain
container = MirModule.functions
hako_operation = OrderedMapBox.set
collision_policy = ReplaceExistingByName
module_function_insertion = 1
condition_fn_injection = 0
full_finalize_module = 0
runtime_fallback = 0
```

The semantic closure report now consumes this artifact and advances the first
executable materialization gap:

```text
first_executable_materialization_gap = finalize_module.condition_fn_injection
next_slice = MIRBUILDER-CONDITION-FN-INJECTION-DERIVED-HAKO-ARTIFACT-001
```

## Artifacts

- `tools/rust_lifecycle/mirbuilder_module_function_insertion_artifacts.py`
- `tools/checks/rust_lifecycle_mirbuilder_module_function_insertion_derived_artifact_guard.sh`
- `docs/development/current/main/design/fixtures/rust-lifecycle/mirbuilder-module-function-insertion-derived-hako-oracle-v0.json`
- `docs/development/current/main/design/fixtures/rust-lifecycle/mirbuilder-module-function-insertion-derived-hako-recipe-v0.json`
- `docs/development/current/main/design/fixtures/rust-lifecycle/mirbuilder-module-function-insertion-derived-hako-verifier-result-v0.json`
- `lang/generated/rust_derived/hakorune_mir_builder/mirbuilder_module_function_insertion.hako`
- `lang/generated/rust_derived/hakorune_mir_builder/mirbuilder_module_function_insertion.artifact.json`

## Non-Claims

```text
condition_fn_injection = 0
all_functions_phi_materialization = 0
region_stack_pop = 0
slot_registry_release = 0
metadata_publication = 0
semantic_refresh = 0
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
python3 -m py_compile tools/rust_lifecycle/mirbuilder_module_function_insertion_artifacts.py
python3 tools/rust_lifecycle/convert_mirbuilder_lightweight_facts.py --family mirbuilder-module-function-insertion --check
bash tools/checks/rust_lifecycle_mirbuilder_module_function_insertion_derived_artifact_guard.sh
bash tools/checks/rust_lifecycle_mirbuilder_minimal_execution_path_semantic_closure_report_guard.sh
bash tools/checks/current_state_pointer_guard.sh
bash tools/checks/rust_lifecycle_no_silent_hardcode_guard.sh
cargo check --release
bash tools/checks/rust_mirbuilder_converter_matrix_guard.sh
```
