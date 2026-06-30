# 1913 - MIRBUILDER-GENERIC-LOOP-BODY-CHECK-STEP-VALIDATION-PROJECTION-POLICY-001

## Token

```text
MIRBUILDER-GENERIC-LOOP-BODY-CHECK-STEP-VALIDATION-PROJECTION-POLICY-001
```

## Purpose

Resolve the `BodyCheckStepValidation` subcluster selected after body-check
expression matchers.

The source module mixes a pure tail control-flow probe with strict/reject
validators. This card decomposes the source into narrower subclusters instead
of selecting one whole step-validation projection policy.

## Output

```text
tool:
  tools/rust_lifecycle/
    mirbuilder_generic_loop_body_check_step_validation_projection_policy.py

fixture:
  docs/development/current/main/design/fixtures/rust-lifecycle/
    mirbuilder-generic-loop-body-check-step-validation-projection-policy-v0.json

guard:
  tools/checks/
    rust_lifecycle_mirbuilder_generic_loop_body_check_step_validation_projection_policy_guard.sh
```

## Subclusters

```text
TailControlFlowProbe:
  has_control_flow_after_step

InBodyStepValidation:
  validate_in_body_step
  validate_in_body_step_v1

ContinueIfStepValidation:
  validate_continue_if_step

BreakElseIfStepValidation:
  validate_break_else_if_step
```

## Decision

```text
kind = SelectStepValidationSubcluster
selected_subcluster = TailControlFlowProbe

next_card =
  MIRBUILDER-GENERIC-LOOP-BODY-CHECK-TAIL-CONTROL-FLOW-PROBE-PROJECTION-POLICY-001
```

## Acceptance

```text
source_subcluster = BodyCheckStepValidation
source_count = 5
subcluster_count = 4
whole_step_validation_projection = 0
projection_surface_selected = 0
strict_reject_semantics_isolated = 1
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
no whole step-validation projection policy
no Hako generation
no HakoAdopted decision
no native seed materialization
no Source Selfhost claim
```
