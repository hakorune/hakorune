# 296x-1410 PHI-CARRIER-JOIN-ID-LIFECYCLE-PRODUCER-INVENTORY-001

Status: closed
Date: 2026-06-20

## Purpose

Inventory whether `CarrierVar.join_id` has a production lifecycle producer
before any resolver or PHI carrier probe treats it as live plan state.

## Selected By

```text
296x-1409-POST-PHI-CARRIER-CONSUMER-INVENTORY-OWNER-SELECTION-001
```

## Scope

```text
field=CarrierVar.join_id
source=src/mir
design=docs/development/current/main/design/phi-carrier-lifecycle-consumer-inventory.md
```

Search:

```bash
rg -n "join_id\\s*=|\\.join_id\\s*=|join_id:\\s*Some|join_id:\\s*None|resolve_promoted_join_id|CarrierVar \\{" src/mir -g '*.rs'
```

## Finding

```text
production_join_id_initializers=None_only
production_join_id_mutation_assignment=0
production_join_id_Some_assignment=0
test_fixture_join_id_Some_assignment=1
```

Observed production constructors initialize `join_id` as `None`.
Observed `Some(ValueId)` construction is limited to tests / fixtures.

## Non-Goals

```text
do_not_add_join_id_producer=1
do_not_remove_join_id_field=1
do_not_change_scope_manager=1
do_not_add_resolver=1
do_not_claim_PHI_lifecycle_complete=1
```

## Acceptance

```text
join_id_producer_absence_documented=1
production_assignment_search_recorded=1
test_fixture_only_Some_documented=1
follow_up_decision_required=1
implementation_started=0
general_resolver_started=0
```

Checks:

```bash
git diff --check
bash tools/checks/current_state_pointer_guard.sh
```

## Closeout

```text
join_id_producer_absence_documented=1
production_assignment_search_recorded=1
test_fixture_only_Some_documented=1
follow_up_decision_required=1
implementation_started=0
general_resolver_started=0
```

Evidence:

```text
docs/development/current/main/design/phi-carrier-lifecycle-consumer-inventory.md
```

Next:

```text
296x-1411-POST-JOIN-ID-PRODUCER-INVENTORY-OWNER-SELECTION-001
```

## Stop Line

```text
do_not_build_resolver_on_test_only_join_id=1
do_not_add_dummy_join_id_assignment=1
do_not_delete_join_id_without_owner_selection=1
```
