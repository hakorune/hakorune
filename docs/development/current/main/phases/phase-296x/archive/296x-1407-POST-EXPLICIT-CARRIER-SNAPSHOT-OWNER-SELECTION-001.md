# 296x-1407 POST-EXPLICIT-CARRIER-SNAPSHOT-OWNER-SELECTION-001

Status: closed
Date: 2026-06-20

## Purpose

Select the next lifecycle owner after both automatic and explicit carrier
snapshot fixtures are green.

## Selected By

```text
296x-1406-VARIABLE-CONTEXT-EXPLICIT-CARRIER-SNAPSHOT-PROBE-001
```

## Candidate Owners

```text
A. PHI carrier lifecycle consumer inventory
   value: names join_id / promoted_body_locals / trim_helper consumers
   risk: can expand into broader JoinIR lifecycle design

B. HakoLifecycleResolver read-only skeleton
   value: starts consuming proven fixture family
   risk: can become general before PHI carrier consumers are named
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

## Selection

```text
selected_owner=A-lite
selected_next_task=PHI-CARRIER-LIFECYCLE-CONSUMER-INVENTORY-001
selected_reason=automatic and explicit CarrierInfo snapshots are green, but
join_id / promoted_body_locals / trim_helper consumers are not yet named as
lifecycle owners. A general resolver would otherwise read unnamed PHI carrier
semantics too early.
```

Parked:

```text
HAKO-LIFECYCLE-RESOLVER-READONLY-SKELETON-001:
  parked until PHI carrier lifecycle consumers are inventoried
```

## Closeout

```text
next_owner_selected=1
selected_owner_scope_documented=1
non_selected_owners_parked=1
implementation_started=0
full_VariableContext_parity_claim=0
MirBuilder_wide_lifecycle_claim=0
```

Next:

```text
296x-1408-PHI-CARRIER-LIFECYCLE-CONSUMER-INVENTORY-001
```

Checks:

```bash
git diff --check
bash tools/checks/current_state_pointer_guard.sh
```

## Stop Line

```text
do_not_start_PHI_consumer_inventory_before_selection=1
do_not_start_general_resolver_before_selection=1
```
