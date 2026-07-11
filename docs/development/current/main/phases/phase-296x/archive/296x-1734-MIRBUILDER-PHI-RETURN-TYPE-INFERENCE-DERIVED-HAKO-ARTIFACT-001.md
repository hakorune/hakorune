---
Status: Landed
Date: 2026-06-26
Card: MIRBUILDER-PHI-RETURN-TYPE-INFERENCE-DERIVED-HAKO-ARTIFACT-001
---

# MIRBUILDER-PHI-RETURN-TYPE-INFERENCE-DERIVED-HAKO-ARTIFACT-001

## Summary

`MirBuilder::finalize_module` PHI return-type inference now has a focused
DerivedShadow Hako artifact for the prepared minimal profile. The artifact
materializes the source-derived resolver-chain frame for this delegated call:

```text
phi_type_inference::infer_return_type_from_phi(self, &mut function)
```

It records the resolver chain, updates the prepared function signature
return-type shell, and returns a `PhiReturnTypeInferenceResultBox`. It does
not materialize PHI inputs, insert module functions, or claim full
`finalize_module`.

## Authority

Semantic source:

```text
MirBuilderPhiReturnTypeInferencePlanV1
  -> PhiReturnTypeInference DerivedShadow artifact
```

Generated artifact:

```text
lang/generated/rust_derived/hakorune_mir_builder/mirbuilder_phi_return_type_inference.hako
lang/generated/rust_derived/hakorune_mir_builder/mirbuilder_phi_return_type_inference.artifact.json
```

The artifact is an executable materialization of the existing PHI return-type
inference plan. It is not PHI input materialization or full finalize_module.

## Acceptance

```text
deterministic regeneration = green
generated Hako parse/MIR = green
generated Hako EXE/AOT = green
PhiReturnTypeInferenceApi.infer/2 direct same-module route = green
proof = typed_global_call_same_module_object_handle
definition_owner = uniform_mir
result box = PhiReturnTypeInferenceResultBox
phi_return_type_inference = 1
phi_input_materialization = 0
full_finalize_module = 0
runtime_fallback = 0
```

The semantic closure report now consumes this artifact and advances the first
executable materialization gap:

```text
first_executable_materialization_gap = finalize_module.phi_input_materialization
next_slice = MIRBUILDER-PHI-INPUT-MATERIALIZATION-DERIVED-HAKO-ARTIFACT-001
```

## Artifacts

- `tools/rust_lifecycle/mirbuilder_phi_return_type_inference_artifacts.py`
- `tools/checks/rust_lifecycle_mirbuilder_phi_return_type_inference_derived_artifact_guard.sh`
- `docs/development/current/main/design/fixtures/rust-lifecycle/mirbuilder-phi-return-type-inference-derived-hako-oracle-v0.json`
- `docs/development/current/main/design/fixtures/rust-lifecycle/mirbuilder-phi-return-type-inference-derived-hako-recipe-v0.json`
- `docs/development/current/main/design/fixtures/rust-lifecycle/mirbuilder-phi-return-type-inference-derived-hako-verifier-result-v0.json`
- `lang/generated/rust_derived/hakorune_mir_builder/mirbuilder_phi_return_type_inference.hako`
- `lang/generated/rust_derived/hakorune_mir_builder/mirbuilder_phi_return_type_inference.artifact.json`

## Non-Claims

```text
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
python3 -m py_compile tools/rust_lifecycle/mirbuilder_phi_return_type_inference_artifacts.py
python3 tools/rust_lifecycle/convert_mirbuilder_lightweight_facts.py --family mirbuilder-phi-return-type-inference --check
bash tools/checks/rust_lifecycle_mirbuilder_phi_return_type_inference_derived_artifact_guard.sh
bash tools/checks/rust_lifecycle_mirbuilder_minimal_execution_path_semantic_closure_report_guard.sh
bash tools/checks/current_state_pointer_guard.sh
bash tools/checks/rust_lifecycle_no_silent_hardcode_guard.sh
cargo check --release
bash tools/checks/rust_mirbuilder_converter_matrix_guard.sh
```
