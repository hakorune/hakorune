# 1895 - MIRBUILDER-PLAN-PARTS-ASSEMBLY-PROJECTION-POLICY-001

## Token

```text
MIRBUILDER-PLAN-PARTS-ASSEMBLY-PROJECTION-POLICY-001
```

## Purpose

Resolve the selected PlanPartsAssembly projection-policy cluster.

The selected surfaces are conditional-update, dispatch, statement-shape, and
RecipeBlock verification predicates:

```text
has_any_assignment(body) -> bool
is_conditional_update_branch_supported(body) -> bool
is_pure_value_expr(ast) -> bool
plans_exit_on_all_paths(plans) -> bool
stmt_has_loop_stmt_recursive(stmt) -> bool
tail_is_exit(body) -> bool
value_has_blockexpr_prelude_loop(value) -> bool
is_block_exit_only_item(item) -> bool
is_exit_only_block(block) -> bool
```

These helpers classify shape and exit-path properties for existing plan parts
assembly. They do not own a standalone Hako projection surface. They remain
under the PlanPartsAssembly parent owner.

## Output

```text
tool:
  tools/rust_lifecycle/
    mirbuilder_plan_parts_assembly_projection_policy.py

fixture:
  docs/development/current/main/design/fixtures/rust-lifecycle/
    mirbuilder-plan-parts-assembly-projection-policy-v0.json

guard:
  tools/checks/
    rust_lifecycle_mirbuilder_plan_parts_assembly_projection_policy_guard.sh
```

## Decision

```text
policy = KeepParentOwner
owner_edge = mirbuilder::plan_parts_assembly
projection_surface_selected = 0

next_card =
  MIRBUILDER-PROJECTION-POLICY-CLUSTER-PRIORITY-RESOLUTION-001
```

## Evidence

```text
source_count = 10

roles:
  conditional_update_branch_predicate = 2
  conditional_update_pure_expr_predicate = 1
  plan_exit_path_predicate = 2
  statement_shape_predicate = 3
  recipe_block_verify_shape_predicate = 2

markers:
  is_conditional_update_branch_supported
  has_any_assignment
  is_pure_value_expr
  Exit-path predicates for RecipeBlock dispatch
  plans_exit_on_all_paths
  Statement shape predicates for return-prelude lowering
  Shape predicates for RecipeBlock verification
  is_exit_only_block
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
