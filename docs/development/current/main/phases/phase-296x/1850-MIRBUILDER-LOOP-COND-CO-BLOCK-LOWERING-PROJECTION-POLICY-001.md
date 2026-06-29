# 1850 - MIRBUILDER-LOOP-COND-CO-BLOCK-LOWERING-PROJECTION-POLICY-001

## Token

```text
MIRBUILDER-LOOP-COND-CO-BLOCK-LOWERING-PROJECTION-POLICY-001
```

## Purpose

Resolve whether `lower_continue_only_block` should become a standalone Hako
projection surface.

## Decision

```text
selected_policy = PrivateBlockLoweringHelper
owner_edge = loop_cond_continue_only
projection_surface_selected = 0
```

The source surface iterates `ContinueOnlyStmtRecipe` items and delegates each
item to `lower_continue_only_stmt`. It is a route-local block dispatcher used
by the continue-only root pipeline, group-if, and continue-if helpers.

It is not a standalone semantic owner edge.

## Output

```text
fixture:
  docs/development/current/main/design/fixtures/rust-lifecycle/
    mirbuilder-loop-cond-co-block-lowering-projection-policy-v0.json

guard:
  tools/checks/
    rust_lifecycle_mirbuilder_loop_cond_co_block_lowering_projection_policy_guard.sh
```

## Evidence

```text
visibility:
  pub(super)

callsites:
  src/mir/builder/control_flow/plan/features/loop_cond_co_pipeline.rs
  src/mir/builder/control_flow/plan/features/loop_cond_co_group_if.rs
  src/mir/builder/control_flow/plan/features/loop_cond_co_continue_if.rs

return_type:
  Result<Vec<LoweredRecipe>, String>

block lowering markers:
  for stmt in items
  lower_continue_only_stmt
  plans.append
  Ok(plans)
```

## Acceptance

```text
policy = PrivateBlockLoweringHelper
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
1. MIRBUILDER-LOOP-COND-CO-CLEANUP-PROJECTION-POLICY-001
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
