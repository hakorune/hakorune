---
Status: Landed
Date: 2026-06-27
Card: MIRBUILDER-DIRECT-STATE-PLAN-REFRESH-DERIVED-HAKO-ARTIFACT-001
---

# MIRBUILDER-DIRECT-STATE-PLAN-REFRESH-DERIVED-HAKO-ARTIFACT-001

## Summary

`DirectStatePlanRefresh` is now a checked-in DerivedShadow Hako artifact.
The analyzer-derived next unsupported edge is
`finalize_module.all_functions_phi_materialization`, and the slice consumes
the already landed plan-only capability from
`MIRBUILDER-DIRECT-STATE-PLAN-REFRESH-001`. It does not reopen all-functions
PHI materialization, direct-state lowering, route selection, NativeDirect
guards, full finalize, generated Hako, backend routes, ABI changes, runtime
fallback, or source selfhost.

## Source Authority

- `src/mir/builder/module_lifecycle.rs::MirBuilder::finalize_module`
- `src/mir/direct_state_plan.rs::refresh_module_direct_state_plans`
- `docs/development/current/main/phases/phase-296x/296x-1752-MIRBUILDER-TYPED-OBJECT-PLAN-REFRESH-DERIVED-HAKO-ARTIFACT-001.md`
- `docs/development/current/main/design/fixtures/rust-lifecycle/minimal-mirbuilder-execution-path-semantic-closure-report-v0.json`
- Predecessor plan:
  `docs/development/current/main/phases/phase-296x/296x-1717-MIRBUILDER-DIRECT-STATE-PLAN-REFRESH-001.md`

The derived artifact must consume the landed typed-object leaf and the
analyzer-derived direct-state leaf selection. It must not rescan source syntax
or infer child ownership from generated Hako.

Python remains oracle / fixture / guard orchestration for this card. New Python
semantic projector growth is not allowed; the projection should land in the
compiler library / Hako shadow lane.

## Selected Scope

```text
timing:
  AfterTypedObjectPlanRefreshBeforeAllFunctionsPhiMaterialization

operation:
  AssignDirectStatePlans

source:
  build_direct_state_plans(module)

target:
  module.metadata.direct_state_plans

module_transport:
  MirModuleMinimalShell
```

## Derived Artifact Shape

```text
DirectStatePlanRefreshExecutionProjectionV1:
  source_plan:
    MirBuilderDirectStatePlanRefreshPlanV1

  leaf_owner:
    finalize_module.direct_state_plan_refresh

  projector_lane:
    Hako shadow semantic projector

  result_surface:
    derived Hako artifact
```

## Expected New Surface

```text
tools/rust_lifecycle/mirbuilder_direct_state_plan_refresh_artifacts.py

docs/development/current/main/design/fixtures/rust-lifecycle/
  mirbuilder-direct-state-plan-refresh-execution-projection-v0.json
  mirbuilder-direct-state-plan-refresh-derived-hako-oracle-v0.json
  mirbuilder-direct-state-plan-refresh-derived-hako-recipe-v0.json
  mirbuilder-direct-state-plan-refresh-derived-hako-verifier-result-v0.json

lang/src/compiler/lib/
  direct_state_plan_refresh_projector.hako

lang/generated/rust_derived/hakorune_mir_builder/
  mirbuilder_direct_state_plan_refresh.hako
  mirbuilder_direct_state_plan_refresh.artifact.json

tools/checks/
  rust_lifecycle_mirbuilder_direct_state_plan_refresh_derived_artifact_guard.sh
```

## Acceptance

```text
python3 tools/rust_lifecycle/convert_mirbuilder_lightweight_facts.py --family mirbuilder-direct-state-plan-refresh --check = green
python3 tools/rust_lifecycle/mirbuilder_direct_state_plan_refresh_artifacts.py = green
Hako shadow projector canonical JSON parity = green
bash tools/checks/rust_lifecycle_mirbuilder_direct_state_plan_refresh_derived_artifact_guard.sh = green
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
  direct_state_plan_refresh = true
  typed_object_plan_refresh = false
  all_functions_phi_materialization = false

module_metadata:
  direct_state_plans assigned from build_direct_state_plans(module)
  source order preserved
  no aliasing across publication
```

## Non-Claims

```text
typed_object_plan_refresh = 0
all_functions_phi_materialization = 0
direct_state_lowering = 0
route_selection = 0
native_direct_guard = 0
full_semantic_refresh = 0
full_finalize_module = 0
generated_hako_artifact = 0
backend_route_changed = 0
abi_changed = 0
runtime_fallback = 0
mainline_selected = 0
```

## Next

```text
MIRBUILDER-ALL-FUNCTIONS-PHI-MATERIALIZATION-DERIVED-HAKO-ARTIFACT-001
```
