---
Status: Landed
Date: 2026-06-26
Card: MIRBUILDER-RETURN-TYPE-PUBLICATION-DERIVED-HAKO-ARTIFACT-001
---

# MIRBUILDER-RETURN-TYPE-PUBLICATION-DERIVED-HAKO-ARTIFACT-001

## Summary

`MirBuilder::finalize_module` return type publication now has a focused
DerivedShadow Hako artifact for the prepared minimal profile. The artifact
materializes only:

```text
lookup result_value type shell
publish MirType::Integer to function signature shell
record publication source ValueId
```

It does not take the current module, verify typed values, run full finalize,
infer PHI return types, or select mainline execution.

## Authority

Semantic source:

```text
MirBuilderReturnTypePublicationPlanV1
  -> ReturnTypePublication DerivedShadow artifact
```

Generated artifact:

```text
lang/generated/rust_derived/hakorune_mir_builder/mirbuilder_return_type_publication.hako
lang/generated/rust_derived/hakorune_mir_builder/mirbuilder_return_type_publication.artifact.json
```

The artifact is an executable materialization of the existing
ReturnTypePublication plan. It is not a full `finalize_module` artifact and
does not reinterpret module take or typed-value verification.

## Acceptance

```text
deterministic regeneration = green
generated Hako parse/MIR = green
generated Hako EXE/AOT = green
ReturnTypePublicationApi.publish/3 direct same-module route = green
definition_owner = uniform_mir
result box = ReturnTypeFunctionSignatureShellBox
return_type_publication = 1
module_take = 0
verify_typed_values = 0
full_finalize_module = 0
runtime_fallback = 0
```

The semantic closure report now consumes this artifact and advances the first
executable materialization gap:

```text
first_executable_materialization_gap = finalize_module.take_module
next_slice = MIRBUILDER-CURRENT-MODULE-TAKE-DERIVED-HAKO-ARTIFACT-001
```

## Artifacts

- `tools/rust_lifecycle/mirbuilder_return_type_publication_artifacts.py`
- `tools/checks/rust_lifecycle_mirbuilder_return_type_publication_derived_artifact_guard.sh`
- `docs/development/current/main/design/fixtures/rust-lifecycle/mirbuilder-return-type-publication-derived-hako-oracle-v0.json`
- `docs/development/current/main/design/fixtures/rust-lifecycle/mirbuilder-return-type-publication-derived-hako-recipe-v0.json`
- `docs/development/current/main/design/fixtures/rust-lifecycle/mirbuilder-return-type-publication-derived-hako-verifier-result-v0.json`
- `lang/generated/rust_derived/hakorune_mir_builder/mirbuilder_return_type_publication.hako`
- `lang/generated/rust_derived/hakorune_mir_builder/mirbuilder_return_type_publication.artifact.json`

## Non-Claims

```text
module_take = 0
verify_typed_values = 0
full_finalize_module = 0
phi_return_type_inference = 0
mainline_selected = 0
source_selfhost_claim = 0
runtime_fallback = 0
new_backend_route = 0
new_abi = 0
new_canonical_mir_instruction = 0
```

## Gates

```text
python3 -m py_compile tools/rust_lifecycle/mirbuilder_return_type_publication_artifacts.py
python3 tools/rust_lifecycle/convert_mirbuilder_lightweight_facts.py --family mirbuilder-return-type-publication --check
bash tools/checks/rust_lifecycle_mirbuilder_return_type_publication_derived_artifact_guard.sh
bash tools/checks/rust_lifecycle_mirbuilder_minimal_execution_path_semantic_closure_report_guard.sh
bash tools/checks/current_state_pointer_guard.sh
bash tools/checks/rust_lifecycle_no_silent_hardcode_guard.sh
cargo check --release
bash tools/checks/rust_mirbuilder_converter_matrix_guard.sh
```
