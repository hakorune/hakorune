# 1912 - MIRBUILDER-GENERIC-LOOP-BODY-CHECK-TRIM-CONDITION-MATCHER-DECOMPOSITION-001

## Token

```text
MIRBUILDER-GENERIC-LOOP-BODY-CHECK-TRIM-CONDITION-MATCHER-DECOMPOSITION-001
```

## Purpose

Resolve the `CompositeTrimConditionMatcher` subcluster selected after the
control-return matcher descriptor.

The source matcher composes `matches_loop_var_compare` and
`matches_is_space_call` under `BinaryOperator::And`. This card therefore
records a composite descriptor that depends on the already materialized compare
and call matcher descriptors. It does not select a standalone projection policy
for the composite matcher.

## Output

```text
tool:
  tools/rust_lifecycle/
    mirbuilder_generic_loop_body_check_trim_condition_matcher_decomposition.py

fixture:
  docs/development/current/main/design/fixtures/rust-lifecycle/
    mirbuilder-generic-loop-body-check-trim-condition-matcher-decomposition-v0.json

guard:
  tools/checks/
    rust_lifecycle_mirbuilder_generic_loop_body_check_trim_condition_matcher_decomposition_guard.sh
```

## Composition

```text
AST root:
  BinaryOp(And)

required matchers:
  matches_loop_var_compare
    from generic_loop_body_check_compare_expr_matchers_v1

  matches_is_space_call
    from generic_loop_body_check_call_expr_matchers_v1
```

## Decision

```text
kind = SelectNextGenericLoopPlanSubcluster

next_card =
  MIRBUILDER-GENERIC-LOOP-BODY-CHECK-STEP-VALIDATION-PROJECTION-POLICY-001
```

## Acceptance

```text
source_count = 1
descriptor_id = generic_loop_body_check_trim_condition_matcher_v1
composite_descriptor_selected = 1
standalone_projection_selected = 0
new_matcher_semantics_invented = 0
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
no standalone composite matcher projection
no Hako generation
no HakoAdopted decision
no native seed materialization
no Source Selfhost claim
```
