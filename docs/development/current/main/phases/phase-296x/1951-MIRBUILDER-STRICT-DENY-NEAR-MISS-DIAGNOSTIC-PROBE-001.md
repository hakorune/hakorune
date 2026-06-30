# 1951 - MIRBUILDER-STRICT-DENY-NEAR-MISS-DIAGNOSTIC-PROBE-001

## Token

```text
MIRBUILDER-STRICT-DENY-NEAR-MISS-DIAGNOSTIC-PROBE-001
```

## Purpose

Add a diagnostic-only relaxed classifier over the crate-wide unconverted
surface report.

This card does not weaken strict conversion. It keeps strict deny
classification authoritative and adds a near-miss view that answers which
surfaces would become actionable after adding projection policy, borrow policy,
owner-edge confidence, or composite evidence.

## Output

```text
fixture:
  docs/development/current/main/design/fixtures/rust-lifecycle/
    mirbuilder-strict-deny-near-miss-diagnostic-probe-v0.json

tool:
  tools/rust_lifecycle/
    mirbuilder_strict_deny_near_miss_diagnostic_probe.py

guard:
  tools/checks/
    rust_lifecycle_mirbuilder_strict_deny_near_miss_diagnostic_probe_guard.sh
```

## Acceptance

```text
strict_classification_remains_authority = 1
diagnostic_relaxed_mode_only = 1
needs_projection_policy_only_count > 0
selection_eligible_cluster_count > 0
selected_next_card =
  MIRBUILDER-STRICT-DENY-NEAR-MISS-CLUSTER-RESOLUTION-001
strict_rules_changed = 0
hako_generation = 0
hako_adopted_decision = 0
native_seed_materialization = 0
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
```

## Result

```text
decision:
  SelectNearMissClusterResolution

reason_token:
  ProjectionPolicyNearMissClustersAvailable

selected_next_card:
  MIRBUILDER-STRICT-DENY-NEAR-MISS-CLUSTER-RESOLUTION-001
```

The probe classifies strict denies without changing strict conversion. Current
diagnostics show many strict-denied surfaces are near misses because a
projection policy descriptor is the remaining missing evidence.

## Recommended Next Task

```text
MIRBUILDER-STRICT-DENY-NEAR-MISS-CLUSTER-RESOLUTION-001
```

Select exactly one near-miss cluster by evidence quality. Do not select by
cluster size alone.

## Non-Claims

```text
no relaxed executable conversion
no Hako generation
no native source seed
no HakoAdopted decision
no Source Selfhost claim
```
