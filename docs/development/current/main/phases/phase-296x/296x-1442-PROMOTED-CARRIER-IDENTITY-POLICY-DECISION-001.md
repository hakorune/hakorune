# 296x-1442 PROMOTED-CARRIER-IDENTITY-POLICY-DECISION-001

Status: closed
Date: 2026-06-20

## Purpose

Choose the promoted carrier identity policy after inventorying join_id and
condition-binding options.

This row chooses policy only. It does not implement a producer, rewrite
resolution, or emit trim route lowering.

## Selected By

```text
296x-1441-POST-PROMOTED-CARRIER-IDENTITY-INVENTORY-OWNER-SELECTION-001
```

## Decision

```text
selected_policy=condition_binding_identity
CarrierVar_join_id_producer_selected=0
keep_denied_forever_selected=0
implementation_started=0
```

Reason:

```text
CarrierVar.join_id is parked/test vocabulary with no production Some(ValueId)
producer. ConditionBinding already carries a JoinIR-local join_value for
condition-only values. The next safe step is to prove whether promoted
body-local names can map to the intended ConditionBinding, not to fabricate or
revive join_id.
```

## Acceptance

```text
policy_decision_recorded=1
selected_policy=condition_binding_identity
join_id_producer_added=0
condition_binding_rewrite_added=0
trim_route_lowering_still_denied=1
backend_behavior_changed=0
generated_program_execution_claim=0
```

Checks:

```bash
bash tools/checks/rust_lifecycle_promoted_carrier_identity_policy_guard.sh
bash tools/checks/rust_lifecycle_promoted_carrier_identity_inventory_guard.sh
git diff --check
bash tools/checks/current_state_pointer_guard.sh
cargo check -q --lib
```

## Stop Line

```text
do_not_implement_condition_binding_resolution=1
do_not_implement_join_id_producer=1
do_not_emit_trim_route_lowering=1
do_not_claim_generated_program_execution=1
```
