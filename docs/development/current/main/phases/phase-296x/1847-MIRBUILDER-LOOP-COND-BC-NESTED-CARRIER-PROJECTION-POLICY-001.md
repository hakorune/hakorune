# 1847 - MIRBUILDER-LOOP-COND-BC-NESTED-CARRIER-PROJECTION-POLICY-001

## Token

```text
MIRBUILDER-LOOP-COND-BC-NESTED-CARRIER-PROJECTION-POLICY-001
```

## Purpose

Resolve whether `extend_nested_loop_carriers` should become a standalone Hako
projection surface.

## Decision

```text
selected_policy = PrivateNestedCarrierHelper
owner_edge = loop_cond_break_continue
projection_surface_selected = 0
```

The source surface is a route-local nested-loop carrier propagation helper. It
only mutates a nested `CorePlan::Loop` by adding outer carrier PHIs and
`final_values` when the nested loop changed an outer carrier.

It is not a standalone semantic owner edge.

## Output

```text
fixture:
  docs/development/current/main/design/fixtures/rust-lifecycle/
    mirbuilder-loop-cond-bc-nested-carrier-projection-policy-v0.json

guard:
  tools/checks/
    rust_lifecycle_mirbuilder_loop_cond_bc_nested_carrier_projection_policy_guard.sh
```

## Evidence

```text
visibility:
  pub(super)

callsite:
  src/mir/builder/control_flow/plan/features/loop_cond_bc_item.rs

return_type:
  <none>

nested carrier markers:
  CorePlan::Loop(loop_plan)
  return; // no-op for non-Loop plans
  existing_names.contains
  build_loop_phi_info
  loop_plan.final_values.push
```

## Acceptance

```text
policy = PrivateNestedCarrierHelper
decision = KeepParentOwner
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

## Recommended Next Tasks

```text
1. MIRBUILDER-LOOP-COND-CONTINUE-ONLY-SURFACE-CLASSIFICATION-001
```

## Non-Claims

```text
no standalone Hako projection surface
no Hako generation
no HakoAdopted decision
no native seed materialization
no Source Selfhost claim
no route repair
```
