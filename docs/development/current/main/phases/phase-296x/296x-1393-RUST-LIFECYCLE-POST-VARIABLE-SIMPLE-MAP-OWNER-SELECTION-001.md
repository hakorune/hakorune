# 296x-1393 RUST-LIFECYCLE-POST-VARIABLE-SIMPLE-MAP-OWNER-SELECTION-001

Status: closed
Date: 2026-06-20

## Purpose

Select the next lifecycle owner after BindingContext and VariableContext
simple-map pilots are both green.

## Selected By

```text
296x-1392-VARIABLE-CONTEXT-LIFECYCLE-SIMPLE-MAP-ORACLE-PARITY-001
```

## Candidate Owners

```text
A. VariableContext returned borrow boundary
   value: addresses variable_map() and variable_map_mut()
   risk: returned mutable map borrow may require API redesign or Deny policy

B. VariableContext snapshot/restore ownership
   value: addresses clone/ReplaceOwned map transfer
   risk: needs explicit map clone and old-map cleanup policy

C. Carrier/PHI consumer lifecycle inventory
   value: connects VariableContext to JoinIR carrier-sensitive use
   risk: can expand into compiler-wide lifecycle parity too quickly

D. HakoLifecycleResolver read-only skeleton
   value: starts consuming the now-proven facts/plans
   risk: can become a general resolver before the hard VariableContext gaps are named
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
selected_next_task=VARIABLE-CONTEXT-RETURNED-BORROW-BOUNDARY-INVENTORY-001
implementation_started=0
```

Reason:

```text
VariableContext simple-map parity is green, but full VariableContext cannot be
claimed while variable_map() and variable_map_mut() expose BTreeMap borrows
outside the method boundary.
```

Selected scope:

```text
document returned immutable and mutable map borrow boundaries
classify current consumers by read-only / mutable / carrier-sensitive use
select initial lifecycle policy for returned map borrows
```

Initial policy:

```text
variable_map():
  inventory read-only consumers and carrier-sensitive consumers
  allow only owner-carrying read BorrowView candidates later

variable_map_mut():
  deny as ReturnedMutableBorrow until an API-specific replacement plan exists
```

Non-selected owners:

```text
B snapshot/restore ownership:
  parked until returned map borrow boundary is named

C carrier/PHI consumer lifecycle inventory:
  parked until returned map borrow consumers are classified

D HakoLifecycleResolver read-only skeleton:
  parked until hard VariableContext lifecycle gaps are documented
```

## Stop Line

```text
do_not_start_returned_borrow_before_selection=1
do_not_start_snapshot_restore_before_selection=1
do_not_start_general_resolver_before_selection=1
do_not_claim_full_VariableContext_parity=1
```
