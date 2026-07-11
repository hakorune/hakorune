---
Status: Landed
Date: 2026-06-27
Card: MIRBUILDER-RECORD-PACKED-LAYOUT-REFRESH-DERIVED-HAKO-ARTIFACT-001
---

# MIRBUILDER-RECORD-PACKED-LAYOUT-REFRESH-DERIVED-HAKO-ARTIFACT-001

## Summary

`finalize_module.record_packed_layout_refresh` has been materialized as the
checked-in DerivedShadow Hako artifact. The composite edge remains explicit in
the frontier model, and this slice keeps the ordered child owners visible
instead of collapsing them into one large leaf.

This slice consumes the plan-only capability from
`MIRBUILDER-RECORD-PACKED-LAYOUT-REFRESH-001` and the decomposition evidence
from `MIRBUILDER-RECORD-PACKED-LAYOUT-REFRESH-EXECUTION-DECOMPOSITION-001`.
It does not reopen typed-object refresh, direct-state refresh, all-functions
PHI materialization, full finalize, generated Hako ownership, backend routes,
ABI changes, runtime fallback, or source selfhost.

## Source Authority

- `src/mir/builder/module_lifecycle.rs::MirBuilder::finalize_module`
- `src/mir/semantic_refresh.rs::refresh_module_record_and_packed_layout_plans`
- `docs/development/current/main/phases/phase-296x/296x-1751-MIRBUILDER-RECORD-PACKED-LAYOUT-REFRESH-EXECUTION-DECOMPOSITION-001.md`
- `docs/development/current/main/design/fixtures/rust-lifecycle/minimal-mirbuilder-execution-path-semantic-closure-report-v0.json`
- Predecessor plan:
  `docs/development/current/main/phases/phase-296x/296x-1714-MIRBUILDER-MODULE-METADATA-PUBLICATION-001.md`

The derived artifact must consume the landed decomposition and the analyzer-
derived composite boundary. It must not rescan source syntax or infer child
ownership from generated Hako.

Python remains oracle / fixture / guard orchestration for this card. New
Python semantic projector growth is not allowed; the projection should land in
the compiler library / Hako shadow lane.

## Selected Scope

```text
entrypoint:
  refresh_module_record_and_packed_layout_plans

timing:
  AfterModuleMetadataPublicationBeforeTypedObjectRefresh

module_arg:
  &mut MirModule

steps:
  refresh_module_record_layout_plans
  refresh_module_array_record_storage_plans
  refresh_module_array_record_autouse_eligibility_plans
  refresh_module_array_record_materialization_boundary_plans
  refresh_module_array_record_packed_autouse_pilot_plans
  refresh_module_source_packed_array_autouse_pilot_plans
  refresh_module_source_packed_array_direct_read_consumption_plans
  refresh_module_hako_alloc_aligned_small_packed_store_pilot_plans
  refresh_module_hako_alloc_huge_page_packed_store_pilot_plans
```

## Derived Artifact Shape

```text
RecordPackedLayoutRefreshExecutionProjectionV1:
  source_plan:
    MirBuilderRecordPackedLayoutRefreshPlanV1

  composite_owner:
    finalize_module.record_packed_layout_refresh

  projector_lane:
    Hako shadow semantic projector

  result_surface:
    derived Hako artifact
```

## Expected New Surface

```text
tools/rust_lifecycle/mirbuilder_record_packed_layout_refresh_artifacts.py

docs/development/current/main/design/fixtures/rust-lifecycle/
  mirbuilder-record-packed-layout-refresh-execution-projection-v0.json
  mirbuilder-record-packed-layout-refresh-derived-hako-oracle-v0.json
  mirbuilder-record-packed-layout-refresh-derived-hako-recipe-v0.json
  mirbuilder-record-packed-layout-refresh-derived-hako-verifier-result-v0.json

lang/src/compiler/lib/
  record_packed_layout_refresh_projector.hako

lang/generated/rust_derived/hakorune_mir_builder/
  mirbuilder_record_packed_layout_refresh.hako
  mirbuilder_record_packed_layout_refresh.artifact.json

tools/checks/
  rust_lifecycle_mirbuilder_record_packed_layout_refresh_derived_artifact_guard.sh
```

## Acceptance

```text
python3 tools/rust_lifecycle/convert_mirbuilder_lightweight_facts.py --family mirbuilder-record-packed-layout-refresh --check = green
python3 tools/rust_lifecycle/mirbuilder_record_packed_layout_refresh_artifacts.py = green
Hako shadow projector canonical JSON parity = green
bash tools/checks/rust_lifecycle_mirbuilder_record_packed_layout_refresh_derived_artifact_guard.sh = green
bash tools/checks/rust_lifecycle_mirbuilder_minimal_execution_path_semantic_closure_report_guard.sh = green
bash tools/checks/current_state_pointer_guard.sh = green
bash tools/checks/rust_lifecycle_no_silent_hardcode_guard.sh = green
bash tools/checks/rust_mirbuilder_converter_matrix_guard.sh = green
cargo check --release = green
git diff --check = green
```

Minimum oracle vectors:

```text
composite_boundary:
  record_packed_layout_refresh = true
  typed_object_plan_refresh = false
  direct_state_plan_refresh = false
  all_functions_phi_materialization = false

module_metadata:
  refresh steps are explicit and ordered
  source order preserved
```

## Non-Claims

```text
typed_object_plan_refresh = 0
direct_state_plan_refresh = 0
all_functions_phi_materialization = 0
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
MIRBUILDER-MINIMAL-PATH-COMPOSED-EXECUTION-CLOSURE-001
```
