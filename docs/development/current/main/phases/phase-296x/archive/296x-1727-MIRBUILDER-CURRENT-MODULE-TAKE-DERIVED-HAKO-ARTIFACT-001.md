---
Status: Landed
Date: 2026-06-26
Card: MIRBUILDER-CURRENT-MODULE-TAKE-DERIVED-HAKO-ARTIFACT-001
---

# MIRBUILDER-CURRENT-MODULE-TAKE-DERIVED-HAKO-ARTIFACT-001

## Summary

`MirBuilder::finalize_module` current-module take now has a focused
DerivedShadow Hako artifact for the prepared minimal profile. The artifact
materializes only:

```text
prepared current_module presence shell
clear current_module presence
mark module payload taken
return MirModuleMinimalShell payload
```

It does not verify typed values, take the current function, run full finalize,
publish module metadata, or select mainline execution.

## Authority

Semantic source:

```text
MirBuilderCurrentModuleTakePlanV1
  -> CurrentModuleTake DerivedShadow artifact
```

Generated artifact:

```text
lang/generated/rust_derived/hakorune_mir_builder/mirbuilder_current_module_take.hako
lang/generated/rust_derived/hakorune_mir_builder/mirbuilder_current_module_take.artifact.json
```

The artifact is an executable materialization of the existing CurrentModuleTake
plan. It is not a full `finalize_module` artifact and does not reinterpret
typed-value verification.

## Acceptance

```text
deterministic regeneration = green
generated Hako parse/MIR = green
generated Hako EXE/AOT = green
CurrentModuleTakeApi.take/2 direct same-module route = green
definition_owner = uniform_mir
result box = CurrentModuleTakeModuleShellBox
current_module_take = 1
verify_typed_values = 0
current_function_take = 0
full_finalize_module = 0
runtime_fallback = 0
```

The semantic closure report now consumes this artifact and advances the first
executable materialization gap:

```text
first_executable_materialization_gap = finalize_module.verify_typed_values
next_slice = MIRBUILDER-TYPED-VALUE-VERIFICATION-DERIVED-HAKO-ARTIFACT-001
```

## Artifacts

- `tools/rust_lifecycle/mirbuilder_current_module_take_artifacts.py`
- `tools/checks/rust_lifecycle_mirbuilder_current_module_take_derived_artifact_guard.sh`
- `docs/development/current/main/design/fixtures/rust-lifecycle/mirbuilder-current-module-take-derived-hako-oracle-v0.json`
- `docs/development/current/main/design/fixtures/rust-lifecycle/mirbuilder-current-module-take-derived-hako-recipe-v0.json`
- `docs/development/current/main/design/fixtures/rust-lifecycle/mirbuilder-current-module-take-derived-hako-verifier-result-v0.json`
- `lang/generated/rust_derived/hakorune_mir_builder/mirbuilder_current_module_take.hako`
- `lang/generated/rust_derived/hakorune_mir_builder/mirbuilder_current_module_take.artifact.json`

## Non-Claims

```text
verify_typed_values = 0
current_function_take = 0
full_finalize_module = 0
module_metadata_publication = 0
mainline_selected = 0
source_selfhost_claim = 0
runtime_fallback = 0
new_backend_route = 0
new_abi = 0
new_canonical_mir_instruction = 0
```

## Gates

```text
python3 -m py_compile tools/rust_lifecycle/mirbuilder_current_module_take_artifacts.py
python3 tools/rust_lifecycle/convert_mirbuilder_lightweight_facts.py --family mirbuilder-current-module-take --check
bash tools/checks/rust_lifecycle_mirbuilder_current_module_take_derived_artifact_guard.sh
bash tools/checks/rust_lifecycle_mirbuilder_minimal_execution_path_semantic_closure_report_guard.sh
bash tools/checks/current_state_pointer_guard.sh
bash tools/checks/rust_lifecycle_no_silent_hardcode_guard.sh
cargo check --release
bash tools/checks/rust_mirbuilder_converter_matrix_guard.sh
```
