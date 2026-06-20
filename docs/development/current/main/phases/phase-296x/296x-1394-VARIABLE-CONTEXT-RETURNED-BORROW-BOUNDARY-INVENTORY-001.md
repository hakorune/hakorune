# 296x-1394 VARIABLE-CONTEXT-RETURNED-BORROW-BOUNDARY-INVENTORY-001

Status: closed
Date: 2026-06-20

## Purpose

Document the lifecycle boundary for `VariableContext::variable_map()` and
`VariableContext::variable_map_mut()` before any returned-borrow projection,
snapshot/restore projection, carrier/PHI integration, or lifecycle resolver
implementation starts.

## Selected By

```text
296x-1393-RUST-LIFECYCLE-POST-VARIABLE-SIMPLE-MAP-OWNER-SELECTION-001
```

## Scope

```text
source=crates/hakorune_mir_builder/src/variable_context.rs
methods=variable_map(),variable_map_mut()
implementation_started=0
```

Document:

```text
immutable returned BTreeMap borrow consumers
mutable returned BTreeMap borrow consumers
carrier-sensitive consumers
initial Allow/Deny policy
next implementation row candidates
```

## Non-Goals

```text
do_not_change_Rust_code=1
do_not_change_Hako_code=1
do_not_add_resolver=1
do_not_project_snapshot_restore=1
do_not_claim_full_VariableContext_parity=1
do_not_claim_carrier_PHI_lifecycle_parity=1
```

## Acceptance

```text
returned_borrow_boundary_ssot_added=1
variable_map_consumers_classified=1
variable_map_mut_consumers_classified=1
initial_policy_selected=1
followup_rows_named=1
implementation_started=0
```

Checks:

```bash
git diff --check
bash tools/checks/current_state_pointer_guard.sh
```

## Closeout

```text
returned_borrow_boundary_ssot_added=1
variable_map_consumers_classified=1
variable_map_mut_consumers_classified=1
initial_policy_selected=1
followup_rows_named=1
implementation_started=0
```

Evidence:

```text
SSOT=docs/development/current/main/design/variable-context-returned-borrow-boundary-inventory.md

variable_map() consumers:
  read-only tests
  region observer classification
  CarrierInfo carrier-sensitive extraction

variable_map_mut() consumers:
  external_callsite_count=0
  still denied as public returned mutable alias boundary
```

Selected initial policy:

```text
variable_map():
  future candidate=OwnerCarryingBorrowView(read)
  read-only observation may be probed first
  carrier-sensitive consumers require a separate contract

variable_map_mut():
  Deny(ReturnedMutableBorrow)
```

Next:

```text
296x-1395-VARIABLE-CONTEXT-POST-RETURNED-BORROW-OWNER-SELECTION-001
```

## Stop Line

```text
do_not_model_returned_mut_BTreeMap_as_direct_owner_mutation=1
do_not_emit_naked_borrow_alias=1
do_not_move_snapshot_restore_into_this_row=1
do_not_start_carrier_PHI_resolver_from_this_row=1
```
