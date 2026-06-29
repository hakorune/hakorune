# 1858 - MIRBUILDER-LOOP-COND-CO-GROUP-IF-NESTED-LOOP-PROJECTION-POLICY-001

## Token

```text
MIRBUILDER-LOOP-COND-CO-GROUP-IF-NESTED-LOOP-PROJECTION-POLICY-001
```

## Purpose

Resolve whether `lower_continue_if_nested_loop` should become a standalone
Hako projection surface.

## Decision

```text
selected_policy = PrivateGroupIfNestedLoopHelper
owner_edge = loop_cond_continue_only
projection_surface_selected = 0
```

The source surface wires the nested-loop form of the loop-cond continue-only
route. It lowers prelude and postlude spans with the existing continue-only
block lowerer, delegates the inner loop to `lower_nested_loop_depth1_any`,
appends the continue exit, restores the caller state, and lowers the outer
conditional branch through the shared route entry.

It is not a standalone semantic owner edge.

## Output

```text
fixture:
  docs/development/current/main/design/fixtures/rust-lifecycle/
    mirbuilder-loop-cond-co-group-if-nested-loop-projection-policy-v0.json

guard:
  tools/checks/
    rust_lifecycle_mirbuilder_loop_cond_co_group_if_nested_loop_projection_policy_guard.sh
```

## Evidence

```text
visibility:
  pub(super)

callsite:
  src/mir/builder/control_flow/plan/features/loop_cond_co_stmt.rs

return_type:
  Result<Vec<LoweredRecipe>, String>

nested-loop markers:
  get_body_span
  lower_continue_only_block
  lower_nested_loop_depth1_any
  LoopRange in nested loop not supported
  expected Loop/While in ContinueIfNestedLoop
  build_continue_with_phi_args
  then_plans.push(CorePlan::Exit(exit))
  builder.variable_ctx.variable_map = saved_map
  *current_bindings = saved_bindings
  lower_if_join_with_branch_lowerers
```

## Acceptance

```text
policy = PrivateGroupIfNestedLoopHelper
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
1. MIRBUILDER-LOOP-COND-CO-HELPER-SURFACE-CLASSIFICATION-001
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
