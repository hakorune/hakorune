# 1948 - MIRBUILDER-CRATE-WIDE-NATIVE-OWNER-SEED-CAPABILITY-SURVEY-RERUN-001

## Token

```text
MIRBUILDER-CRATE-WIDE-NATIVE-OWNER-SEED-CAPABILITY-SURVEY-RERUN-001
```

## Purpose

Rerun the native-owner seed capability resolver after both projection-policy
queues are exhausted:

```text
global projection-policy priority queue:
  selectable_cluster_count = 0

Other shape-signature queue:
  selection_eligible_shape_count = 0
```

This card does not select a family by hand. It verifies queue exhaustion,
checks whether the source-surface report is fresh against the landed descriptor
ledger, and returns the next machine-derived resolver.

## Output

```text
fixture:
  docs/development/current/main/design/fixtures/rust-lifecycle/
    mirbuilder-crate-wide-native-owner-seed-capability-survey-rerun-v0.json

tool:
  tools/rust_lifecycle/
    mirbuilder_crate_wide_native_owner_seed_capability_survey_rerun.py

guard:
  tools/checks/
    rust_lifecycle_mirbuilder_crate_wide_native_owner_seed_capability_survey_rerun_guard.sh
```

## Acceptance

```text
global_projection_policy_exhaustion_consumed = 1
global_selectable_cluster_count = 0
global_reason_token = NoEligibleProjectionPolicyCluster
other_shape_queue_exhaustion_consumed = 1
other_selection_eligible_shape_count = 0
other_reason_token = NoUnclosedOtherShapeSignatureClusterEligible
projection_descriptor_ledger_hash_recorded = 1
unconverted_surface_report_hash_recorded = 1
freshness_checked = 1
needs_unconverted_surface_report_rerun = 1
manual_family_selection = 0
manual_shape_selection = 0
manual_axis_selection = 0
cluster_size_as_proof = 0
coverage_percentage_as_proof = 0
route_membership_alone_as_proof = 0
generated_artifact_as_native_edit_authority = 0
source_selfhost_claim = 0
runtime_fallback = 0
new_backend_route = 0
new_abi = 0
new_python_semantic_projector = 0
runner_semantic_owner = 0
family_name_based_policy = 0
```

## Result

```text
decision:
  SelectUnconvertedSurfaceReportRerun

reason_token:
  UnconvertedSurfaceReportStaleAfterProjectionDescriptorCloseout

selected_next_card:
  MIRBUILDER-UNCONVERTED-SURFACE-REPORT-RERUN-001
```

The unconverted surface report is stale relative to the current family guard
manifest / descriptor ledger, so native owner seed selection remains deferred
until the report is regenerated.

## Recommended Next Task

```text
MIRBUILDER-UNCONVERTED-SURFACE-REPORT-RERUN-001
```

Regenerate the crate-wide source-surface report against the landed descriptor
ledger before selecting native owner seeds, route repairs, or Other blocker
axes.

## Non-Claims

```text
no native source seed
no HakoAdopted decision
no Source Selfhost claim
no runtime fallback
```
