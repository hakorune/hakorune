# 1864 - MIRBUILDER-LOOP-COND-CO-AST-STATEMENT-LOWERING-PROJECTION-POLICY-001

## Token

```text
MIRBUILDER-LOOP-COND-CO-AST-STATEMENT-LOWERING-PROJECTION-POLICY-001
```

## Purpose

Resolve whether `lower_stmt_ast` should become a standalone Hako projection
surface.

## Decision

```text
selected_policy = CompositeAstStatementLoweringNeedsSubclassification
owner_edge = loop_cond_continue_only
projection_surface_selected = 0
decision = SelectFurtherSubclassification
```

The source surface handles multiple AST statement shapes and delegates to
several existing lowering helpers. Treating the whole function as one
projection owner would hide the statement-shape boundaries.

It is not selected as a standalone semantic owner edge. It must be subdivided
before projection policy is assigned to any child surface.

## Output

```text
fixture:
  docs/development/current/main/design/fixtures/rust-lifecycle/
    mirbuilder-loop-cond-co-ast-statement-lowering-projection-policy-v0.json

guard:
  tools/checks/
    rust_lifecycle_mirbuilder_loop_cond_co_ast_statement_lowering_projection_policy_guard.sh
```

## Evidence

```text
visibility:
  pub(super)

return_type:
  Result<Vec<LoweredRecipe>, String>

AST statement shapes:
  Assignment
  Local
  MethodCall
  FunctionCall
  Print
  If
  Continue
  Break
  Return
  Loop
  LoopRange

delegates:
  lower_assignment_stmt
  lower_local_init_stmt
  loop_body_lowering::{lower_method_call_stmt, lower_function_call_stmt}
  lower_conditional_update_if
  try_lower_general_if_recipe_authority
  lower_if_exit_stmt
  lower_nested_loop_depth1_any
  sync_carrier_bindings
```

## Acceptance

```text
policy = CompositeAstStatementLoweringNeedsSubclassification
decision = SelectFurtherSubclassification
projection_surface_selected = 0
composite_owner_as_semantic_owner = 0
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
1. MIRBUILDER-LOOP-COND-CO-AST-STATEMENT-LOWERING-SURFACE-CLASSIFICATION-001
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
