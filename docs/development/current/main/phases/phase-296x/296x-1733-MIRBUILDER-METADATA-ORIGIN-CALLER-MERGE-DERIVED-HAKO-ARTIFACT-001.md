---
Status: Landed
Date: 2026-06-26
Card: MIRBUILDER-METADATA-ORIGIN-CALLER-MERGE-DERIVED-HAKO-ARTIFACT-001
---

# MIRBUILDER-METADATA-ORIGIN-CALLER-MERGE-DERIVED-HAKO-ARTIFACT-001

## Summary

`MirBuilder::finalize_module` metadata origin-caller merge now has a focused
DerivedShadow Hako artifact for the prepared minimal profile. The artifact
materializes only this source-derived operation:

```text
let mut origin_callers = function.metadata.value_origin_callers.clone();
for (k, v) in self.metadata_ctx.value_origin_callers().iter() {
    origin_callers.insert(*k, v.clone());
}
function.metadata.value_origin_callers = origin_callers;
```

The generated artifact uses `ValueIdOrderedMapBox` for ValueId/i64 keys,
preserves SourceWins collision behavior, and validates that the source map
does not alias the merged result after assignment. It does not infer PHI return
types, materialize PHI inputs, insert module functions, or claim full
`finalize_module`.

## Authority

Semantic source:

```text
MirBuilderMetadataOriginCallerMergePlanV1
  -> MetadataOriginCallerMerge DerivedShadow artifact
```

Generated artifact:

```text
lang/generated/rust_derived/hakorune_mir_builder/mirbuilder_metadata_origin_caller_merge.hako
lang/generated/rust_derived/hakorune_mir_builder/mirbuilder_metadata_origin_caller_merge.artifact.json
```

The artifact is an executable materialization of the existing metadata
origin-caller merge plan. It is not PHI return-type inference or full
finalize_module.

## Acceptance

```text
deterministic regeneration = green
generated Hako parse/MIR = green
generated Hako EXE/AOT = green
MetadataOriginCallerMergeApi.merge/2 direct same-module route = green
proof = typed_global_call_same_module_object_handle
definition_owner = uniform_mir
result box = MetadataOriginCallerMergeResultBox
metadata_origin_caller_merge = 1
phi_return_type_inference = 0
full_finalize_module = 0
runtime_fallback = 0
```

The semantic closure report now consumes this artifact and advances the first
executable materialization gap:

```text
first_executable_materialization_gap = finalize_module.phi_return_type_inference
next_slice = MIRBUILDER-PHI-RETURN-TYPE-INFERENCE-DERIVED-HAKO-ARTIFACT-001
```

## Artifacts

- `tools/rust_lifecycle/mirbuilder_metadata_origin_caller_merge_artifacts.py`
- `tools/checks/rust_lifecycle_mirbuilder_metadata_origin_caller_merge_derived_artifact_guard.sh`
- `docs/development/current/main/design/fixtures/rust-lifecycle/mirbuilder-metadata-origin-caller-merge-derived-hako-oracle-v0.json`
- `docs/development/current/main/design/fixtures/rust-lifecycle/mirbuilder-metadata-origin-caller-merge-derived-hako-recipe-v0.json`
- `docs/development/current/main/design/fixtures/rust-lifecycle/mirbuilder-metadata-origin-caller-merge-derived-hako-verifier-result-v0.json`
- `lang/generated/rust_derived/hakorune_mir_builder/mirbuilder_metadata_origin_caller_merge.hako`
- `lang/generated/rust_derived/hakorune_mir_builder/mirbuilder_metadata_origin_caller_merge.artifact.json`

## Non-Claims

```text
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
python3 -m py_compile tools/rust_lifecycle/mirbuilder_metadata_origin_caller_merge_artifacts.py
python3 tools/rust_lifecycle/convert_mirbuilder_lightweight_facts.py --family mirbuilder-metadata-origin-caller-merge --check
bash tools/checks/rust_lifecycle_mirbuilder_metadata_origin_caller_merge_derived_artifact_guard.sh
bash tools/checks/rust_lifecycle_mirbuilder_minimal_execution_path_semantic_closure_report_guard.sh
bash tools/checks/current_state_pointer_guard.sh
bash tools/checks/rust_lifecycle_no_silent_hardcode_guard.sh
cargo check --release
bash tools/checks/rust_mirbuilder_converter_matrix_guard.sh
```
