# 296x-1401 VARIABLE-CONTEXT-POST-MUTABLE-DENY-OWNER-SELECTION-001

Status: closed
Date: 2026-06-20

## Purpose

Select the next lifecycle owner after VariableContext simple-map,
immutable-BorrowView, snapshot/restore, and mutable-map Deny closeout are
green.

## Selected By

```text
296x-1400-VARIABLE-CONTEXT-MUTABLE-MAP-DENY-CLOSEOUT-001
```

## Candidate Owners

```text
A. Carrier/PHI lifecycle inventory
   value: names how CarrierInfo::from_variable_map and PHI-sensitive consumers
   use VariableContext map state
   risk: can expand beyond VariableContext if not kept inventory-only

B. HakoLifecycleResolver read-only skeleton
   value: starts consuming proven BindingContext/VariableContext fixture family
   risk: can become general before carrier-sensitive contracts are named
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
selected_next_task=VARIABLE-CONTEXT-CARRIER-PHI-LIFECYCLE-INVENTORY-001
implementation_started=0
```

Reason:

```text
BindingContext and VariableContext local map/borrow/clone/replace boundaries
are now fixture-guarded. Before a general lifecycle resolver can consume them,
carrier-sensitive reads from variable_map must be named so resolver scope does
not silently absorb JoinIR/PHI behavior.
```

Selected scope:

```text
inventory CarrierInfo::from_variable_map
inventory CarrierInfo::with_explicit_carriers
inventory region observer slot classification
document why carrier/PHI remains implementation-disabled
```

Non-selected owner:

```text
B HakoLifecycleResolver read-only skeleton:
  parked until carrier-sensitive map reads have a contract boundary
```

Next:

```text
296x-1402-VARIABLE-CONTEXT-CARRIER-PHI-LIFECYCLE-INVENTORY-001
```

## Stop Line

```text
do_not_start_carrier_PHI_before_selection=1
do_not_start_general_resolver_before_selection=1
```
