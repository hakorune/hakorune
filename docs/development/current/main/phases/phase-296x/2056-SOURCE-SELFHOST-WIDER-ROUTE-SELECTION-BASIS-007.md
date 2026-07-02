# 2056 - SOURCE-SELFHOST-WIDER-ROUTE-SELECTION-BASIS-007

## Token

```text
SOURCE-SELFHOST-WIDER-ROUTE-SELECTION-BASIS-007
```

## Status

```text
planned_task_contract = 1
implementation_done = 1
```

## Purpose

Select the next machine-checkable lane after
`MIRBUILDER-ID-SCALAR-PARENT-OWNED-SUBJECT-BOUNDARY-RESOLUTION-001`
classified `mirbuilder::context_registry` as `RemainParentOwned`.

This card is a local mechanical selector. It must not materialize source plans,
native seeds, Hako adoption decisions, or Source Selfhost claims.

## Local Authority

```text
local_selection_authority = LocalMechanicalSelectorAuthorityV1
worker_inventory = consumed
worker_inventory_scope = read_only_current_fixtures_cards_ledgers
```

## Input Authority

```text
current_blocker:
  SOURCE-SELFHOST-WIDER-ROUTE-SELECTION-DESIGN-STOP-001

latest_parent_owned_boundary:
  docs/development/current/main/design/fixtures/rust-lifecycle/
    mirbuilder-id-scalar-parent-owned-subject-boundary-resolution-v0.json

latest_seed_capability_rerun:
  docs/development/current/main/design/fixtures/rust-lifecycle/
    mirbuilder-crate-wide-native-owner-seed-capability-survey-rerun-010-v0.json

current_surface_report:
  docs/development/current/main/design/fixtures/rust-lifecycle/
    mirbuilder-crate-wide-unconverted-surface-report-v0.json

native_owner_manifest:
  docs/development/current/main/design/fixtures/rust-lifecycle/
    source-selfhost-family-guard-manifest-v0.json
```

## Selection Rule

```text
1. Consume the 2055 parent-owned subject boundary result.
   Require:
     classification = RemainParentOwned
     standalone_projection_subject_established = 0
     source_plan_materialization_allowed = 0

2. Check whether the unconverted surface report is fresh after the
   emission_ssa_phi HakoAdopted native-owner delta.

3. If the report is stale:
     decision = SelectUnconvertedSurfaceReportRerun
     reason_token = SourceSurfaceReportStaleAfterEmissionSsaPhiAdoption
     selected_next_card = MIRBUILDER-UNCONVERTED-SURFACE-REPORT-RERUN-004

4. If the report is fresh and the native-owner checkpoint is stale:
     decision = SelectNativeOwnerCheckpointRerun
     reason_token = NativeOwnerCheckpointStaleAfterEmissionSsaPhiAdoption
     selected_next_card = SOURCE-SELFHOST-NATIVE-OWNER-CHECKPOINT-RERUN-002

5. If report and checkpoint are both fresh, select exactly one blocker-class
   lane only from machine-derived checkpoint evidence.

6. If no exactly-one lane exists:
     decision = KeepStopped
```

## Expected Immediate Result

```text
decision = SelectUnconvertedSurfaceReportRerun
reason_token = SourceSurfaceReportStaleAfterEmissionSsaPhiAdoption
selected_next_card = MIRBUILDER-UNCONVERTED-SURFACE-REPORT-RERUN-004
```

The decision is based on the `emission_ssa_phi` adoption after the latest
documented unconverted surface report refresh.

## Forbidden Claims

```text
manual_family_selection = 0
manual_shape_selection = 0
manual_axis_selection = 0
remaining_owner_count_as_proof = 0
owner_name_as_proof = 0
source_symbol_as_proof = 0
source_path_as_authority = 0
keep_parent_owner_as_standalone_proof = 0
projection_descriptor_coverage_as_standalone_proof = 0
cluster_size_as_proof = 0
coverage_percentage_as_proof = 0
generated_artifact_as_native_edit_authority = 0
source_plan_materialization = 0
behavior_recipe_materialization = 0
verifier_result_materialization = 0
derived_artifact_seed_draft_materialization = 0
native_seed_materialization = 0
hako_generation = 0
hako_adopted_decision = 0
source_selfhost_claim = 0
runtime_fallback = 0
new_backend_route = 0
new_abi = 0
new_python_semantic_projector = 0
runner_semantic_owner = 0
```

## Implementation Outputs

When executed, this task should materialize:

```text
fixture:
  docs/development/current/main/design/fixtures/rust-lifecycle/
    source-selfhost-wider-route-selection-basis-007-v0.json

tool:
  tools/rust_lifecycle/
    source_selfhost_wider_route_selection_basis_007.py

guard:
  tools/checks/
    rust_lifecycle_source_selfhost_wider_route_selection_basis_007_guard.sh
```
