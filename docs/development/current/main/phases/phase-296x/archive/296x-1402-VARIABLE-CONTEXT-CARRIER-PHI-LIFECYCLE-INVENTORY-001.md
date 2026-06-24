# 296x-1402 VARIABLE-CONTEXT-CARRIER-PHI-LIFECYCLE-INVENTORY-001

Status: closed
Date: 2026-06-20

## Purpose

Inventory carrier-sensitive VariableContext map consumers before any
carrier/PHI lifecycle projection or HakoLifecycleResolver implementation starts.

## Selected By

```text
296x-1401-VARIABLE-CONTEXT-POST-MUTABLE-DENY-OWNER-SELECTION-001
```

## Scope

```text
source=VariableContext.variable_map read consumers
implementation_started=0
```

Inventory:

```text
CarrierInfo::from_variable_map
CarrierInfo::with_explicit_carriers
region observer slot classification
PHI-sensitive ownership questions
```

## Non-Goals

```text
do_not_add_carrier_PHI_plan=1
do_not_add_general_resolver=1
do_not_change_Rust_code=1
do_not_change_Hako_code=1
do_not_claim_full_VariableContext_parity=1
```

## Acceptance

```text
carrier_sensitive_consumers_inventoried=1
carrier_info_contract_boundary_named=1
region_observer_boundary_named=1
PHI_lifecycle_questions_named=1
implementation_started=0
resolver_started=0
```

Checks:

```bash
git diff --check
bash tools/checks/current_state_pointer_guard.sh
```

## Closeout

```text
carrier_sensitive_consumers_inventoried=1
carrier_info_contract_boundary_named=1
region_observer_boundary_named=1
PHI_lifecycle_questions_named=1
implementation_started=0
resolver_started=0
```

Evidence:

```text
SSOT=docs/development/current/main/design/variable-context-carrier-phi-lifecycle-inventory.md
```

Inventory result:

```text
CarrierInfo::from_variable_map:
  future candidate=CarrierSnapshotFromBorrowView

CarrierInfo::with_explicit_carriers:
  future candidate=ExplicitCarrierSnapshotFromBorrowView

region observer:
  observation-only BorrowView-style read

PHI lifecycle:
  still open; join_id / promoted_body_locals / trim_helper consumers need a
  separate inventory before implementation
```

Next:

```text
296x-1403-POST-CARRIER-PHI-INVENTORY-OWNER-SELECTION-001
```

## Stop Line

```text
do_not_fold_carrier_info_into_BorrowView_probe=1
do_not_claim_PHI_lifecycle_safety_from_map_iteration_order=1
do_not_start_resolver_before_inventory_closeout=1
```
