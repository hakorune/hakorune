# 296x-1438 TRIM-ROUTE-LOWERING-DECISION-PROBE-001

Status: closed
Date: 2026-06-20

## Purpose

Fixture-guard a read-only decision surface for trim route lowering.

This row does not lower trim routes. It proves the current facts can classify
the trim metadata as present, while executable route lowering remains denied
until promoted carrier identity / join_id proof exists.

## Selected By

```text
296x-1437-POST-TRIM-ROUTE-LOWERING-INVENTORY-OWNER-SELECTION-001
```

## Output

```text
design_doc=docs/development/current/main/design/trim-route-lowering-decision-probe.md
facts=docs/development/current/main/design/fixtures/rust-lifecycle/trim-route-lowering-decision-facts-v0.json
plan=docs/development/current/main/design/fixtures/rust-lifecycle/trim-route-lowering-decision-plan-v0.json
oracle=docs/development/current/main/design/fixtures/rust-lifecycle/trim-route-lowering-decision-oracle-vectors-v0.json
guard=tools/checks/rust_lifecycle_trim_route_lowering_decision_guard.sh
```

## Probe Result

```text
trim_route_metadata_candidate=1
metadata_preconditions_green=1
executable_route_lowering_decision=DenyMissingPromotedCarrierIdentity
join_id_producer_required=1
backend_behavior_changed=0
generated_program_execution_claim=0
```

## Acceptance

```text
decision_probe_fixture=green
metadata_candidate_allow=1
executable_lowering_allow=0
deny_reason=MissingPromotedCarrierIdentity
join_id_producer=0
backend_behavior_changed=0
resolver_selection_owner=0
```

Checks:

```bash
bash tools/checks/rust_lifecycle_trim_route_lowering_decision_guard.sh
bash tools/checks/rust_lifecycle_trim_route_lowering_inventory_guard.sh
git diff --check
bash tools/checks/current_state_pointer_guard.sh
cargo check -q --lib
```

## Stop Line

```text
do_not_emit_trim_route_lowering=1
do_not_add_backend_lowering=1
do_not_fabricate_join_id=1
do_not_treat_metadata_presence_as_executable_route_proof=1
do_not_claim_generated_program_execution=1
```
