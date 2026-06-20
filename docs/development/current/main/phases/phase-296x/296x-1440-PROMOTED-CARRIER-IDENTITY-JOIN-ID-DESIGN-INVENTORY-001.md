# 296x-1440 PROMOTED-CARRIER-IDENTITY-JOIN-ID-DESIGN-INVENTORY-001

Status: closed
Date: 2026-06-20

## Purpose

Inventory the promoted carrier identity / join_id design choice after trim
route lowering is denied by `MissingPromotedCarrierIdentity`.

This row does not implement a production `CarrierVar.join_id` producer.

## Selected By

```text
296x-1439-POST-TRIM-ROUTE-LOWERING-DECISION-OWNER-SELECTION-001
```

## Output

```text
design_doc=docs/development/current/main/design/promoted-carrier-identity-join-id-design-inventory.md
guard=tools/checks/rust_lifecycle_promoted_carrier_identity_inventory_guard.sh
```

## Inventory Result

```text
current_join_id_status=parked_test_fixture_or_stale_vocabulary
production_join_id_producer=0
condition_binding_value_path_present=1
resolve_promoted_join_id_depends_on_join_id=1
implementation_started=0
backend_behavior_changed=0
```

## Acceptance

```text
promoted_carrier_identity_inventory=1
current_join_id_producer_still_absent=1
candidate_designs_documented=1
selected_implementation=none
trim_route_lowering_still_denied=1
backend_behavior_changed=0
generated_program_execution_claim=0
```

Checks:

```bash
bash tools/checks/rust_lifecycle_promoted_carrier_identity_inventory_guard.sh
bash tools/checks/rust_lifecycle_join_id_vocabulary_guard.sh
bash tools/checks/rust_lifecycle_trim_route_lowering_decision_guard.sh
git diff --check
bash tools/checks/current_state_pointer_guard.sh
cargo check -q --lib
```

## Stop Line

```text
do_not_implement_join_id_producer=1
do_not_repurpose_condition_bindings_without_design=1
do_not_fabricate_join_id=1
do_not_emit_trim_route_lowering=1
do_not_claim_generated_program_execution=1
```
