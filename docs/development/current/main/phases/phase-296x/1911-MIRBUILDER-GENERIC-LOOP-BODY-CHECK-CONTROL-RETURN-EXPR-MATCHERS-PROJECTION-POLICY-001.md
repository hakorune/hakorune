# 1911 - MIRBUILDER-GENERIC-LOOP-BODY-CHECK-CONTROL-RETURN-EXPR-MATCHERS-PROJECTION-POLICY-001

## Token

```text
MIRBUILDER-GENERIC-LOOP-BODY-CHECK-CONTROL-RETURN-EXPR-MATCHERS-PROJECTION-POLICY-001
```

## Purpose

Resolve the `ControlReturnExprMatchers` subcluster selected after the compare
matcher descriptor.

This card does not generate Hako. It materializes a source-extracted descriptor
for the `if` / return shape predicates used by GenericLoop body checking.

## Output

```text
tool:
  tools/rust_lifecycle/
    mirbuilder_generic_loop_body_check_control_return_expr_matchers_projection_policy.py

fixture:
  docs/development/current/main/design/fixtures/rust-lifecycle/
    mirbuilder-generic-loop-body-check-control-return-expr-matchers-projection-policy-v0.json

guard:
  tools/checks/
    rust_lifecycle_mirbuilder_generic_loop_body_check_control_return_expr_matchers_projection_policy_guard.sh
```

## Descriptor

```text
if-return literal:
  condition = loop_var == literal
  then = return integer literal

if-return var:
  condition = loop_var == 0
  then = return named variable

if-return local:
  condition = loop_var == literal
  then = local literal init; return local

if-else variants:
  single return literal / variable / local patterns
```

## Decision

```text
kind = SelectMatcherDescriptorPolicy

next_card =
  MIRBUILDER-GENERIC-LOOP-BODY-CHECK-TRIM-CONDITION-MATCHER-DECOMPOSITION-001
```

## Acceptance

```text
source_count = 6
descriptor_id = generic_loop_body_check_control_return_expr_matchers_v1
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
