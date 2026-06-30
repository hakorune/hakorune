# 1909 - MIRBUILDER-GENERIC-LOOP-BODY-CHECK-CALL-EXPR-MATCHERS-PROJECTION-POLICY-001

## Token

```text
MIRBUILDER-GENERIC-LOOP-BODY-CHECK-CALL-EXPR-MATCHERS-PROJECTION-POLICY-001
```

## Purpose

Resolve the `CallExprMatchers` subcluster selected by the body-check expression
matcher policy.

This card does not generate Hako. It materializes a source-extracted call
matcher descriptor for the `_is_space` and `substring` method-call patterns so
the call matcher surface is no longer an opaque missing projection policy.

## Output

```text
tool:
  tools/rust_lifecycle/
    mirbuilder_generic_loop_body_check_call_expr_matchers_projection_policy.py

fixture:
  docs/development/current/main/design/fixtures/rust-lifecycle/
    mirbuilder-generic-loop-body-check-call-expr-matchers-projection-policy-v0.json

guard:
  tools/checks/
    rust_lifecycle_mirbuilder_generic_loop_body_check_call_expr_matchers_projection_policy_guard.sh
```

## Descriptor

```text
matches_is_space_call:
  MethodCall("_is_space")
  argument_policy = AnyArgumentMatchesSubstringLoopVar

matches_substring_call_with_loop_var:
  MethodCall("substring")
  arguments.len = 2
  accepted_argument_shapes:
    LoopVar, LoopVarPlusOne
    LoopVarMinusOne, LoopVar
```

## Decision

```text
kind = SelectMatcherDescriptorPolicy

next_card =
  MIRBUILDER-GENERIC-LOOP-BODY-CHECK-COMPARE-EXPR-MATCHERS-PROJECTION-POLICY-001
```

## Acceptance

```text
source_count = 2
descriptor_id = generic_loop_body_check_call_expr_matchers_v1
matcher_descriptor_selected = 1
hako_projection_selected = 0
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
no Hako projection selected
no Hako generation
no HakoAdopted decision
no native seed materialization
no Source Selfhost claim
```
