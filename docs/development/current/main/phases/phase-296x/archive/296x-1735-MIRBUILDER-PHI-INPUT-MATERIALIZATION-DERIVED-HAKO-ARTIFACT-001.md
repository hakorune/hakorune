---
Status: Landed
Date: 2026-06-26
Card: MIRBUILDER-PHI-INPUT-MATERIALIZATION-DERIVED-HAKO-ARTIFACT-001
---

# MIRBUILDER-PHI-INPUT-MATERIALIZATION-DERIVED-HAKO-ARTIFACT-001

## Summary

`MirBuilder::finalize_module` PHI input materialization now has a focused
DerivedShadow Hako artifact for the prepared minimal profile. The artifact
materializes the source-derived PHI input materialization step order:

```text
phi_input_materializer::materialize_all_phi_inputs(&mut function)
```

It records the selected step sequence, mutates the prepared function shell,
returns the changed-count result shell, and does not claim dev birth
verification, module insertion, all-functions PHI materialization, or full
`finalize_module`.

## Authority

Semantic source:

```text
MirBuilderPhiInputMaterializationPlanV1
  -> PhiInputMaterialization DerivedShadow artifact
```

Generated artifact:

```text
lang/generated/rust_derived/hakorune_mir_builder/mirbuilder_phi_input_materialization.hako
lang/generated/rust_derived/hakorune_mir_builder/mirbuilder_phi_input_materialization.artifact.json
```

The artifact is an executable materialization of the existing PHI input
materialization plan. It is not dev birth verification, module insertion, an
all-functions sweep, semantic refresh, or full finalize_module.

## Acceptance

```text
deterministic regeneration = green
generated Hako parse/MIR = green
generated Hako EXE/AOT = green
PhiInputMaterializationApi.run/1 direct same-module route = green
proof = typed_global_call_same_module_object_handle
definition_owner = uniform_mir
result box = PhiInputMaterializationResultBox
materialization_steps = 7
phi_input_materialization = 1
dev_birth_verification = 0
module_function_insertion = 0
condition_fn_injection = 0
all_functions_phi_materialization = 0
full_finalize_module = 0
runtime_fallback = 0
```

The semantic closure report now consumes this artifact and advances the first
executable materialization gap:

```text
first_executable_materialization_gap = finalize_module.dev_birth_verification
next_slice = MIRBUILDER-DEV-BIRTH-VERIFICATION-DERIVED-HAKO-ARTIFACT-001
```

## Artifacts

- `tools/rust_lifecycle/mirbuilder_phi_input_materialization_artifacts.py`
- `tools/checks/rust_lifecycle_mirbuilder_phi_input_materialization_derived_artifact_guard.sh`
- `docs/development/current/main/design/fixtures/rust-lifecycle/mirbuilder-phi-input-materialization-derived-hako-oracle-v0.json`
- `docs/development/current/main/design/fixtures/rust-lifecycle/mirbuilder-phi-input-materialization-derived-hako-recipe-v0.json`
- `docs/development/current/main/design/fixtures/rust-lifecycle/mirbuilder-phi-input-materialization-derived-hako-verifier-result-v0.json`
- `lang/generated/rust_derived/hakorune_mir_builder/mirbuilder_phi_input_materialization.hako`
- `lang/generated/rust_derived/hakorune_mir_builder/mirbuilder_phi_input_materialization.artifact.json`

## Non-Claims

```text
dev_birth_verification = 0
module_function_insertion = 0
condition_fn_injection = 0
all_functions_phi_materialization = 0
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
python3 -m py_compile tools/rust_lifecycle/mirbuilder_phi_input_materialization_artifacts.py
python3 tools/rust_lifecycle/convert_mirbuilder_lightweight_facts.py --family mirbuilder-phi-input-materialization --check
bash tools/checks/rust_lifecycle_mirbuilder_phi_input_materialization_derived_artifact_guard.sh
bash tools/checks/rust_lifecycle_mirbuilder_minimal_execution_path_semantic_closure_report_guard.sh
bash tools/checks/current_state_pointer_guard.sh
bash tools/checks/rust_lifecycle_no_silent_hardcode_guard.sh
cargo check --release
bash tools/checks/rust_mirbuilder_converter_matrix_guard.sh
```
