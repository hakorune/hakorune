# 1839 - MIRBUILDER-LOOP-COND-BC-CLEANUP-APPLICATION-PROJECTION-POLICY-001

## Token

```text
MIRBUILDER-LOOP-COND-BC-CLEANUP-APPLICATION-PROJECTION-POLICY-001
```

## Purpose

Resolve whether `apply_loop_cond_break_continue_cleanup` should be a
standalone Hako projection surface.

## Decision

```text
selected_policy = PrivateRouteCleanupHelper
owner_edge = loop_cond_break_continue
projection_surface_selected = 0
```

The source surface is route-local cleanup under `loop_cond_break_continue`.
It mutates the lowered body plan list by appending the route fallthrough
continue exit when needed; it is not a standalone semantic owner edge.

## Output

```text
fixture:
  docs/development/current/main/design/fixtures/rust-lifecycle/
    mirbuilder-loop-cond-bc-cleanup-application-projection-policy-v0.json

guard:
  tools/checks/
    rust_lifecycle_mirbuilder_loop_cond_bc_cleanup_application_projection_policy_guard.sh
```

## Evidence

```text
visibility:
  pub(in crate::mir::builder)

callsite:
  src/mir/builder/control_flow/plan/features/loop_cond_bc.rs

return_type:
  Result<LoopCondBreakContinueCleanupResult, String>
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
1. MIRBUILDER-LOOP-COND-BC-CLEANUP-EXIT-PREDICATE-PROJECTION-POLICY-001
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
