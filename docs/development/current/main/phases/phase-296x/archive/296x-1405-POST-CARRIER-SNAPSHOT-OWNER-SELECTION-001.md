# 296x-1405 POST-CARRIER-SNAPSHOT-OWNER-SELECTION-001

Status: closed
Date: 2026-06-20

## Purpose

Select the next lifecycle owner after
`CarrierInfo::from_variable_map` snapshot fixtures are green.

## Selected By

```text
296x-1404-VARIABLE-CONTEXT-CARRIER-SNAPSHOT-PLAN-PROBE-001
```

## Candidate Owners

```text
A. ExplicitCarrierSnapshotFromBorrowView probe
   value: model CarrierInfo::with_explicit_carriers separately
   risk: requested names and missing-carrier fail-fast must stay explicit

B. PHI carrier lifecycle consumer inventory
   value: names join_id / promoted_body_locals / trim_helper consumers
   risk: can expand into broader JoinIR lifecycle design

C. HakoLifecycleResolver read-only skeleton
   value: starts consuming proven fixture family
   risk: can become general before explicit carriers / PHI consumers are named
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
selected_next_task=VARIABLE-CONTEXT-EXPLICIT-CARRIER-SNAPSHOT-PROBE-001
implementation_started=0
```

Reason:

```text
CarrierInfo::with_explicit_carriers is the remaining carrier snapshot variant.
It shares the BorrowView input boundary but adds requested-name ownership and
missing-carrier fail-fast behavior. That is still smaller than PHI consumer
inventory or a general resolver.
```

Selected scope:

```text
fixture CarrierInfo::with_explicit_carriers
plan kind=ExplicitCarrierSnapshotFromBorrowView
preserve missing carrier fail-fast
deny downstream PHI lifecycle
```

Non-selected owners:

```text
B PHI carrier lifecycle consumer inventory:
  parked until explicit carrier snapshot is fixed

C HakoLifecycleResolver read-only skeleton:
  parked until both carrier snapshot variants are fixed
```

Next:

```text
296x-1406-VARIABLE-CONTEXT-EXPLICIT-CARRIER-SNAPSHOT-PROBE-001
```

## Stop Line

```text
do_not_start_explicit_carrier_snapshot_before_selection=1
do_not_start_PHI_consumer_inventory_before_selection=1
do_not_start_general_resolver_before_selection=1
```
