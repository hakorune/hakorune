# 296x-1408 PHI-CARRIER-LIFECYCLE-CONSUMER-INVENTORY-001

Status: closed
Date: 2026-06-20

## Purpose

Inventory PHI carrier lifecycle consumers after automatic and explicit
CarrierInfo snapshot plans are green.

## Selected By

```text
296x-1407-POST-EXPLICIT-CARRIER-SNAPSHOT-OWNER-SELECTION-001
```

## Scope

```text
source=src/mir/join_ir/lowering/carrier_info
design=docs/development/current/main/design/phi-carrier-lifecycle-consumer-inventory.md
```

Inventory:

```text
CarrierVar.join_id producer/consumer boundary
CarrierInfo.promoted_body_locals producer/consumer boundary
CarrierInfo.trim_helper producer/consumer boundary
CarrierInfo::merge_from mutation boundary
read-only CarrierInfo consumer boundary
```

## Non-Goals

```text
do_not_add_resolver=1
do_not_add_verifier=1
do_not_add_emitter=1
do_not_add_HakoLifecyclePlan_kind=1
do_not_change_Rust_code=1
do_not_claim_full_VariableContext_parity=1
do_not_claim_MirBuilder_wide_lifecycle_parity=1
```

## Acceptance

```text
join_id_consumer_boundary_documented=1
promoted_body_locals_boundary_documented=1
trim_helper_boundary_documented=1
merge_from_boundary_documented=1
read_only_consumers_documented=1
follow_up_owners_named=1
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
join_id_consumer_boundary_documented=1
promoted_body_locals_boundary_documented=1
trim_helper_boundary_documented=1
merge_from_boundary_documented=1
read_only_consumers_documented=1
follow_up_owners_named=1
implementation_started=0
general_resolver_started=0
```

Evidence:

```text
docs/development/current/main/design/phi-carrier-lifecycle-consumer-inventory.md
```

Next:

```text
296x-1409-POST-PHI-CARRIER-CONSUMER-INVENTORY-OWNER-SELECTION-001
```

## Stop Line

```text
do_not_promote_CarrierInfo_snapshot_to_PHI_lifecycle_complete=1
do_not_start_HakoLifecycleResolver_from_this_row=1
do_not_modify_carrier_info_code_in_this_row=1
```
