# 1893 - MIRBUILDER-GENERIC-LOOP-BODY-FEATURE-PROJECTION-POLICY-001

## Token

```text
MIRBUILDER-GENERIC-LOOP-BODY-FEATURE-PROJECTION-POLICY-001
```

## Purpose

Resolve the selected GenericLoopBodyFeature projection-policy cluster.

The selected surfaces are generic loop body feature predicates / matchers:

```text
body_has_blockexpr_prelude_loop(body: &[ASTNode]) -> bool
stmt_has_blockexpr_prelude_loop(stmt: &ASTNode) -> bool
expr_has_blockexpr_prelude_loop(expr: &ASTNode) -> bool
can_attempt_conditional_update_branch(body: &[ASTNode]) -> bool
has_non_loop_assignment(body: &[ASTNode], loop_var: &str) -> bool
matches_loop_increment(stmt: &ASTNode, loop_var: &str, loop_increment: &ASTNode) -> bool
body_plans_exit_on_all_paths(plans: &[LoweredRecipe]) -> bool
plans_require_continue_edge(plans: &[LoweredRecipe]) -> bool
```

These functions classify already-owned generic loop body materialization
conditions. They do not own a standalone Hako projection surface. They remain
under the generic loop body feature parent owner.

## Output

```text
tool:
  tools/rust_lifecycle/
    mirbuilder_generic_loop_body_feature_projection_policy.py

fixture:
  docs/development/current/main/design/fixtures/rust-lifecycle/
    mirbuilder-generic-loop-body-feature-projection-policy-v0.json

guard:
  tools/checks/
    rust_lifecycle_mirbuilder_generic_loop_body_feature_projection_policy_guard.sh
```

## Decision

```text
policy = KeepParentOwner
owner_edge = mirbuilder::generic_loop_body_feature
projection_surface_selected = 0

next_card =
  MIRBUILDER-PROJECTION-POLICY-CLUSTER-PRIORITY-RESOLUTION-001
```

## Evidence

```text
source_count = 8

roles:
  blockexpr_prelude_loop_detector = 3
  conditional_update_branch_gate = 1
  non_loop_assignment_detector = 1
  loop_increment_matcher = 1
  body_terminality_detector = 1
  continue_edge_detector = 1

markers:
  try_lower_blockexpr_loop_prelude_value
  body_has_blockexpr_prelude_loop
  expr_has_blockexpr_prelude_loop
  can_attempt_conditional_update_branch
  is_pure_value_expr_for_generic_loop
  matches_loop_increment
  Route-local terminality / continue-edge classification
  plan_requires_continue_edge
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
