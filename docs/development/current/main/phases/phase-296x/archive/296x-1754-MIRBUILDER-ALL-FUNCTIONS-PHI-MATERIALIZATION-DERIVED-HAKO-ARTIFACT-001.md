---
Status: Landed
Date: 2026-06-27
Card: MIRBUILDER-ALL-FUNCTIONS-PHI-MATERIALIZATION-DERIVED-HAKO-ARTIFACT-001
---

# MIRBUILDER-ALL-FUNCTIONS-PHI-MATERIALIZATION-DERIVED-HAKO-ARTIFACT-001

## Summary

`AllFunctionsPhiMaterialization` is now the next leaf owner under the
record/packed layout decomposition after the direct-state leaf landed as a
checked-in DerivedShadow Hako artifact. This slice materializes
`finalize_module.all_functions_phi_materialization` into a checked-in
DerivedShadow Hako artifact. It consumes the already landed plan-only
capability from `MIRBUILDER-ALL-FUNCTIONS-PHI-MATERIALIZATION-001` and does
not reopen completion design stop, full finalize, generated Hako, backend
routes, ABI changes, runtime fallback, or source selfhost.

## Source Authority

- `src/mir/builder/module_lifecycle.rs::MirBuilder::finalize_module`
- `docs/development/current/main/phases/phase-296x/296x-1753-MIRBUILDER-DIRECT-STATE-PLAN-REFRESH-DERIVED-HAKO-ARTIFACT-001.md`
- `docs/development/current/main/design/fixtures/rust-lifecycle/minimal-mirbuilder-execution-path-semantic-closure-report-v0.json`
- Predecessor plan:
  `docs/development/current/main/phases/phase-296x/296x-1718-MIRBUILDER-ALL-FUNCTIONS-PHI-MATERIALIZATION-001.md`

The derived artifact must consume the landed direct-state leaf and the
analyzer-derived all-functions-PHI leaf selection. It must not rescan source
syntax or infer child ownership from generated Hako.

Python remains oracle / fixture / guard orchestration for this card. New
Python semantic projector growth is not allowed; the projection should land in
the compiler library / Hako shadow lane.

## Selected Scope

```text
iteration:
  for function in module.functions.values_mut()

delegate:
  phi_input_materializer::materialize_all_phi_inputs

delegate_context:
  finalize_module_all_functions

delegate_capability:
  PhiInputMaterialization

error_transport:
  ResultPropagatedByQuestionMark
```

## Derived Artifact Shape

```text
AllFunctionsPhiMaterializationExecutionProjectionV1:
  source_plan:
    MirBuilderAllFunctionsPhiMaterializationPlanV1

  leaf_owner:
    finalize_module.all_functions_phi_materialization

  projector_lane:
    Hako shadow semantic projector

  result_surface:
    derived Hako artifact
```

## Expected New Surface

```text
tools/rust_lifecycle/mirbuilder_all_functions_phi_materialization_artifacts.py

docs/development/current/main/design/fixtures/rust-lifecycle/
  mirbuilder-all-functions-phi-materialization-execution-projection-v0.json
  mirbuilder-all-functions-phi-materialization-derived-hako-oracle-v0.json
  mirbuilder-all-functions-phi-materialization-derived-hako-recipe-v0.json
  mirbuilder-all-functions-phi-materialization-derived-hako-verifier-result-v0.json

lang/src/compiler/lib/
  all_functions_phi_materialization_projector.hako

lang/generated/rust_derived/hakorune_mir_builder/
  mirbuilder_all_functions_phi_materialization.hako
  mirbuilder_all_functions_phi_materialization.artifact.json

tools/checks/
  rust_lifecycle_mirbuilder_all_functions_phi_materialization_derived_artifact_guard.sh
```

## Acceptance

```text
python3 tools/rust_lifecycle/convert_mirbuilder_lightweight_facts.py --family mirbuilder-all-functions-phi-materialization --check = green
python3 tools/rust_lifecycle/mirbuilder_all_functions_phi_materialization_artifacts.py = green
Hako shadow projector canonical JSON parity = green
bash tools/checks/rust_lifecycle_mirbuilder_all_functions_phi_materialization_derived_artifact_guard.sh = green
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
  all_functions_phi_materialization = true
  direct_state_plan_refresh = false
  record_packed_layout_refresh = false

module_metadata:
  all_functions sweep is explicit over module.functions.values_mut()
  delegate context is finalize_module_all_functions
  source order preserved
```

## Non-Claims

```text
direct_state_plan_refresh = 0
record_packed_layout_refresh = 0
full_finalize_module = 0
generated_hako_artifact = 0
backend_route_changed = 0
abi_changed = 0
runtime_fallback = 0
mainline_selected = 0
source_selfhost_claim = 0
```

## Next

```text
MIRBUILDER-RECORD-PACKED-LAYOUT-REFRESH-DERIVED-HAKO-ARTIFACT-001
```
