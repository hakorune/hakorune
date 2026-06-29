# 1863 - MIRBUILDER-LOOP-COND-CO-STATEMENT-DISPATCHER-PROJECTION-POLICY-001

## Token

```text
MIRBUILDER-LOOP-COND-CO-STATEMENT-DISPATCHER-PROJECTION-POLICY-001
```

## Purpose

Resolve whether `lower_continue_only_stmt` should become a standalone Hako
projection surface.

## Decision

```text
selected_policy = PrivateStatementDispatcherHelper
owner_edge = loop_cond_continue_only
projection_surface_selected = 0
```

The source surface dispatches `ContinueOnlyStmtRecipe` variants to existing
route-local lowerers. It owns no independent semantic projection; it preserves
the source-order dispatch boundary for the parent `loop_cond_continue_only`
owner.

It is not a standalone semantic owner edge.

## Output

```text
fixture:
  docs/development/current/main/design/fixtures/rust-lifecycle/
    mirbuilder-loop-cond-co-statement-dispatcher-projection-policy-v0.json

guard:
  tools/checks/
    rust_lifecycle_mirbuilder_loop_cond_co_statement_dispatcher_projection_policy_guard.sh
```

## Evidence

```text
visibility:
  pub(super)

callsite:
  src/mir/builder/control_flow/plan/features/loop_cond_co_block.rs

return_type:
  Result<Vec<LoweredRecipe>, String>

statement-dispatcher markers:
  match stmt
  ContinueOnlyStmtRecipe::Stmt(node)
  ContinueOnlyStmtRecipe::ContinueIf
  ContinueOnlyStmtRecipe::ContinueIfGroupPrelude
  ContinueOnlyStmtRecipe::GroupIf
  ContinueOnlyStmtRecipe::ContinueIfNestedLoop
  lower_stmt_ast
  lower_continue_if_no_else
  lower_continue_if_group_prelude
  lower_group_if
  lower_continue_if_nested_loop
```

## Acceptance

```text
policy = PrivateStatementDispatcherHelper
decision = KeepParentOwner
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

## Recommended Next Tasks

```text
1. MIRBUILDER-LOOP-COND-CO-AST-STATEMENT-LOWERING-PROJECTION-POLICY-001
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
