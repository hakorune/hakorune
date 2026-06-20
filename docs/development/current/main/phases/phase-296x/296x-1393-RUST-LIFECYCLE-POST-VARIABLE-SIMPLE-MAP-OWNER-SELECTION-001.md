# 296x-1393 RUST-LIFECYCLE-POST-VARIABLE-SIMPLE-MAP-OWNER-SELECTION-001

Status: open
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

## Stop Line

```text
do_not_start_returned_borrow_before_selection=1
do_not_start_snapshot_restore_before_selection=1
do_not_start_general_resolver_before_selection=1
do_not_claim_full_VariableContext_parity=1
```
