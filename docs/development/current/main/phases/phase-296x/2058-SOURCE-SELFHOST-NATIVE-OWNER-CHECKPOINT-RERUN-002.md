# 2058 - SOURCE-SELFHOST-NATIVE-OWNER-CHECKPOINT-RERUN-002

## Token

```text
SOURCE-SELFHOST-NATIVE-OWNER-CHECKPOINT-RERUN-002
```

## Purpose

Compute the current native Hako owner map and select the next blocker class
after `MIRBUILDER-UNCONVERTED-SURFACE-REPORT-RERUN-004` refreshed the report.

This checkpoint is not a Source Selfhost claim. It only decides which
implementation lane should proceed next.

## Local Authority

```text
local_selection_authority = LocalMechanicalSelectorAuthorityV1
worker_inventory = consumed
worker_inventory_scope = read_only_current_fixtures_cards_ledgers
```

## Input Authority

```text
fresh_report:
  docs/development/current/main/design/fixtures/rust-lifecycle/
    mirbuilder-crate-wide-unconverted-surface-report-v0.json

native_owner_manifest:
  docs/development/current/main/design/fixtures/rust-lifecycle/
    source-selfhost-family-guard-manifest-v0.json
```

## Result

```text
native_owner_count = 12
missing_projection_policy_count = 1004
missing_projection_evidence_quality_count = 819
borrow_surface_policy_needed_count = 112
borrow_surface_evidence_quality_count = 0
route_repair_candidate_count = 0

decision = SelectMissingProjectionPolicyClusterResolutionV4
reason_token = MissingProjectionPolicyEvidenceQualityWinsCheckpoint
selected_blocker_class = MissingProjectionPolicy
selected_next_card =
  MIRBUILDER-MISSING-PROJECTION-POLICY-CLUSTER-RESOLUTION-V4
```

The selection is not count-based. `MissingProjectionPolicy` wins because it has
fixture-mapped owner confidence, known shape signatures, and stable deny
reasons. `BorrowSurfaceNeedsPolicy` remains blocked by owner-confidence /
policy evidence quality.

## Non-Claims

```text
native_owner_checkpoint = 1
source_selfhost_claim = 0
rust_deletion = 0
manual_family_selection = 0
manual_shape_selection = 0
manual_axis_selection = 0
candidate_count_as_proof = 0
cluster_size_as_proof = 0
coverage_percentage_as_proof = 0
generated_artifact_as_native_edit_authority = 0
native_seed_materialization = 0
hako_generation = 0
hako_adopted_decision = 0
runtime_fallback = 0
new_backend_route = 0
new_abi = 0
new_python_semantic_projector = 0
runner_semantic_owner = 0
```

## Output

```text
fixture:
  docs/development/current/main/design/fixtures/rust-lifecycle/
    source-selfhost-native-owner-checkpoint-rerun-002-v0.json

tool:
  tools/rust_lifecycle/
    source_selfhost_native_owner_checkpoint_rerun_002.py

guard:
  tools/checks/
    rust_lifecycle_source_selfhost_native_owner_checkpoint_rerun_002_guard.sh
```
