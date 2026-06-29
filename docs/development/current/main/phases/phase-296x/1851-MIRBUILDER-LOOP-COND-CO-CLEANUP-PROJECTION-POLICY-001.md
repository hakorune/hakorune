# 1851 - MIRBUILDER-LOOP-COND-CO-CLEANUP-PROJECTION-POLICY-001

## Token

```text
MIRBUILDER-LOOP-COND-CO-CLEANUP-PROJECTION-POLICY-001
```

## Purpose

Resolve whether `apply_fallthrough_continue_exit` should become a standalone
Hako projection surface for the loop-cond continue-only route.

## Decision

```text
selected_policy = PrivateRouteCleanupHelper
owner_edge = loop_cond_continue_only
projection_surface_selected = 0
```

The source surface appends the route-local fallthrough continue exit by building
a continue-with-PHI-args exit and pushing it onto `body_plans`.

It is not a standalone semantic owner edge.

## Output

```text
fixture:
  docs/development/current/main/design/fixtures/rust-lifecycle/
    mirbuilder-loop-cond-co-cleanup-projection-policy-v0.json

guard:
  tools/checks/
    rust_lifecycle_mirbuilder_loop_cond_co_cleanup_projection_policy_guard.sh
```

## Evidence

```text
visibility:
  pub(in crate::mir::builder)

callsite:
  src/mir/builder/control_flow/plan/features/loop_cond_co_pipeline.rs

return_type:
  Result<(), String>

cleanup markers:
  build_continue_with_phi_args
  body_plans.push(CorePlan::Exit(exit))
  Ok(())
```

## Acceptance

```text
policy = PrivateRouteCleanupHelper
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
1. MIRBUILDER-LOOP-COND-CO-CONTINUE-IF-SURFACE-CLASSIFICATION-001
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
