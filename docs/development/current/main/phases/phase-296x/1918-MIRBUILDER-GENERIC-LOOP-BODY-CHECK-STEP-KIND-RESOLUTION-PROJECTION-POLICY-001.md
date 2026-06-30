# 1918 - MIRBUILDER-GENERIC-LOOP-BODY-CHECK-STEP-KIND-RESOLUTION-PROJECTION-POLICY-001

## Token

```text
MIRBUILDER-GENERIC-LOOP-BODY-CHECK-STEP-KIND-RESOLUTION-PROJECTION-POLICY-001
```

## Purpose

Resolve the GenericLoop body-check step-kind dispatch after all step
validation leaf descriptors are materialized.

This card records the source-extracted handoff from `StepPlacement` variants to
the validator descriptors. It does not select Hako projection for the dispatch.

## Output

```text
tool:
  tools/rust_lifecycle/
    mirbuilder_generic_loop_body_check_step_kind_resolution_projection_policy.py

fixture:
  docs/development/current/main/design/fixtures/rust-lifecycle/
    mirbuilder-generic-loop-body-check-step-kind-resolution-projection-policy-v0.json

guard:
  tools/checks/
    rust_lifecycle_mirbuilder_generic_loop_body_check_step_kind_resolution_projection_policy_guard.sh
```

## Dispatch

```text
StepPlacement::InBody:
  v0 -> validate_in_body_step
  v1 -> validate_in_body_step_v1

StepPlacement::InContinueIf:
  validate_continue_if_step

StepPlacement::InBreakElseIf:
  validate_break_else_if_step

StepPlacement::Last / other:
  accept_without_step_validator
```

## Decision

```text
kind = SelectDispatchResolutionPolicy

next_card =
  MIRBUILDER-GENERIC-LOOP-BODY-CHECK-STEP-VALIDATION-CLOSEOUT-001
```

## Acceptance

```text
dispatch_resolution_selected = 1
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
