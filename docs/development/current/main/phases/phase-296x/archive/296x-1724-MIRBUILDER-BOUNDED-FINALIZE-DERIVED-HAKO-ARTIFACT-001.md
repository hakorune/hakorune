---
Status: Landed
Date: 2026-06-26
Card: MIRBUILDER-BOUNDED-FINALIZE-DERIVED-HAKO-ARTIFACT-001
---

# MIRBUILDER-BOUNDED-FINALIZE-DERIVED-HAKO-ARTIFACT-001

## Summary

`MirBuilder::finalize_module` now has a bounded DerivedShadow Hako artifact for
the prepared minimal profile. The artifact materializes the composition shell
for:

```text
append Return(result_value)
publish integer return type
take prepared module/function/block presence tags
add main function
inject condition_fn
publish metadata shell
run semantic refresh subset shell
return finalized module shell
```

It uses direct prepared payload arguments inside the same-module helper to avoid
backend-specific interpretation of nested object fields. It does not claim full
`finalize_module`, full `build_module`, reusable child capabilities, mainline
selection, or source selfhost.

## Authority

Semantic source:

```text
MirBuilderBoundedFinalizeCompositionPlanV1
  -> BoundedFinalizeComposition DerivedShadow artifact
```

Generated artifact:

```text
lang/generated/rust_derived/hakorune_mir_builder/mirbuilder_bounded_finalize_composition.hako
lang/generated/rust_derived/hakorune_mir_builder/mirbuilder_bounded_finalize_composition.artifact.json
```

The artifact is an executable materialization of the existing bounded finalize
composition plan. It is not a reusable ReturnEmission or TypePublication
artifact.

## Acceptance

```text
deterministic regeneration = green
generated Hako parse/MIR = green
generated Hako EXE/AOT = green
return instruction shell connected = green
return type integer published = green
condition_fn injected = green
metadata publication shell = green
semantic refresh subset shell = green
state presence tags cleared = green
```

The semantic closure report now consumes this artifact and advances the first
executable materialization gap:

```text
first_executable_materialization_gap = finalize_module.return_emission
next_slice = MIRBUILDER-RETURN-EMISSION-DERIVED-HAKO-ARTIFACT-001
```

## Artifacts

- `tools/rust_lifecycle/mirbuilder_bounded_finalize_artifacts.py`
- `tools/checks/rust_lifecycle_mirbuilder_bounded_finalize_derived_artifact_guard.sh`
- `docs/development/current/main/design/fixtures/rust-lifecycle/mirbuilder-bounded-finalize-derived-hako-oracle-v0.json`
- `docs/development/current/main/design/fixtures/rust-lifecycle/mirbuilder-bounded-finalize-derived-hako-recipe-v0.json`
- `docs/development/current/main/design/fixtures/rust-lifecycle/mirbuilder-bounded-finalize-derived-hako-verifier-result-v0.json`
- `lang/generated/rust_derived/hakorune_mir_builder/mirbuilder_bounded_finalize_composition.hako`
- `lang/generated/rust_derived/hakorune_mir_builder/mirbuilder_bounded_finalize_composition.artifact.json`

## Non-Claims

```text
full_finalize_module = 0
full_build_module_execution = 0
reusable_return_emission = 0
reusable_type_publication = 0
current_module_take_artifact = 0
current_function_take_artifact = 0
mainline_selected = 0
source_selfhost_claim = 0
runtime_fallback = 0
new_backend_route = 0
new_abi = 0
```

## Gates

```text
python3 -m py_compile tools/rust_lifecycle/mirbuilder_bounded_finalize_artifacts.py
python3 tools/rust_lifecycle/convert_mirbuilder_lightweight_facts.py --family mirbuilder-bounded-finalize-composition --check
bash tools/checks/rust_lifecycle_mirbuilder_bounded_finalize_derived_artifact_guard.sh
bash tools/checks/rust_lifecycle_mirbuilder_minimal_execution_path_semantic_closure_report_guard.sh
bash tools/checks/current_state_pointer_guard.sh
bash tools/checks/rust_lifecycle_no_silent_hardcode_guard.sh
cargo check --release
bash tools/checks/rust_mirbuilder_converter_matrix_guard.sh
```
