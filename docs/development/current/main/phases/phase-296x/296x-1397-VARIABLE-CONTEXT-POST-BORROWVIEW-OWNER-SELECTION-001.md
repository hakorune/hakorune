# 296x-1397 VARIABLE-CONTEXT-POST-BORROWVIEW-OWNER-SELECTION-001

Status: open
Date: 2026-06-20

## Purpose

Select the next VariableContext lifecycle owner after the immutable
`variable_map()` BorrowView probe is green.

## Selected By

```text
296x-1396-VARIABLE-CONTEXT-IMMUTABLE-MAP-BORROWVIEW-PROBE-001
```

## Candidate Owners

```text
A. Mutable map API replacement selection
   value: decides whether variable_map_mut() stays denied or gets explicit APIs
   risk: may require Rust API shape changes

B. Snapshot/restore ownership
   value: covers clone and ReplaceOwned map transfer
   risk: needs explicit old-map cleanup and clone ownership facts

C. Carrier/PHI lifecycle inventory
   value: connects read BorrowView to JoinIR carrier extraction
   risk: can expand beyond VariableContext if opened before ownership transfers

D. HakoLifecycleResolver read-only skeleton
   value: starts consuming proven BindingContext/VariableContext fixtures
   risk: can become general too early if remaining VariableContext gaps are unnamed
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
do_not_change_Rust_API_before_selection=1
do_not_start_snapshot_restore_before_selection=1
do_not_start_carrier_PHI_before_selection=1
do_not_start_general_resolver_before_selection=1
```
