# 1879 - MIRBUILDER-PROJECTION-POLICY-CLUSTER-PRIORITY-RESOLUTION-001

## Token

```text
MIRBUILDER-PROJECTION-POLICY-CLUSTER-PRIORITY-RESOLUTION-001
```

## Purpose

Resolve the 42 eligible projection-policy clusters exposed by the shape
signature inventory without manual family selection.

This card is a resolver only. It does not define the selected projection policy
or emit Hako.

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

## Result

```text
eligible_cluster_count = 42
selected_cluster =
  projection_policy::UnsupportedDirectShape::
  shape.loop_cond_bc_carrier_sync::FixtureMapped::
  LoopCondBcCarrierSyncCluster

selected_next_card =
  MIRBUILDER-LOOP-COND-BC-CARRIER-SYNC-PROJECTION-POLICY-001
```

## Priority Rule

```text
1. native_seed_or_adoption_proximity
2. control_flow_axis
3. borrow_axis
4. verifier_or_oracle_state
5. type_transport_axis
6. cluster_size as tiebreaker only
7. lexical cluster_id tiebreaker
```

## Acceptance

```text
deterministic_priority_resolution = 1
eligible_cluster_count = 42
cluster_size_as_proof = 0
cluster_size_tiebreaker_only = 1
manual_family_selection = 0
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
no projection policy definition
no HakoAdopted decision
no native source seed materialization
no Source Selfhost claim
```
