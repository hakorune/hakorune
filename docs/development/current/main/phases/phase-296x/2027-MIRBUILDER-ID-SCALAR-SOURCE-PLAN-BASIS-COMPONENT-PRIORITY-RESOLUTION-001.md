# 2027 - MIRBUILDER-ID-SCALAR-SOURCE-PLAN-BASIS-COMPONENT-PRIORITY-RESOLUTION-001

## Token

```text
MIRBUILDER-ID-SCALAR-SOURCE-PLAN-BASIS-COMPONENT-PRIORITY-RESOLUTION-001
```

## Purpose

Select the next unresolved ID scalar `SourcePlanAndRecipe` basis component by a
dependency graph, not by manual axis choice.

## Result

```text
decision:
  SelectBasisComponent

selected_component_id:
  OwnerScopeBoundedness

reason_token:
  OwnerScopeBoundednessSelectedAsSourcePlanRootComponent

selected_next_card:
  MIRBUILDER-ID-SCALAR-OWNER-SCOPE-BOUNDEDNESS-RESOLUTION-001
```

## Dependency Boundary

`OwnerScopeBoundedness` is the root component because it defines the owner
subject for later mutation-frame, behavior-recipe, verifier-input, and native
seed file boundary work.

```text
manual_component_selection = 0
cluster_size_as_proof = 0
surface_count_as_proof = 0
route_membership_alone_as_proof = 0
```

## Output

```text
fixture:
  docs/development/current/main/design/fixtures/rust-lifecycle/
    mirbuilder-id-scalar-source-plan-basis-component-priority-resolution-v0.json

tool:
  tools/rust_lifecycle/
    mirbuilder_id_scalar_source_plan_basis_component_priority_resolution.py

guard:
  tools/checks/
    rust_lifecycle_mirbuilder_id_scalar_source_plan_basis_component_priority_resolution_guard.sh
```

## Non-Claims

```text
source_plan_materialization = 0
behavior_recipe_materialization = 0
verifier_result_materialization = 0
derived_artifact_seed_draft_materialization = 0
native_seed_materialization = 0
hako_generation = 0
hako_adopted_decision = 0
source_selfhost_claim = 0
runtime_fallback = 0
new_backend_route = 0
new_abi = 0
new_python_semantic_projector = 0
```
