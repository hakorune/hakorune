# 1906 - MIRBUILDER-PROJECTION-POLICY-CLUSTER-PRIORITY-DECOMPOSED-CLUSTER-FILTER-001

## Token

```text
MIRBUILDER-PROJECTION-POLICY-CLUSTER-PRIORITY-DECOMPOSED-CLUSTER-FILTER-001
```

## Purpose

Repair the projection-policy cluster priority resolver after the CallLowering
cluster was decomposed into narrower subcluster cards.

The resolver previously filtered only by the generated next-card token. That
missed clusters resolved by a decomposition fixture whose card token differs
from the original cluster-level `MIRBUILDER-...-PROJECTION-POLICY-001` slug.

This card makes the filter generic: any manifest fixture with
`input_state.source_cluster_id` excludes that source cluster from the selectable
priority pool.

## Output

```text
tool:
  tools/rust_lifecycle/
    mirbuilder_projection_policy_cluster_priority_resolution.py

fixture:
  docs/development/current/main/design/fixtures/rust-lifecycle/
    mirbuilder-projection-policy-cluster-priority-resolution-v0.json

guard:
  tools/checks/
    rust_lifecycle_mirbuilder_projection_policy_cluster_priority_resolution_guard.sh
```

## Decision

```text
source_cluster_decomposition_filter_enabled = 1
excluded_existing_decision_cluster_count = 32
selectable_cluster_count = 10

next_card =
  MIRBUILDER-GENERIC-LOOP-PLAN-PROJECTION-POLICY-001
```

## Acceptance

```text
deterministic_priority_resolution = 1
source_cluster_decomposition_filter_enabled = 1
manual_family_selection = 0
cluster_size_as_proof = 0
source_selfhost_claim = 0
runtime_fallback = 0
new_backend_route = 0
new_abi = 0
new_python_semantic_projector = 0
runner_semantic_owner = 0
```

## Non-Claims

```text
no Source Selfhost claim
no Hako generation
no HakoAdopted decision
no native seed materialization
```
