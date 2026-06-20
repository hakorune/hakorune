# 296x-1411 POST-JOIN-ID-PRODUCER-INVENTORY-OWNER-SELECTION-001

Status: open
Date: 2026-06-20

## Purpose

Select the next lifecycle owner after `CarrierVar.join_id` is inventoried as
having no production `Some(ValueId)` producer.

## Selected By

```text
296x-1410-PHI-CARRIER-JOIN-ID-LIFECYCLE-PRODUCER-INVENTORY-001
```

## Candidate Owners

```text
A. join_id vocabulary retirement/design decision
   value: decide whether join_id is stale/test-only vocabulary or should gain
          a real production producer
   risk: can become broader JoinIR carrier value-space redesign

B. CarrierInfo merge_from lifecycle probe
   value: proceed with a live owned mutation boundary that does exist today
   risk: leaves join_id unresolved but parked

C. HakoLifecycleResolver read-only skeleton
   value: consume proven fixtures only
   risk: must explicitly deny/ignore join_id-dependent paths
```

## Recommended Direction

```text
recommended=B-lite
reason=merge_from is an actual production mutation boundary, while join_id
needs a separate design decision before implementation. Resolver work should
still wait until at least merge_from ownership is named.
```

## Acceptance

```text
next_owner_selected=1
selected_owner_scope_documented=1
non_selected_owners_parked=1
implementation_started=0
general_resolver_started=0
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
do_not_delete_or_implement_join_id_in_this_selection_row=1
do_not_start_general_resolver_before_selection=1
do_not_merge_join_id_design_with_merge_from_probe=1
```
