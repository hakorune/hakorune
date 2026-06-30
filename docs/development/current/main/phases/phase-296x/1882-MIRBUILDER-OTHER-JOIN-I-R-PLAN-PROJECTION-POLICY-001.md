# 1882 - MIRBUILDER-OTHER-JOIN-I-R-PLAN-PROJECTION-POLICY-001

## Token

```text
MIRBUILDER-OTHER-JOIN-I-R-PLAN-PROJECTION-POLICY-001
```

## Purpose

Resolve the selected OtherJoinIRPlan projection-policy cluster.

The selected surfaces are JoinIR planner helpers: rule/session dispatch,
skeleton/wiring constructors, trace helpers, branch-to-if rewriting, and
if-join payload construction. They are parent-owned plan internals, not a
standalone Hako projection surface.

## Output

```text
tool:
  tools/rust_lifecycle/
    mirbuilder_other_join_i_r_plan_projection_policy.py

fixture:
  docs/development/current/main/design/fixtures/rust-lifecycle/
    mirbuilder-other-join-i-r-plan-projection-policy-v0.json

guard:
  tools/checks/
    rust_lifecycle_mirbuilder_other_join_i_r_plan_projection_policy_guard.sh
```

## Decision

```text
policy = KeepParentOwner
owner_edge = mirbuilder::join_i_r_plan
projection_surface_selected = 0

next_card =
  MIRBUILDER-PROJECTION-POLICY-CLUSTER-PRIORITY-RESOLUTION-001
```

## Evidence

```text
source_count = 19
helper buckets:
  join_payload
  planner_gate_or_fact_count
  planner_session_or_rule_dispatch
  route_rewrite_helper
  skeleton_or_wiring
  trace_or_debug

markers:
  CorePlan::If
  PlanBuildSession
  PLAN_RULE_ORDER
  GenericLoopSkeleton
  LoopTrueSkeleton
  LoopStepMode
  CoreIfJoin
  FragEmitSession
  [plan/trace]
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
