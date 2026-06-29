# 1853 - MIRBUILDER-LOOP-COND-CO-CONTINUE-IF-PRELUDE-SPAN-PROJECTION-POLICY-001

## Token

```text
MIRBUILDER-LOOP-COND-CO-CONTINUE-IF-PRELUDE-SPAN-PROJECTION-POLICY-001
```

## Purpose

Resolve whether `lower_continue_if_prelude_span` should become a standalone
Hako projection surface.

## Decision

```text
selected_policy = PrivatePreludeSpanHelper
owner_edge = loop_cond_continue_only
projection_surface_selected = 0
```

The source surface extracts a continue-if prelude span and delegates each
statement to `lower_stmt_ast`. It is a route-local helper used by the no-else
continue-if lowering path.

It is not a standalone semantic owner edge.

## Output

```text
fixture:
  docs/development/current/main/design/fixtures/rust-lifecycle/
    mirbuilder-loop-cond-co-continue-if-prelude-span-projection-policy-v0.json

guard:
  tools/checks/
    rust_lifecycle_mirbuilder_loop_cond_co_continue_if_prelude_span_projection_policy_guard.sh
```

## Evidence

```text
visibility:
  pub(super)

callsite:
  src/mir/builder/control_flow/plan/features/loop_cond_co_continue_if.rs

return_type:
  Result<Vec<LoweredRecipe>, String>

prelude span markers:
  get_body_span
  for stmt in prelude_body
  lower_stmt_ast
  out.append
```

## Acceptance

```text
policy = PrivatePreludeSpanHelper
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
1. MIRBUILDER-LOOP-COND-CO-CONTINUE-IF-NO-ELSE-PROJECTION-POLICY-001
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
