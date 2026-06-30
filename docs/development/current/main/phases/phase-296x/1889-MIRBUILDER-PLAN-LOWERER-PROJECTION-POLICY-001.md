# 1889 - MIRBUILDER-PLAN-LOWERER-PROJECTION-POLICY-001

## Token

```text
MIRBUILDER-PLAN-LOWERER-PROJECTION-POLICY-001
```

## Purpose

Resolve the selected PlanLowerer projection-policy cluster.

The selected surface is the span-formatting helper:

```text
current_span_location(builder: &MirBuilder) -> String
```

This helper reads the current metadata span and formats it for diagnostics. It
is owned by the PlanLowerer diagnostics surface and does not open a standalone
Hako projection surface.

## Output

```text
tool:
  tools/rust_lifecycle/
    mirbuilder_plan_lowerer_projection_policy.py

fixture:
  docs/development/current/main/design/fixtures/rust-lifecycle/
    mirbuilder-plan-lowerer-projection-policy-v0.json

guard:
  tools/checks/
    rust_lifecycle_mirbuilder_plan_lowerer_projection_policy_guard.sh
```

## Decision

```text
policy = KeepParentOwner
owner_edge = mirbuilder::plan_lowerer
projection_surface_selected = 0

next_card =
  MIRBUILDER-PROJECTION-POLICY-CLUSTER-PRIORITY-RESOLUTION-001
```

## Evidence

```text
source_count = 1
symbol = current_span_location

markers:
  current_span_location
  metadata_ctx.current_span().location_string()
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
