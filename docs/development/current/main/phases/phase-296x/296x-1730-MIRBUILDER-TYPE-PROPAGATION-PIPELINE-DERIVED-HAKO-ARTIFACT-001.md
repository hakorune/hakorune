---
Status: Landed
Date: 2026-06-26
Card: MIRBUILDER-TYPE-PROPAGATION-PIPELINE-DERIVED-HAKO-ARTIFACT-001
---

# MIRBUILDER-TYPE-PROPAGATION-PIPELINE-DERIVED-HAKO-ARTIFACT-001

## Summary

`MirBuilder::finalize_module` TypePropagationPipeline execution now has a
focused DerivedShadow Hako artifact for the prepared minimal profile. The
artifact materializes only the existing source-derived pipeline order:

```text
seed_declared_field_types
copy_propagation_initial
binop_repropagation
copy_propagation_after_binop
phi_type_inference
```

It records the prepared function and value-types mutation frame and returns a
`TypePropagationPipelineResultBox`. It does not provide type hints, publish
metadata value types, infer PHI return types for finalize, materialize PHI
inputs, insert module functions, or claim full finalize.

## Authority

Semantic source:

```text
MirBuilderTypePropagationPipelinePlanV1
  -> TypePropagationPipelineExecution DerivedShadow artifact
```

Generated artifact:

```text
lang/generated/rust_derived/hakorune_mir_builder/mirbuilder_type_propagation_pipeline.hako
lang/generated/rust_derived/hakorune_mir_builder/mirbuilder_type_propagation_pipeline.artifact.json
```

The artifact is an executable materialization of the existing TypePropagation
pipeline plan. It is not a full type system artifact and does not reinterpret
the later type-hint or metadata publication stages.

## Acceptance

```text
deterministic regeneration = green
generated Hako parse/MIR = green
generated Hako EXE/AOT = green
TypePropagationPipelineApi.run/2 direct same-module route = green
proof = typed_global_call_same_module_object_handle
definition_owner = uniform_mir
result box = TypePropagationPipelineResultBox
type_propagation = 1
type_hint_provision = 0
metadata_value_type_publication = 0
full_finalize_module = 0
runtime_fallback = 0
```

The semantic closure report now consumes this artifact and advances the first
executable materialization gap:

```text
first_executable_materialization_gap = finalize_module.type_hint_provision
next_slice = MIRBUILDER-TYPE-HINT-PROVISION-DERIVED-HAKO-ARTIFACT-001
```

## Artifacts

- `tools/rust_lifecycle/mirbuilder_type_propagation_pipeline_artifacts.py`
- `tools/checks/rust_lifecycle_mirbuilder_type_propagation_pipeline_derived_artifact_guard.sh`
- `docs/development/current/main/design/fixtures/rust-lifecycle/mirbuilder-type-propagation-pipeline-derived-hako-oracle-v0.json`
- `docs/development/current/main/design/fixtures/rust-lifecycle/mirbuilder-type-propagation-pipeline-derived-hako-recipe-v0.json`
- `docs/development/current/main/design/fixtures/rust-lifecycle/mirbuilder-type-propagation-pipeline-derived-hako-verifier-result-v0.json`
- `lang/generated/rust_derived/hakorune_mir_builder/mirbuilder_type_propagation_pipeline.hako`
- `lang/generated/rust_derived/hakorune_mir_builder/mirbuilder_type_propagation_pipeline.artifact.json`

## Non-Claims

```text
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
python3 -m py_compile tools/rust_lifecycle/mirbuilder_type_propagation_pipeline_artifacts.py
python3 tools/rust_lifecycle/convert_mirbuilder_lightweight_facts.py --family mirbuilder-type-propagation-pipeline --check
bash tools/checks/rust_lifecycle_mirbuilder_type_propagation_pipeline_derived_artifact_guard.sh
bash tools/checks/rust_lifecycle_mirbuilder_minimal_execution_path_semantic_closure_report_guard.sh
bash tools/checks/current_state_pointer_guard.sh
bash tools/checks/rust_lifecycle_no_silent_hardcode_guard.sh
cargo check --release
bash tools/checks/rust_mirbuilder_converter_matrix_guard.sh
```
