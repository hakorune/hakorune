# 1914 - MIRBUILDER-GENERIC-LOOP-BODY-CHECK-TAIL-CONTROL-FLOW-PROBE-PROJECTION-POLICY-001

## Token

```text
MIRBUILDER-GENERIC-LOOP-BODY-CHECK-TAIL-CONTROL-FLOW-PROBE-PROJECTION-POLICY-001
```

## Purpose

Resolve the `TailControlFlowProbe` subcluster selected by the step-validation
decomposition.

This card materializes the pure tail statement scan descriptor for
`has_control_flow_after_step`. It intentionally keeps strict/reject validation
semantics out of this owner.

## Output

```text
tool:
  tools/rust_lifecycle/
    mirbuilder_generic_loop_body_check_tail_control_flow_probe_projection_policy.py

fixture:
  docs/development/current/main/design/fixtures/rust-lifecycle/
    mirbuilder-generic-loop-body-check-tail-control-flow-probe-projection-policy-v0.json

guard:
  tools/checks/
    rust_lifecycle_mirbuilder_generic_loop_body_check_tail_control_flow_probe_projection_policy_guard.sh
```

## Descriptor

```text
scan_range:
  body[(step_index + 1)..]

returns:
  bool

control_flow_predicates:
  is_exit_if(stmt)
  ASTNode::Break
  ASTNode::Continue
  ASTNode::Return
```

## Decision

```text
kind = SelectProbeDescriptorPolicy

next_card =
  MIRBUILDER-GENERIC-LOOP-BODY-CHECK-IN-BODY-STEP-VALIDATION-PROJECTION-POLICY-001
```

## Acceptance

```text
source_count = 1
descriptor_id = generic_loop_body_check_tail_control_flow_probe_v1
probe_descriptor_selected = 1
strict_reject_semantics_selected = 0
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
no strict/reject validation policy
no Hako projection selected
no Hako generation
no HakoAdopted decision
no native seed materialization
no Source Selfhost claim
```
