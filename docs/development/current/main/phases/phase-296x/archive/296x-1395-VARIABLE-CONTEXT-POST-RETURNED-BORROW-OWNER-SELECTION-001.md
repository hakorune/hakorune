# 296x-1395 VARIABLE-CONTEXT-POST-RETURNED-BORROW-OWNER-SELECTION-001

Status: closed
Date: 2026-06-20

## Purpose

Select the next VariableContext lifecycle owner after the returned borrow
boundary has been inventoried.

## Selected By

```text
296x-1394-VARIABLE-CONTEXT-RETURNED-BORROW-BOUNDARY-INVENTORY-001
```

## Candidate Owners

```text
A. Immutable map BorrowView probe
   value: tests/observation consumers can validate owner-carrying read borrow
   risk: must not include carrier/PHI semantics

B. Mutable map API replacement selection
   value: closes variable_map_mut() as Deny or replaces it with explicit APIs
   risk: may require Rust API changes, so selection must be explicit first

C. Snapshot/restore ownership
   value: covers clone and ReplaceOwned map transfer
   risk: old-map cleanup and clone semantics must be explicit

D. Carrier/PHI lifecycle inventory
   value: connects VariableContext map read to JoinIR carrier extraction
   risk: can expand beyond VariableContext if opened too early
```

## Acceptance

```text
next_owner_selected=1
selected_owner_scope_documented=1
non_selected_owners_parked=1
implementation_started=0
full_VariableContext_parity_claim=0
carrier_PHI_lifecycle_claim=0
```

Checks:

```bash
git diff --check
bash tools/checks/current_state_pointer_guard.sh
```

## Decision

```text
selected_owner=A
selected_next_task=VARIABLE-CONTEXT-IMMUTABLE-MAP-BORROWVIEW-PROBE-001
implementation_started=0
```

Reason:

```text
The returned-borrow inventory shows a narrow read-only path through tests and
region observation. It can validate owner-carrying BorrowView vocabulary
without carrier/PHI semantics, mutable borrow replacement, or snapshot/restore.
```

Selected scope:

```text
variable_map() read-only BorrowView probe
consumer_scope=tests_and_region_observation_only
carrier_PHI_claim=0
mutable_map_claim=0
snapshot_restore_claim=0
```

Non-selected owners:

```text
B mutable map API replacement:
  parked because it may require Rust API changes

C snapshot/restore ownership:
  parked until borrow view probe proves read alias shape

D carrier/PHI lifecycle inventory:
  parked until read-only borrow and carrier-sensitive contracts are separated
```

Next:

```text
296x-1396-VARIABLE-CONTEXT-IMMUTABLE-MAP-BORROWVIEW-PROBE-001
```

## Stop Line

```text
do_not_start_borrowview_probe_before_selection=1
do_not_change_Rust_API_before_selection=1
do_not_start_snapshot_restore_before_selection=1
do_not_start_carrier_PHI_before_selection=1
```
