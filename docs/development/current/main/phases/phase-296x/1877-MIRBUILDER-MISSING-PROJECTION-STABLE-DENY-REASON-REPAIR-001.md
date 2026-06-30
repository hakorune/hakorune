# 1877 - MIRBUILDER-MISSING-PROJECTION-STABLE-DENY-REASON-REPAIR-001

## Token

```text
MIRBUILDER-MISSING-PROJECTION-STABLE-DENY-REASON-REPAIR-001
```

## Purpose

Repair the second blocking axis for crate-wide MissingProjectionPolicy
clusters. After owner-edge confidence repair, 1211 candidates are
`FixtureMapped`, but their deny reason is still the coarse
`PublicRustSurfaceMissingProjectionPolicy` diagnostic.

This card adds a stable, medium-grained deny reason axis while preserving the
original reason token.

## Output

```text
tool:
  tools/rust_lifecycle/
    mirbuilder_missing_projection_stable_deny_reason_repair.py

fixture:
  docs/development/current/main/design/fixtures/rust-lifecycle/
    mirbuilder-missing-projection-stable-deny-reason-repair-v0.json

guard:
  tools/checks/
    rust_lifecycle_mirbuilder_missing_projection_stable_deny_reason_repair_guard.sh
```

## Result

```text
stable_deny_reason.UnsupportedDirectShape = 1211
stable_deny_reason.OwnerEdgeConfidenceMissing = 185

decision = ApplyStableDenyReasonRepair
next_card = MIRBUILDER-CRATE-WIDE-SHAPE-SIGNATURE-INVENTORY-001
```

## Boundary

```text
original reason_token:
  preserved

stable_deny_reason:
  added as resolver axis

UnsupportedDirectShape:
  assigned only when owner_edge_confidence = FixtureMapped

OwnerEdgeConfidenceMissing:
  assigned to OtherMissingProjectionPolicyCluster surfaces
```

## Acceptance

```text
stable_deny_reason_repair_defined = 1
unsupported_direct_shape_count_after_repair = 1211
owner_edge_confidence_missing_count_after_repair = 185
manual_family_selection = 0
cluster_size_as_proof = 0
coverage_percentage_as_proof = 0
route_membership_alone_as_proof = 0
generated_artifact_as_edit_authority = 0
source_selfhost_claim = 0
runtime_fallback = 0
new_backend_route = 0
new_abi = 0
new_python_semantic_projector = 0
runner_semantic_owner = 0
family_name_based_policy = 0
```

## Non-Claims

```text
no Hako generation
no projection policy selection
no HakoAdopted decision
no native source seed materialization
no Source Selfhost claim
```
