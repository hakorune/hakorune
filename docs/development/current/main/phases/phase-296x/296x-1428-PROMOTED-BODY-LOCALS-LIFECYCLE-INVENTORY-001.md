# 296x-1428 PROMOTED-BODY-LOCALS-LIFECYCLE-INVENTORY-001

Status: closed
Date: 2026-06-20

## Purpose

Inventory `CarrierInfo.promoted_body_locals` ownership before any resolver,
verifier, or emitter path treats promoted-name metadata as a proven lifecycle
owner.

## Selected By

```text
296x-1427-POST-TRIM-HELPER-PRODUCER-OWNER-SELECTION-001
```

## Scope

```text
inventory=docs/development/current/main/design/promoted-body-locals-lifecycle-inventory.md
guard=tools/checks/rust_lifecycle_promoted_body_locals_inventory_guard.sh
```

Decision:

```text
promoted_body_locals_lifecycle_owner_selected=0
promoted_body_locals_inventory_only=1
default_carrier_snapshots_start_empty=1
trim_producer_records_name=1
digitpos_producer_records_name=1
merge_from_deduplicates_names=1
join_id_producer=0
resolver_allows_promoted_body_locals_owner=0
emitter_claims_promoted_body_locals_owner=0
```

## Acceptance

```text
promoted_body_locals_field_present=1
default_constructors_start_empty=present
trim_producer_records_name=1
digitpos_producer_records_name=1
merge_from_deduplicates_names=1
resolve_promoted_join_id_consumes_names=1
resolver_denies_promoted_body_locals_owner=1
verifier_denies_promoted_body_locals_owner=1
emitter_denies_promoted_body_locals_owner=1
implementation_started=0
backend_behavior_changed=0
```

Checks:

```bash
bash tools/checks/rust_lifecycle_promoted_body_locals_inventory_guard.sh
git diff --check
bash tools/checks/current_state_pointer_guard.sh
```

## Closeout

```text
promoted_body_locals_field_present=1
default_constructors_start_empty=present
trim_producer_records_name=1
digitpos_producer_records_name=1
merge_from_deduplicates_names=1
resolve_promoted_join_id_consumes_names=1
resolver_denies_promoted_body_locals_owner=1
verifier_denies_promoted_body_locals_owner=1
emitter_denies_promoted_body_locals_owner=1
implementation_started=0
backend_behavior_changed=0
```

Evidence:

```bash
bash tools/checks/rust_lifecycle_promoted_body_locals_inventory_guard.sh
```

Guard output:

```text
output_contract=rust-lifecycle-promoted-body-locals-inventory-v0
promoted_body_locals_field_present=1
default_constructors_start_empty=present
trim_producer_records_name=1
digitpos_producer_records_name=1
merge_from_deduplicates_names=1
resolve_promoted_join_id_consumes_names=1
resolver_denies_promoted_body_locals_owner=green
verifier_denies_promoted_body_locals_owner=green
emitter_denies_promoted_body_locals_owner=green
join_id_producer=0
implementation_started=0
summary=ok
```

Next:

```text
296x-1429-POST-PROMOTED-BODY-LOCALS-INVENTORY-OWNER-SELECTION-001
```

## Stop Line

```text
do_not_use_promoted_body_locals_as_join_id_evidence=1
do_not_claim_trim_route_lowering_complete=1
do_not_add_resolver_selection_owner=1
do_not_modify_Rust_behavior=1
```

