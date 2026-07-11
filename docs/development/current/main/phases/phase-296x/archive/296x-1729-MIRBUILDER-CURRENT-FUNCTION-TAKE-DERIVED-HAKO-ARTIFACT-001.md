---
Status: Landed
Date: 2026-06-26
Card: MIRBUILDER-CURRENT-FUNCTION-TAKE-DERIVED-HAKO-ARTIFACT-001
---

# MIRBUILDER-CURRENT-FUNCTION-TAKE-DERIVED-HAKO-ARTIFACT-001

## Summary

`MirBuilder::finalize_module` current-function take now has a focused
DerivedShadow Hako artifact for the prepared minimal profile. The artifact
materializes only:

```text
prepared current_function presence shell
clear current_function presence
mark function payload taken
return MirFunctionPreparedMain payload
```

It does not run type propagation, provide type hints, infer PHI return types,
materialize PHI inputs, insert module functions, publish metadata, or claim full
finalize.

## Authority

Semantic source:

```text
MirBuilderCurrentFunctionTakePlanV1
  -> CurrentFunctionTake DerivedShadow artifact
```

Generated artifact:

```text
lang/generated/rust_derived/hakorune_mir_builder/mirbuilder_current_function_take.hako
lang/generated/rust_derived/hakorune_mir_builder/mirbuilder_current_function_take.artifact.json
```

The artifact is an executable materialization of the existing current-function
take plan. It is not a full `finalize_module` artifact and does not reinterpret
type propagation or later finalize stages.

## Acceptance

```text
deterministic regeneration = green
generated Hako parse/MIR = green
generated Hako EXE/AOT = green
CurrentFunctionTakeApi.take/1 direct same-module route = green
proof = typed_global_call_same_module_object_handle
definition_owner = uniform_mir
result box = MirFunctionPreparedMainBox
current_function_take = 1
type_propagation = 0
type_hint_provision = 0
full_finalize_module = 0
runtime_fallback = 0
```

The semantic closure report now consumes this artifact and advances the first
executable materialization gap:

```text
first_executable_materialization_gap = finalize_module.type_propagation
next_slice = MIRBUILDER-TYPE-PROPAGATION-PIPELINE-DERIVED-HAKO-ARTIFACT-001
```

## Artifacts

- `tools/rust_lifecycle/mirbuilder_current_function_take_artifacts.py`
- `tools/checks/rust_lifecycle_mirbuilder_current_function_take_derived_artifact_guard.sh`
- `docs/development/current/main/design/fixtures/rust-lifecycle/mirbuilder-current-function-take-derived-hako-oracle-v0.json`
- `docs/development/current/main/design/fixtures/rust-lifecycle/mirbuilder-current-function-take-derived-hako-recipe-v0.json`
- `docs/development/current/main/design/fixtures/rust-lifecycle/mirbuilder-current-function-take-derived-hako-verifier-result-v0.json`
- `lang/generated/rust_derived/hakorune_mir_builder/mirbuilder_current_function_take.hako`
- `lang/generated/rust_derived/hakorune_mir_builder/mirbuilder_current_function_take.artifact.json`

## Non-Claims

```text
type_propagation = 0
type_hint_provision = 0
metadata_value_type_publication = 0
phi_return_type_inference = 0
phi_input_materialization = 0
module_function_insertion = 0
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
python3 -m py_compile tools/rust_lifecycle/mirbuilder_current_function_take_artifacts.py
python3 tools/rust_lifecycle/convert_mirbuilder_lightweight_facts.py --family mirbuilder-current-function-take --check
bash tools/checks/rust_lifecycle_mirbuilder_current_function_take_derived_artifact_guard.sh
bash tools/checks/rust_lifecycle_mirbuilder_minimal_execution_path_semantic_closure_report_guard.sh
bash tools/checks/current_state_pointer_guard.sh
bash tools/checks/rust_lifecycle_no_silent_hardcode_guard.sh
cargo check --release
bash tools/checks/rust_mirbuilder_converter_matrix_guard.sh
```
