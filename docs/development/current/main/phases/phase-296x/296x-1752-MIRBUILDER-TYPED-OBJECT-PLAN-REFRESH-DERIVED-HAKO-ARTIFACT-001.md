---
Status: Selected
Date: 2026-06-26
Card: MIRBUILDER-TYPED-OBJECT-PLAN-REFRESH-DERIVED-HAKO-ARTIFACT-001
---

# MIRBUILDER-TYPED-OBJECT-PLAN-REFRESH-DERIVED-HAKO-ARTIFACT-001

## Summary

`finalize_module.record_packed_layout_refresh` has already been decomposed, and
the analyzer now reports `finalize_module.typed_object_plan_refresh` as the
first leaf owner. This slice materializes that leaf into a checked-in
DerivedShadow Hako artifact. It consumes the already landed plan-only
capability from `MIRBUILDER-TYPED-OBJECT-PLAN-REFRESH-001` and does not
reopen record/packed layout decomposition, direct-state refresh,
typed-object field value refresh, typed-object collection field element
refresh, all-functions PHI materialization, full finalize, backend routing,
ABI, or runtime fallback.

## Authority

Semantic source:

```text
MirBuilderTypedObjectPlanRefreshPlanV1
  -> TypedObjectPlanRefreshExecutionProjectionV1
  -> Hako shadow semantic projector
  -> VerifiedHakoFamilyIR
  -> derived Hako artifact
```

Existing source files:

```text
src/mir/typed_object_plan.rs::refresh_module_typed_object_plans
src/mir/builder/module_lifecycle.rs::MirBuilder::finalize_module
docs/development/current/main/phases/phase-296x/296x-1716-MIRBUILDER-TYPED-OBJECT-PLAN-REFRESH-001.md
docs/development/current/main/phases/phase-296x/296x-1751-MIRBUILDER-RECORD-PACKED-LAYOUT-REFRESH-EXECUTION-DECOMPOSITION-001.md
docs/development/current/main/design/fixtures/rust-lifecycle/minimal-mirbuilder-execution-path-semantic-closure-report-v0.json
```

The derived artifact must consume the landed plan and the analyzer-derived leaf
selection. It must not rescan source syntax or infer child ownership from
generated Hako.

Python remains oracle / fixture / guard orchestration for this card. New Python
semantic projector growth is not allowed; the projection should land in the
compiler library / Hako shadow lane.

## Selected Scope

```text
timing:
  AfterRecordPackedLayoutRefreshBeforeDirectStateRefresh

operation:
  AssignTypedObjectPlans

source:
  build_typed_object_plans(module)

target:
  module.metadata.typed_object_plans

module_transport:
  MirModuleMinimalShell
```

## Derived Artifact Shape

```text
TypedObjectPlanRefreshExecutionProjectionV1:
  source_plan:
    MirBuilderTypedObjectPlanRefreshPlanV1

  leaf_owner:
    finalize_module.typed_object_plan_refresh

  projector_lane:
    Hako shadow semantic projector

  result_surface:
    derived Hako artifact
```

## Expected New Surface

```text
tools/rust_lifecycle/mirbuilder_typed_object_plan_refresh_artifacts.py

docs/development/current/main/design/fixtures/rust-lifecycle/
  mirbuilder-typed-object-plan-refresh-execution-projection-v0.json
  mirbuilder-typed-object-plan-refresh-derived-hako-oracle-v0.json
  mirbuilder-typed-object-plan-refresh-derived-hako-recipe-v0.json
  mirbuilder-typed-object-plan-refresh-derived-hako-verifier-result-v0.json

lang/src/compiler/lib/
  typed_object_plan_refresh_projector.hako

lang/generated/rust_derived/hakorune_mir_builder/
  mirbuilder_typed_object_plan_refresh.hako
  mirbuilder_typed_object_plan_refresh.artifact.json

tools/checks/
  rust_lifecycle_mirbuilder_typed_object_plan_refresh_derived_artifact_guard.sh
```

## Acceptance

```text
python3 tools/rust_lifecycle/convert_mirbuilder_lightweight_facts.py --family mirbuilder-typed-object-plan-refresh --check = green
python3 tools/rust_lifecycle/mirbuilder_typed_object_plan_refresh_artifacts.py = green
Hako shadow projector canonical JSON parity = green
bash tools/checks/rust_lifecycle_mirbuilder_typed_object_plan_refresh_derived_artifact_guard.sh = green
bash tools/checks/rust_lifecycle_mirbuilder_minimal_execution_path_semantic_closure_report_guard.sh = green
bash tools/checks/current_state_pointer_guard.sh = green
bash tools/checks/rust_lifecycle_no_silent_hardcode_guard.sh = green
bash tools/checks/rust_mirbuilder_converter_matrix_guard.sh = green
cargo check --release = green
git diff --check = green
```

Minimum oracle vectors:

```text
leaf_projection:
  typed_object_plan_refresh = true
  direct_state_plan_refresh = false
  all_functions_phi_materialization = false

module_metadata:
  typed_object_plans assigned from build_typed_object_plans(module)
  source order preserved
  no aliasing across publication
```

## Non-Claims

```text
typed_object_field_value_type_refresh = 0
typed_object_collection_field_element_refresh = 0
direct_state_plan_refresh = 0
all_functions_phi_materialization = 0
full_finalize_module = 0
mainline_selected = 0
source_selfhost_claim = 0
runtime_fallback = 0
new_backend_route = 0
new_abi = 0
full record/packed composite artifact = 0
```

## Next

```text
MIRBUILDER-DIRECT-STATE-PLAN-REFRESH-DERIVED-HAKO-ARTIFACT-001
```
