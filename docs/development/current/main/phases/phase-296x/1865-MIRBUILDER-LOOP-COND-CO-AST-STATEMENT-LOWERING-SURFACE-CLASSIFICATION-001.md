# 1865 - MIRBUILDER-LOOP-COND-CO-AST-STATEMENT-LOWERING-SURFACE-CLASSIFICATION-001

## Token

```text
MIRBUILDER-LOOP-COND-CO-AST-STATEMENT-LOWERING-SURFACE-CLASSIFICATION-001
```

## Purpose

Classify the statement-shape surfaces inside `lower_stmt_ast` after 1864
rejected the whole function as a standalone projection owner.

This remains diagnostic-only. It does not generate Hako, select a native source
seed, run an adoption decision, or claim Source Selfhost.

## Result

```text
source_surface:
  lower_stmt_ast

shape_inventory:
  AssignmentStatementShape
  LocalStatementShape
  MethodCallStatementShape
  FunctionCallStatementShape
  PrintStatementShape
  IfStatementShape
  ExitRejectStatementShape
  NestedLoopStatementShape
  LoopRangeRejectStatementShape
  UnsupportedStatementShape

decision:
  SelectShapeProjectionPolicy

selected_next_card:
  MIRBUILDER-LOOP-COND-CO-AST-ASSIGNMENT-STATEMENT-PROJECTION-POLICY-001
```

The assignment shape is selected first because it is a leaf statement shape and
delegates to the existing carrier-merge assignment helper. This card does not
decide that projection policy; it only fixes the next child surface.

## Output

```text
fixture:
  docs/development/current/main/design/fixtures/rust-lifecycle/
    mirbuilder-loop-cond-co-ast-statement-lowering-surface-classification-v0.json

guard:
  tools/checks/
    rust_lifecycle_mirbuilder_loop_cond_co_ast_statement_lowering_surface_classification_guard.sh
```

## Acceptance

```text
input_projection_policy_consumed = 1
source_surface = lower_stmt_ast
shape_inventory_complete_for_current_match_arms = 1
shape_count = 10
selected_next_card = MIRBUILDER-LOOP-COND-CO-AST-ASSIGNMENT-STATEMENT-PROJECTION-POLICY-001
manual_family_selection = 0
projection_policy_selected = 0
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
1. MIRBUILDER-LOOP-COND-CO-AST-ASSIGNMENT-STATEMENT-PROJECTION-POLICY-001
   Decide whether the assignment statement shape is a private helper route or
   a child projection surface.
```

## Non-Claims

```text
no Hako generation
no HakoAdopted decision
no native source seed
no Source Selfhost claim
no route repair
no composed closure adoption
```
