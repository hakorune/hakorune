# 1890 - MIRBUILDER-PLAN-COMPOSER-PROJECTION-POLICY-001

## Token

```text
MIRBUILDER-PLAN-COMPOSER-PROJECTION-POLICY-001
```

## Purpose

Resolve the selected PlanComposer projection-policy cluster.

The selected surfaces are CoreLoop composer gate predicates:

```text
coreloop_base_gate(facts: &CanonicalLoopFacts) -> bool
exit_kinds_empty(facts: &CanonicalLoopFacts) -> bool
```

These helpers inspect already-extracted `CanonicalLoopFacts` and gate the
parent CorePlan composer. They do not independently define a Hako projection
surface.

## Output

```text
tool:
  tools/rust_lifecycle/
    mirbuilder_plan_composer_projection_policy.py

fixture:
  docs/development/current/main/design/fixtures/rust-lifecycle/
    mirbuilder-plan-composer-projection-policy-v0.json

guard:
  tools/checks/
    rust_lifecycle_mirbuilder_plan_composer_projection_policy_guard.sh
```

## Decision

```text
policy = KeepParentOwner
owner_edge = mirbuilder::plan_composer
projection_surface_selected = 0

next_card =
  MIRBUILDER-PROJECTION-POLICY-CLUSTER-PRIORITY-RESOLUTION-001
```

## Evidence

```text
source_count = 2
symbols:
  coreloop_base_gate
  exit_kinds_empty

markers:
  CanonicalLoopFacts
  SkeletonKind::Loop
  cleanup_kinds_present.is_empty()
  exit_kinds_present.is_empty()
  coreloop_base_gate
  exit_kinds_empty
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
