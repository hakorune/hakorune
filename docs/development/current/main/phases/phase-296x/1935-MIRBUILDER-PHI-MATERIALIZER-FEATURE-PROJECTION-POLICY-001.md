# 1935 - MIRBUILDER-PHI-MATERIALIZER-FEATURE-PROJECTION-POLICY-001

## Token

```text
MIRBUILDER-PHI-MATERIALIZER-FEATURE-PROJECTION-POLICY-001
```

## Purpose

Materialize the projection-policy descriptor for the selected
`shape.phi_materializer_feature` cluster.

This card records CorePhiInfo builders, loop binding construction, and
route-local PHI closure constructors as a bounded descriptor. It does not
generate Hako, does not select a native seed, and does not claim Source
Selfhost.

## Output

```text
fixture:
  docs/development/current/main/design/fixtures/rust-lifecycle/
    mirbuilder-phi-materializer-feature-projection-policy-v0.json

tool:
  tools/rust_lifecycle/
    mirbuilder_phi_materializer_feature_projection_policy.py

guard:
  tools/checks/
    rust_lifecycle_mirbuilder_phi_materializer_feature_projection_policy_guard.sh
```

## Acceptance

```text
priority_resolution_consumed = 1
unconverted_surface_report_consumed = 1
source_count = 7
descriptor_selected = 1
hako_projection_selected = 0
mutation_frame = []
manual_family_selection = 0
hako_generation = 0
hako_adopted_decision = 0
native_seed_materialization = 0
source_selfhost_claim = 0
runtime_fallback = 0
new_backend_route = 0
new_abi = 0
new_python_semantic_projector = 0
```

## Recommended Next Tasks

```text
1. MIRBUILDER-PROJECTION-POLICY-CLUSTER-PRIORITY-RESOLUTION-001
   Rerun the cluster-priority resolver and select the next unclosed
   projection-policy cluster.
```

## Non-Claims

```text
no Hako projection
no native source seed
no HakoAdopted decision
no Source Selfhost claim
```
