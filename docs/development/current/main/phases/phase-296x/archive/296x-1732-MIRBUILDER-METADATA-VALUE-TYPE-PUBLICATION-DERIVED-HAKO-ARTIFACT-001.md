---
Status: Landed
Date: 2026-06-26
Card: MIRBUILDER-METADATA-VALUE-TYPE-PUBLICATION-DERIVED-HAKO-ARTIFACT-001
---

# MIRBUILDER-METADATA-VALUE-TYPE-PUBLICATION-DERIVED-HAKO-ARTIFACT-001

## Summary

`MirBuilder::finalize_module` metadata value-type publication now has a
focused DerivedShadow Hako artifact for the prepared minimal profile. The
artifact materializes only this source-derived assignment:

```text
function.metadata.value_types = self.type_ctx.value_types.clone()
```

It records the clone-owned publication frame from `self.type_ctx.value_types`
to `function.metadata.value_types` and returns a
`MetadataValueTypePublicationResultBox`. It does not merge origin callers,
infer PHI return types, materialize PHI inputs, insert module functions, or
claim full finalize.

## Authority

Semantic source:

```text
MirBuilderMetadataValueTypePublicationPlanV1
  -> MetadataValueTypePublication DerivedShadow artifact
```

Generated artifact:

```text
lang/generated/rust_derived/hakorune_mir_builder/mirbuilder_metadata_value_type_publication.hako
lang/generated/rust_derived/hakorune_mir_builder/mirbuilder_metadata_value_type_publication.artifact.json
```

The artifact is an executable materialization of the existing metadata
value-type publication plan. It is not origin-caller merge or full
finalize_module.

## Acceptance

```text
deterministic regeneration = green
generated Hako parse/MIR = green
generated Hako EXE/AOT = green
MetadataValueTypePublicationApi.publish/2 direct same-module route = green
proof = typed_global_call_same_module_object_handle
definition_owner = uniform_mir
result box = MetadataValueTypePublicationResultBox
metadata_value_type_publication = 1
metadata_origin_caller_merge = 0
full_finalize_module = 0
runtime_fallback = 0
```

The semantic closure report now consumes this artifact and advances the first
executable materialization gap:

```text
first_executable_materialization_gap = finalize_module.metadata_origin_caller_merge
next_slice = MIRBUILDER-METADATA-ORIGIN-CALLER-MERGE-DERIVED-HAKO-ARTIFACT-001
```

## Artifacts

- `tools/rust_lifecycle/mirbuilder_metadata_value_type_publication_artifacts.py`
- `tools/checks/rust_lifecycle_mirbuilder_metadata_value_type_publication_derived_artifact_guard.sh`
- `docs/development/current/main/design/fixtures/rust-lifecycle/mirbuilder-metadata-value-type-publication-derived-hako-oracle-v0.json`
- `docs/development/current/main/design/fixtures/rust-lifecycle/mirbuilder-metadata-value-type-publication-derived-hako-recipe-v0.json`
- `docs/development/current/main/design/fixtures/rust-lifecycle/mirbuilder-metadata-value-type-publication-derived-hako-verifier-result-v0.json`
- `lang/generated/rust_derived/hakorune_mir_builder/mirbuilder_metadata_value_type_publication.hako`
- `lang/generated/rust_derived/hakorune_mir_builder/mirbuilder_metadata_value_type_publication.artifact.json`

## Non-Claims

```text
metadata_origin_caller_merge = 0
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
python3 -m py_compile tools/rust_lifecycle/mirbuilder_metadata_value_type_publication_artifacts.py
python3 tools/rust_lifecycle/convert_mirbuilder_lightweight_facts.py --family mirbuilder-metadata-value-type-publication --check
bash tools/checks/rust_lifecycle_mirbuilder_metadata_value_type_publication_derived_artifact_guard.sh
bash tools/checks/rust_lifecycle_mirbuilder_minimal_execution_path_semantic_closure_report_guard.sh
bash tools/checks/current_state_pointer_guard.sh
bash tools/checks/rust_lifecycle_no_silent_hardcode_guard.sh
cargo check --release
bash tools/checks/rust_mirbuilder_converter_matrix_guard.sh
```
