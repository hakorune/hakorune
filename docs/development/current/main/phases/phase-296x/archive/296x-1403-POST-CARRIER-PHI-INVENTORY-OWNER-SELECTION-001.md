# 296x-1403 POST-CARRIER-PHI-INVENTORY-OWNER-SELECTION-001

Status: closed
Date: 2026-06-20

## Purpose

Select the next lifecycle owner after VariableContext carrier/PHI inventory is
closed.

## Selected By

```text
296x-1402-VARIABLE-CONTEXT-CARRIER-PHI-LIFECYCLE-INVENTORY-001
```

## Candidate Owners

```text
A. VariableContext CarrierSnapshotFromBorrowView probe
   value: model CarrierInfo::from_variable_map as a snapshot from an
   owner-carrying read BorrowView
   risk: must not claim downstream PHI join_id lifecycle

B. ExplicitCarrierSnapshotFromBorrowView probe
   value: model CarrierInfo::with_explicit_carriers separately
   risk: requested names / missing carrier fail-fast must stay explicit

C. PHI carrier lifecycle consumer inventory
   value: names join_id / promoted_body_locals / trim_helper consumers
   risk: can expand into broader JoinIR lifecycle design

D. HakoLifecycleResolver read-only skeleton
   value: starts consuming proven fixture family
   risk: can become general before carrier snapshot contract is probed
```

## Acceptance

```text
next_owner_selected=1
selected_owner_scope_documented=1
non_selected_owners_parked=1
implementation_started=0
full_VariableContext_parity_claim=0
MirBuilder_wide_lifecycle_claim=0
```

Checks:

```bash
git diff --check
bash tools/checks/current_state_pointer_guard.sh
```

## Decision

```text
selected_owner=A
selected_next_task=VARIABLE-CONTEXT-CARRIER-SNAPSHOT-PLAN-PROBE-001
implementation_started=0
```

Reason:

```text
CarrierInfo::from_variable_map is the broad carrier-sensitive consumer named
by the inventory. It can be modeled as a read-only snapshot from an
owner-carrying BorrowView without claiming downstream PHI join_id lifecycle.
```

Selected scope:

```text
fixture CarrierInfo::from_variable_map
plan kind=CarrierSnapshotFromBorrowView
deny downstream PHI join_id / promoted_body_locals / trim_helper lifecycle
```

Non-selected owners:

```text
B ExplicitCarrierSnapshotFromBorrowView:
  parked until automatic carrier snapshot is fixed

C PHI carrier lifecycle consumer inventory:
  parked until carrier snapshot output contract is fixed

D HakoLifecycleResolver read-only skeleton:
  parked until carrier snapshot contract is probed
```

Next:

```text
296x-1404-VARIABLE-CONTEXT-CARRIER-SNAPSHOT-PLAN-PROBE-001
```

## Stop Line

```text
do_not_start_carrier_snapshot_before_selection=1
do_not_start_PHI_consumer_inventory_before_selection=1
do_not_start_general_resolver_before_selection=1
```
