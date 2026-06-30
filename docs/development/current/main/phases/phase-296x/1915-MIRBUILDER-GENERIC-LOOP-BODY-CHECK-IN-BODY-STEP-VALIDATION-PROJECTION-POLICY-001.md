# 1915 - MIRBUILDER-GENERIC-LOOP-BODY-CHECK-IN-BODY-STEP-VALIDATION-PROJECTION-POLICY-001

## Token

```text
MIRBUILDER-GENERIC-LOOP-BODY-CHECK-IN-BODY-STEP-VALIDATION-PROJECTION-POLICY-001
```

## Purpose

Resolve the `InBodyStepValidation` subcluster selected after the tail
control-flow probe.

This card materializes a source-extracted strict/reject validation descriptor
for `validate_in_body_step` and `validate_in_body_step_v1`. It does not select
Hako projection for the validators.

## Output

```text
tool:
  tools/rust_lifecycle/
    mirbuilder_generic_loop_body_check_in_body_step_validation_projection_policy.py

fixture:
  docs/development/current/main/design/fixtures/rust-lifecycle/
    mirbuilder-generic-loop-body-check-in-body-step-validation-projection-policy-v0.json

guard:
  tools/checks/
    rust_lifecycle_mirbuilder_generic_loop_body_check_in_body_step_validation_projection_policy_guard.sh
```

## Descriptor

```text
validate_in_body_step:
  version = v0
  continue policy = RejectAnyContinueInBodyBeforeTailScan
  reject dispatch = reject_or_false(strict, reason.as_freeze_message())

validate_in_body_step_v1:
  version = v1
  continue policy = AllowContinueBeforeTailScan
  reject dispatch = reject_or_false(strict, reason.as_freeze_message())
```

## Decision

```text
kind = SelectValidatorDescriptorPolicy

next_card =
  MIRBUILDER-GENERIC-LOOP-BODY-CHECK-CONTINUE-IF-STEP-VALIDATION-PROJECTION-POLICY-001
```

## Acceptance

```text
source_count = 2
descriptor_id = generic_loop_body_check_in_body_step_validation_v1
validator_descriptor_selected = 1
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
