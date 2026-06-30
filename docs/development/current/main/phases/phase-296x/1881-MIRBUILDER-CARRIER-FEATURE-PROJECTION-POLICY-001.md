# 1881 - MIRBUILDER-CARRIER-FEATURE-PROJECTION-POLICY-001

## Token

```text
MIRBUILDER-CARRIER-FEATURE-PROJECTION-POLICY-001
```

## Purpose

Resolve the selected CarrierFeature projection-policy cluster.

The selected surfaces are generic-loop carrier finalization helpers. They update
`CoreLoopPlan` PHIs/final values and publish the emission cache. They are not a
standalone Hako projection surface.

## Output

```text
tool:
  tools/rust_lifecycle/
    mirbuilder_carrier_feature_projection_policy.py

fixture:
  docs/development/current/main/design/fixtures/rust-lifecycle/
    mirbuilder-carrier-feature-projection-policy-v0.json

guard:
  tools/checks/
    rust_lifecycle_mirbuilder_carrier_feature_projection_policy_guard.sh
```

## Decision

```text
policy = KeepParentOwner
owner_edge = generic_loop_v1_carrier_finalization
projection_surface_selected = 0

next_card =
  MIRBUILDER-PROJECTION-POLICY-CLUSTER-PRIORITY-RESOLUTION-001
```

## Evidence

```text
source_count = 2
symbols:
  finalize
  finalize_generic_loop_v1_carriers

markers:
  finalize_generic_loop_v1_carriers(
  loop_plan.phis
  loop_plan.final_values
  publish_emission_cache
  body_has_continue_edge
```

## Acceptance

```text
policy = KeepParentOwner
projection_surface_selected = 0
manual_family_selection = 0
hako_generation = 0
hako_adopted_decision = 0
native_seed_materialization = 0
source_selfhost_claim = 0
runtime_fallback = 0
new_backend_route = 0
new_abi = 0
new_python_semantic_projector = 0
runner_semantic_owner = 0
```

## Non-Claims

```text
no standalone Hako projection surface
no Hako generation
no HakoAdopted decision
no native seed materialization
no Source Selfhost claim
```
