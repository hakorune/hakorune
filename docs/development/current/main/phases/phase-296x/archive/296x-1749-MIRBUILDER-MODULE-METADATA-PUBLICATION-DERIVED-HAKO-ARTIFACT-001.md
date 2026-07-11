---
Status: Landed
Date: 2026-06-26
Card: MIRBUILDER-MODULE-METADATA-PUBLICATION-DERIVED-HAKO-ARTIFACT-001
---

# MIRBUILDER-MODULE-METADATA-PUBLICATION-DERIVED-HAKO-ARTIFACT-001

## Closeout

The derived artifact is checked in and the semantic-closure report now treats
`ModuleMetadataPublication` as executable artifact evidence.

The regenerated report advances the first executable materialization gap to:

```text
edge_id: finalize_module.record_packed_layout_refresh
next_slice: MIRBUILDER-RECORD-PACKED-LAYOUT-REFRESH-EXECUTION-DECOMPOSITION-001
```

The immediate follow-up is the allocation-policy Hako adoption decision. The
record/packed refresh edge remains parked until that decision is recorded.

## Summary

Materialize the analyzer-derived first executable gap:

```text
edge_id: finalize_module.module_metadata_publication
callsite: MirBuilder::finalize_module -> module metadata publication
next_slice: MIRBUILDER-MODULE-METADATA-PUBLICATION-DERIVED-HAKO-ARTIFACT-001
```

The PlanOnly authority is already landed in
`MIRBUILDER-MODULE-METADATA-PUBLICATION-001`. This card does not reselect the
top-level semantics. It projects that plan into a checked-in derived Hako
artifact, but the projection must be explicit enough for execution: publication
fields are split into child projections instead of being left as opaque
`CloneIntoIterCollect` / `FunctionCall` labels.

## Authority

Semantic source:

```text
MirBuilderModuleMetadataPublicationPlanV1
  -> ModuleMetadataPublicationExecutionProjectionV1
  -> Hako shadow semantic projector
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

Python remains oracle / emitter / guard orchestration for this card. New
Python semantic projector growth is not allowed; the semantic projection for
this family is exercised through a Hako shadow projector.

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

## Child Projections

The execution projection owns a four-field replace snapshot. It is not a
merge/update operation.

```text
ModuleMetadataPublicationExecutionProjectionV1:
  update_mode = ReplaceSnapshot

  subprojections:
    user_box_decls:
      UserBoxNameInventoryOwnedProjectionV1

    user_box_field_decls:
      UserBoxFieldDeclProjectionV1

    record_decls:
      RecordDeclOwnedSnapshotProjectionV1

    enum_decls:
      EnumDeclModuleMetadataProjectionV1
```

Projection requirements:

```text
UserBoxNameInventoryOwnedProjectionV1:
  String key -> owned ArrayBox of owned String values
  order_observed = 0

UserBoxFieldDeclProjectionV1:
  selected fields = name, declared_type_name, is_weak
  omitted fields = default_value
  declared_type_name = Option<StringBox>
  is_weak = bool, not anonymous raw i64

RecordDeclOwnedSnapshotProjectionV1:
  clone_depth = DeepOwned
  key_order = CanonicalKeyOrder
  type_parameters = owned array copy
  fields = UserBoxFieldDeclProjectionV1 each

EnumDeclModuleMetadataProjectionV1:
  EnumDeclLocal -> MirEnumDecl
  variants -> MirEnumVariantDecl { name, payload_type_name }
  container = canonical module metadata order
```

The enum projection stays inside this top-level owner because the source block
publishes it as one of the same four metadata fields, no intermediate state is
observed, and there is no independent side effect.

## Expected New Surface

```text
tools/rust_lifecycle/mirbuilder_module_metadata_publication_artifacts.py

docs/development/current/main/design/fixtures/rust-lifecycle/
  mirbuilder-module-metadata-publication-execution-projection-v0.json
  mirbuilder-module-metadata-publication-derived-hako-oracle-v0.json
  mirbuilder-module-metadata-publication-hako-shadow-result-v0.json
  mirbuilder-module-metadata-publication-derived-hako-recipe-v0.json
  mirbuilder-module-metadata-publication-derived-hako-verifier-result-v0.json

lang/src/compiler/lib/
  module_metadata_publication_projector.hako

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
Hako shadow projector canonical JSON parity = green
python3 tools/rust_lifecycle/mirbuilder_minimal_execution_path_semantic_closure_report.py --check-reference --drift-probes = green
bash tools/checks/rust_lifecycle_mirbuilder_module_metadata_publication_derived_artifact_guard.sh = green
bash tools/checks/rust_lifecycle_mirbuilder_minimal_execution_path_semantic_closure_report_guard.sh = green
bash tools/checks/current_state_pointer_guard.sh = green
bash tools/checks/rust_lifecycle_no_silent_hardcode_guard.sh = green
bash tools/checks/rust_mirbuilder_converter_matrix_guard.sh = green
cargo check --release = green
git diff --check = green
```

Minimum oracle vectors:

```text
minimal_literal_0:
  user boxes empty
  records empty
  prelude enums preserved

user_box_fields:
  declared_type_name = None
  declared_type_name = Some("Integer")
  is_weak = false / true
  default_value omitted

generic_record:
  type parameters preserved
  multiple fields
  canonical key order

custom_enum:
  unit variant
  payload variant
  type parameters preserved

replacement:
  stale target rows removed

alias_isolation:
  source mutation after publication does not affect target
  target mutation after publication does not affect source
```

## Frontier Rule

After the derived artifact lands, regenerate the semantic-closure report. The
next executable materialization gap must be analyzer-derived from the updated
report, not hand-written into the phase card or task-order first.

Expected but non-authoritative current successor:

```text
finalize_module.record_packed_layout_refresh
```

If that successor remains `RecordAndPackedLayoutRefresh`, do not implement it
as one large artifact. First split it with a directability/decomposition card:

```text
MIRBUILDER-RECORD-PACKED-LAYOUT-REFRESH-EXECUTION-DECOMPOSITION-001
```

The recommended D+ follow-on order is:

```text
1. close this ModuleMetadataPublication derived artifact
2. regenerate the frontier report
3. run MIRBUILDER-ALLOCATION-POLICY-HAKO-ADOPTION-DECISION-001
4. decompose RecordAndPackedLayoutRefresh before derived artifact work
5. continue from the first analyzer-derived child owner
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
new_python_semantic_projector = 0
source_selfhost_claim = 0
mainline_selected = 0
coverage_percentage_as_proof = 0
bundle_size_as_proof = 0
```
