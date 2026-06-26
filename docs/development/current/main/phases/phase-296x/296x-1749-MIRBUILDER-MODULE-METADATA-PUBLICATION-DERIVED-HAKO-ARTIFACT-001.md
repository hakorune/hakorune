---
Status: Selected
Date: 2026-06-26
Card: MIRBUILDER-MODULE-METADATA-PUBLICATION-DERIVED-HAKO-ARTIFACT-001
---

# MIRBUILDER-MODULE-METADATA-PUBLICATION-DERIVED-HAKO-ARTIFACT-001

## Summary

Materialize the analyzer-derived first executable gap:

```text
edge_id: finalize_module.module_metadata_publication
callsite: MirBuilder::finalize_module -> module metadata publication
next_slice: MIRBUILDER-MODULE-METADATA-PUBLICATION-DERIVED-HAKO-ARTIFACT-001
```

The PlanOnly authority is already landed in
`MIRBUILDER-MODULE-METADATA-PUBLICATION-001`. This card does not reselect the
semantics. It only projects that plan into a checked-in derived Hako artifact.

## Authority

Semantic source:

```text
MirBuilderModuleMetadataPublicationPlanV1
  -> ModuleMetadataPublicationExecutionProjectionV1
  -> VerifiedHakoFamilyIR
  -> derived Hako artifact
```

Existing authority files:

```text
tools/rust_lifecycle/mirbuilder_module_metadata_publication.py
docs/development/current/main/design/fixtures/rust-lifecycle/mirbuilder-module-metadata-publication-plan-v0.json
tools/checks/rust_lifecycle_mirbuilder_module_metadata_publication_guard.sh
```

The derived artifact must consume the plan. It must not rescan source syntax,
reinterpret finalize ordering, or infer publication fields from generated Hako.

## Selected Scope

```text
timing:
  AfterSlotRegistryReleaseBeforeSemanticRefresh

mutates:
  module.metadata.user_box_decls
  module.metadata.user_box_field_decls
  module.metadata.record_decls
  module.metadata.enum_decls

publication:
  user_box_decls <- comp_ctx.user_defined_boxes.clone()
  user_box_field_decls <- comp_ctx.user_box_field_decls mapped to UserBoxFieldDecl
  record_decls <- comp_ctx.record_decls.clone().into_iter().collect()
  enum_decls <- comp_ctx.enum_decls_for_module_metadata()
```

## Expected New Surface

```text
tools/rust_lifecycle/mirbuilder_module_metadata_publication_artifacts.py

docs/development/current/main/design/fixtures/rust-lifecycle/
  mirbuilder-module-metadata-publication-execution-projection-v0.json
  mirbuilder-module-metadata-publication-derived-hako-oracle-v0.json
  mirbuilder-module-metadata-publication-derived-hako-recipe-v0.json
  mirbuilder-module-metadata-publication-derived-hako-verifier-result-v0.json

lang/generated/rust_derived/hakorune_mir_builder/
  mirbuilder_module_metadata_publication.hako
  mirbuilder_module_metadata_publication.artifact.json

tools/checks/
  rust_lifecycle_mirbuilder_module_metadata_publication_derived_artifact_guard.sh
```

Expected registration/update points:

```text
tools/rust_lifecycle/convert_mirbuilder_lightweight_facts.py
tools/rust_lifecycle/mirbuilder_minimal_execution_path_semantic_closure_report.py
docs/development/current/main/design/fixtures/rust-lifecycle/minimal-mirbuilder-execution-path-semantic-closure-report-v0.json
tools/checks/rust_lifecycle_mirbuilder_minimal_execution_path_semantic_closure_report_guard.sh
tools/checks/rust_mirbuilder_converter_matrix_guard.sh
docs/tools/check-scripts-index.md
docs/development/current/main/CURRENT_STATE.toml
docs/development/current/main/design/mirbuilder-rust-to-hako-converter-task-order-ssot.md
```

## Acceptance

```text
python3 tools/rust_lifecycle/convert_mirbuilder_lightweight_facts.py --family mirbuilder-module-metadata-publication --check = green
python3 tools/rust_lifecycle/mirbuilder_module_metadata_publication_artifacts.py = green
python3 tools/rust_lifecycle/mirbuilder_minimal_execution_path_semantic_closure_report.py --check-reference --drift-probes = green
bash tools/checks/rust_lifecycle_mirbuilder_module_metadata_publication_derived_artifact_guard.sh = green
bash tools/checks/rust_lifecycle_mirbuilder_minimal_execution_path_semantic_closure_report_guard.sh = green
bash tools/checks/current_state_pointer_guard.sh = green
bash tools/checks/rust_lifecycle_no_silent_hardcode_guard.sh = green
bash tools/checks/rust_mirbuilder_converter_matrix_guard.sh = green
cargo check --release = green
git diff --check = green
```

## Frontier Rule

After the derived artifact lands, regenerate the semantic-closure report. The
next executable materialization gap must be analyzer-derived from the updated
report, not hand-written into the phase card or task-order first.

Expected but non-authoritative current successor:

```text
finalize_module.record_packed_layout_refresh
```

## Non-Claims

```text
semantic_refresh = 0
record_and_packed_layout_refresh = 0
typed_object_plan_refresh = 0
direct_state_plan_refresh = 0
all_functions_phi_materialization = 0
full_finalize_module = 0
new_backend_route = 0
new_abi = 0
runtime_fallback = 0
source_selfhost_claim = 0
mainline_selected = 0
coverage_percentage_as_proof = 0
bundle_size_as_proof = 0
```
