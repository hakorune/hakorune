# 1857 - MIRBUILDER-LOOP-COND-CO-GROUP-IF-BRANCH-PROJECTION-POLICY-001

## Token

```text
MIRBUILDER-LOOP-COND-CO-GROUP-IF-BRANCH-PROJECTION-POLICY-001
```

## Purpose

Resolve whether `lower_group_if` should become a standalone Hako projection
surface.

## Decision

```text
selected_policy = PrivateGroupIfBranchHelper
owner_edge = loop_cond_continue_only
projection_surface_selected = 0
```

The source surface wires then/else recipe branches for the loop-cond
continue-only route. It snapshots and restores caller state, lowers both
branches with the existing continue-only block lowerer, rejects fallthrough
mutation that would require join generation, and delegates branch assembly to
the shared route entry.

It is not a standalone semantic owner edge.

## Output

```text
fixture:
  docs/development/current/main/design/fixtures/rust-lifecycle/
    mirbuilder-loop-cond-co-group-if-branch-projection-policy-v0.json

guard:
  tools/checks/
    rust_lifecycle_mirbuilder_loop_cond_co_group_if_branch_projection_policy_guard.sh
```

## Evidence

```text
visibility:
  pub(super)

callsite:
  src/mir/builder/control_flow/plan/features/loop_cond_co_stmt.rs

return_type:
  Result<Vec<LoweredRecipe>, String>

group-if branch markers:
  let pre_if_map = builder.variable_ctx.variable_map.clone()
  let pre_bindings = current_bindings.clone()
  lower_continue_only_block
  map_mutates_existing_vars
  group-if fallthrough mutates existing vars
  builder.variable_ctx.variable_map = pre_if_map
  *current_bindings = pre_bindings
  lower_if_join_with_branch_lowerers
```

## Acceptance

```text
policy = PrivateGroupIfBranchHelper
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
1. MIRBUILDER-LOOP-COND-CO-GROUP-IF-NESTED-LOOP-PROJECTION-POLICY-001
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
