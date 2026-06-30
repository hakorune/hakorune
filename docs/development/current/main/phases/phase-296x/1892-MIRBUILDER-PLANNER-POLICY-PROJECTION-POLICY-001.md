# 1892 - MIRBUILDER-PLANNER-POLICY-PROJECTION-POLICY-001

## Token

```text
MIRBUILDER-PLANNER-POLICY-PROJECTION-POLICY-001
```

## Purpose

Resolve the selected PlannerPolicy projection-policy cluster.

The selected surfaces are stable planner log tag helpers:

```text
planner_first_tag(rule_id: PlanRuleId) -> String
planner_first_tag_with_label(rule_id: PlanRuleId) -> String
```

These helpers format stable diagnostic / TSV-facing tags for the planner
policy owner. They do not select a new Hako projection surface. They remain
under the PlannerPolicy parent owner.

## Output

```text
tool:
  tools/rust_lifecycle/
    mirbuilder_planner_policy_projection_policy.py

fixture:
  docs/development/current/main/design/fixtures/rust-lifecycle/
    mirbuilder-planner-policy-projection-policy-v0.json

guard:
  tools/checks/
    rust_lifecycle_mirbuilder_planner_policy_projection_policy_guard.sh
```

## Decision

```text
policy = KeepParentOwner
owner_edge = mirbuilder::planner_policy
projection_surface_selected = 0

next_card =
  MIRBUILDER-PROJECTION-POLICY-CLUSTER-PRIORITY-RESOLUTION-001
```

## Evidence

```text
source_count = 2

roles:
  planner_first_stable_tag_builder = 1
  planner_first_stable_tag_with_label_builder = 1

markers:
  Planner tag SSOT
  Keep `[joinir/planner_first rule=...]` tags stable across refactors
  Prevent TSV expectations from drifting due to incidental formatting changes
  planner_rule_tag_name
  planner_rule_semantic_label
  planner_first_tag
  planner_first_tag_with_label
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
