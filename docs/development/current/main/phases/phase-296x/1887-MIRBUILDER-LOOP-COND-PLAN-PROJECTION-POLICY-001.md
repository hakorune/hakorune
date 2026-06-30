# 1887 - MIRBUILDER-LOOP-COND-PLAN-PROJECTION-POLICY-001

## Token

```text
MIRBUILDER-LOOP-COND-PLAN-PROJECTION-POLICY-001
```

## Purpose

Resolve the selected LoopCondPlan projection-policy cluster.

The selected cluster contains 24 `bool` predicate helpers used by
loop-condition facts extraction and pattern validation. These predicates
recognize supported `if`/exit/prelude shapes for loop-condition planning; they
do not independently define a Hako projection surface.

## Output

```text
tool:
  tools/rust_lifecycle/
    mirbuilder_loop_cond_plan_projection_policy.py

fixture:
  docs/development/current/main/design/fixtures/rust-lifecycle/
    mirbuilder-loop-cond-plan-projection-policy-v0.json

guard:
  tools/checks/
    rust_lifecycle_mirbuilder_loop_cond_plan_projection_policy_guard.sh
```

## Decision

```text
policy = KeepParentOwner
owner_edge = mirbuilder::loop_cond_plan
projection_surface_selected = 0

next_card =
  MIRBUILDER-PROJECTION-POLICY-CLUSTER-PRIORITY-RESOLUTION-001
```

## Evidence

```text
source_count = 24

roles:
  branch_exit_helper = 5
  conditional_update_validator = 1
  else_shape_validator = 5
  exit_shape_validator = 6
  general_if_classifier = 1
  prelude_validator = 6

markers:
  loop_cond_break_continue facts extraction
  pattern validators
  is_supported_bool_expr_with_canon
  branch_has_exit_or_loop
  exit_prelude_is_allowed
  return_prelude_is_allowed
  is_exit_if_stmt
  allow_extended
  allow_return
  ASTNode::If
  ASTNode::Break
  ASTNode::Return
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
