# 1866 - MIRBUILDER-LOOP-COND-CO-AST-ASSIGNMENT-STATEMENT-PROJECTION-POLICY-001

## Token

```text
MIRBUILDER-LOOP-COND-CO-AST-ASSIGNMENT-STATEMENT-PROJECTION-POLICY-001
```

## Purpose

Resolve whether the `ASTNode::Assignment` arm inside `lower_stmt_ast` is a
standalone Hako projection surface.

## Decision

```text
selected_policy = DelegateToCarrierMergeAssignmentPolicy
projection_surface_selected = 0
decision = SelectDelegatedProjectionPolicy
```

The AST arm only extracts `target` / `value`, delegates to
`carrier_merge::lower_assignment_stmt`, and wraps returned effects with
`effects_to_plans`. The statement-shape arm is therefore not the semantic owner
for assignment lowering.

The delegated public helper is still unclassified in the crate-wide source
surface report:

```text
src/mir/builder/control_flow/plan/features/carrier_merge.rs::lower_assignment_stmt
classification = MissingProjectionPolicy
```

## Output

```text
fixture:
  docs/development/current/main/design/fixtures/rust-lifecycle/
    mirbuilder-loop-cond-co-ast-assignment-statement-projection-policy-v0.json

guard:
  tools/checks/
    rust_lifecycle_mirbuilder_loop_cond_co_ast_assignment_statement_projection_policy_guard.sh
```

## Acceptance

```text
input_statement_shape_classification_consumed = 1
selected_shape_id = AssignmentStatementShape
projection_surface_selected = 0
delegated_surface = carrier_merge::lower_assignment_stmt
delegated_surface_classification = MissingProjectionPolicy
selected_next_card = MIRBUILDER-CARRIER-MERGE-ASSIGNMENT-STATEMENT-PROJECTION-POLICY-001
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
1. MIRBUILDER-CARRIER-MERGE-ASSIGNMENT-STATEMENT-PROJECTION-POLICY-001
   Decide the policy for the delegated carrier-merge assignment helper.
```

## Non-Claims

```text
no Hako generation
no HakoAdopted decision
no native source seed
no Source Selfhost claim
no route repair
```
