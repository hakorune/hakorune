# 2003 - SOURCE-SELFHOST-NATIVE-OWNER-CHECKPOINT-001

## Token

```text
SOURCE-SELFHOST-NATIVE-OWNER-CHECKPOINT-001
```

## Purpose

Compute the current native Hako owner map and select the next blocker class
after the source-surface report became fresh.

This checkpoint is not a Source Selfhost claim. It only decides which
implementation lane should proceed next.

## Input Evidence

```text
fresh report:
  MIRBUILDER-UNCONVERTED-SURFACE-REPORT-RERUN-003

native owner adoption ledger:
  source-selfhost-family-guard-manifest-v0.json
```

## Result

```text
native_owner_count = 11
missing_projection_policy_count = 1384
missing_projection_evidence_quality_count = 1199
borrow_surface_policy_needed_count = 112
borrow_surface_evidence_quality_count = 0

decision:
  SelectMissingProjectionPolicyClusterResolutionV2

reason_token:
  MissingProjectionPolicyEvidenceQualityWinsCheckpoint

selected_next_card:
  MIRBUILDER-MISSING-PROJECTION-POLICY-CLUSTER-RESOLUTION-V2
```

The selection is not count-based. `MissingProjectionPolicy` wins because 1199
rows have fixture-mapped owner confidence, known shape signatures, and stable
deny reasons. `BorrowSurfaceNeedsPolicy` remains blocked because its current
rows have no owner-edge confidence.

## Output

```text
fixture:
  docs/development/current/main/design/fixtures/rust-lifecycle/
    source-selfhost-native-owner-checkpoint-v0.json

tool:
  tools/rust_lifecycle/
    source_selfhost_native_owner_checkpoint.py

guard:
  tools/checks/
    rust_lifecycle_source_selfhost_native_owner_checkpoint_guard.sh
```

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
new_canonical_mir_instruction = 0
new_python_semantic_projector = 0
runner_semantic_owner = 0
```
