---
Status: Landed
Date: 2026-06-26
Card: MIRBUILDER-RETURN-EMISSION-DERIVED-HAKO-ARTIFACT-001
---

# MIRBUILDER-RETURN-EMISSION-DERIVED-HAKO-ARTIFACT-001

## Summary

`MirBuilder::finalize_module` ReturnEmission now has a focused DerivedShadow
Hako artifact for the prepared minimal profile. The artifact materializes only:

```text
append Return(result_value)
mark target block terminated
record Some(result_value)
record empty successors
```

It does not publish return types, run finalize composition, take module/function
state, or select mainline execution.

## Authority

Semantic source:

```text
MirBuilderReturnEmissionPlanV1
  -> ReturnEmission DerivedShadow artifact
```

Generated artifact:

```text
lang/generated/rust_derived/hakorune_mir_builder/mirbuilder_return_emission.hako
lang/generated/rust_derived/hakorune_mir_builder/mirbuilder_return_emission.artifact.json
```

The artifact is an executable materialization of the existing ReturnEmission
plan. It is not a full `finalize_module` artifact and does not reinterpret
return type publication.

## Acceptance

```text
deterministic regeneration = green
generated Hako parse/MIR = green
generated Hako EXE/AOT = green
ReturnEmissionApi.emit/2 direct same-module route = green
definition_owner = uniform_mir
result box = ReturnEmissionBasicBlockShellBox
return_emission = 1
return_type_publication = 0
full_finalize_module = 0
runtime_fallback = 0
```

The semantic closure report now consumes this artifact and advances the first
executable materialization gap:

```text
first_executable_materialization_gap = finalize_module.return_type_publication
next_slice = MIRBUILDER-RETURN-TYPE-PUBLICATION-DERIVED-HAKO-ARTIFACT-001
```

## Artifacts

- `tools/rust_lifecycle/mirbuilder_return_emission_artifacts.py`
- `tools/checks/rust_lifecycle_mirbuilder_return_emission_derived_artifact_guard.sh`
- `docs/development/current/main/design/fixtures/rust-lifecycle/mirbuilder-return-emission-derived-hako-oracle-v0.json`
- `docs/development/current/main/design/fixtures/rust-lifecycle/mirbuilder-return-emission-derived-hako-recipe-v0.json`
- `docs/development/current/main/design/fixtures/rust-lifecycle/mirbuilder-return-emission-derived-hako-verifier-result-v0.json`
- `lang/generated/rust_derived/hakorune_mir_builder/mirbuilder_return_emission.hako`
- `lang/generated/rust_derived/hakorune_mir_builder/mirbuilder_return_emission.artifact.json`

## Non-Claims

```text
return_type_publication = 0
full_finalize_module = 0
other_terminator_shapes = 0
already_terminated_block_behavior = 0
mainline_selected = 0
source_selfhost_claim = 0
runtime_fallback = 0
new_backend_route = 0
new_abi = 0
new_canonical_mir_instruction = 0
```

## Gates

```text
python3 -m py_compile tools/rust_lifecycle/mirbuilder_return_emission_artifacts.py
python3 tools/rust_lifecycle/convert_mirbuilder_lightweight_facts.py --family mirbuilder-return-emission --check
bash tools/checks/rust_lifecycle_mirbuilder_return_emission_derived_artifact_guard.sh
bash tools/checks/rust_lifecycle_mirbuilder_minimal_execution_path_semantic_closure_report_guard.sh
bash tools/checks/current_state_pointer_guard.sh
bash tools/checks/rust_lifecycle_no_silent_hardcode_guard.sh
cargo check --release
bash tools/checks/rust_mirbuilder_converter_matrix_guard.sh
```
