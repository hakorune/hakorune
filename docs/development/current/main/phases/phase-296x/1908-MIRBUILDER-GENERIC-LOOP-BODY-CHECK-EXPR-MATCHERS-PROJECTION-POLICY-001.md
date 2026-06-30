# 1908 - MIRBUILDER-GENERIC-LOOP-BODY-CHECK-EXPR-MATCHERS-PROJECTION-POLICY-001

## Token

```text
MIRBUILDER-GENERIC-LOOP-BODY-CHECK-EXPR-MATCHERS-PROJECTION-POLICY-001
```

## Purpose

Resolve the `BodyCheckExprMatchers` subcluster selected by the
GenericLoopPlan decomposition without selecting one mixed projection policy for
all expression matchers.

The selected subcluster contains call, compare, control-return, and composite
trim-condition matchers. This card therefore records a module-role
decomposition and selects the narrow call matcher subcluster first.

## Output

```text
tool:
  tools/rust_lifecycle/
    mirbuilder_generic_loop_body_check_expr_matchers_projection_policy.py

fixture:
  docs/development/current/main/design/fixtures/rust-lifecycle/
    mirbuilder-generic-loop-body-check-expr-matchers-projection-policy-v0.json

guard:
  tools/checks/
    rust_lifecycle_mirbuilder_generic_loop_body_check_expr_matchers_projection_policy_guard.sh
```

## Subclusters

```text
CallExprMatchers:
  body_check/expr_matchers/call.rs

CompareExprMatchers:
  body_check/expr_matchers/compare.rs

ControlReturnExprMatchers:
  body_check/expr_matchers/control.rs

CompositeTrimConditionMatcher:
  body_check/expr_matchers/mod.rs
```

## Decision

```text
kind = SelectExpressionMatcherSubcluster
selected_subcluster = CallExprMatchers

next_card =
  MIRBUILDER-GENERIC-LOOP-BODY-CHECK-CALL-EXPR-MATCHERS-PROJECTION-POLICY-001
```

## Acceptance

```text
source_subcluster = BodyCheckExprMatchers
source_count = 12
subcluster_count = 4
whole_expr_matcher_projection = 0
projection_surface_selected = 0
candidate_count_as_proof = 0
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

## Non-Claims

```text
no whole expression-matcher projection policy
no Hako generation
no HakoAdopted decision
no native seed materialization
no Source Selfhost claim
```
