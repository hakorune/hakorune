# 296x-1399 VARIABLE-CONTEXT-POST-SNAPSHOT-RESTORE-OWNER-SELECTION-001

Status: closed
Date: 2026-06-20

## Purpose

Select the next VariableContext lifecycle owner after simple map, immutable
BorrowView, and snapshot/restore ownership fixtures are green.

## Selected By

```text
296x-1398-VARIABLE-CONTEXT-SNAPSHOT-RESTORE-OWNERSHIP-001
```

## Candidate Owners

```text
A. Mutable map API replacement selection
   value: closes variable_map_mut() as Deny or replaces it with explicit APIs
   risk: may require Rust API shape changes

B. Carrier/PHI lifecycle inventory
   value: connects variable_map() BorrowView / snapshot facts to JoinIR carrier
   extraction and PHI-sensitive consumers
   risk: can expand beyond VariableContext if not kept inventory-only

C. HakoLifecycleResolver read-only skeleton
   value: starts consuming proven BindingContext/VariableContext fixture family
   risk: can become general before carrier-sensitive gaps are named
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
selected_next_task=VARIABLE-CONTEXT-MUTABLE-MAP-DENY-CLOSEOUT-001
implementation_started=0
```

Reason:

```text
variable_map_mut() has no external Rust callsites, while changing the API would
be a semantic/code-shape decision. The cleanest next step is to close the
current lifecycle lane with an explicit Deny(ReturnedMutableBorrow) contract
and a no-consumer guard.
```

Selected scope:

```text
validate external variable_map_mut() callsite count remains 0
keep Deny(ReturnedMutableBorrow) as the lifecycle policy
do not change Rust API or generated Hako API
```

Non-selected owners:

```text
B carrier/PHI lifecycle inventory:
  parked until the mutable returned-borrow boundary is closed

C HakoLifecycleResolver read-only skeleton:
  parked until VariableContext unresolved API boundary is closed
```

Next:

```text
296x-1400-VARIABLE-CONTEXT-MUTABLE-MAP-DENY-CLOSEOUT-001
```

## Stop Line

```text
do_not_change_Rust_API_before_selection=1
do_not_start_carrier_PHI_before_selection=1
do_not_start_general_resolver_before_selection=1
```
