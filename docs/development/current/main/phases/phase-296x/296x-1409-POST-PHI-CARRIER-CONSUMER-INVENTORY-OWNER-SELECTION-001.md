# 296x-1409 POST-PHI-CARRIER-CONSUMER-INVENTORY-OWNER-SELECTION-001

Status: open
Date: 2026-06-20

## Purpose

Select the next lifecycle owner after PHI carrier lifecycle consumers are
inventoried.

## Selected By

```text
296x-1408-PHI-CARRIER-LIFECYCLE-CONSUMER-INVENTORY-001
```

## Candidate Owners

```text
A. PHI carrier join_id lifecycle probe
   value: names the producer that turns CarrierVar.join_id from None to Some
   risk: can expand into broader header-PHI / JoinIR value-space design

B. CarrierInfo merge_from lifecycle probe
   value: isolates owned CarrierInfo mutation, trim_helper copy, and
          promoted_body_locals dedupe
   risk: can expand into all route-specific carrier promotion

C. HakoLifecycleResolver read-only skeleton
   value: starts reading existing frozen lifecycle fixtures
   risk: still premature if join_id / merge_from owners are not named
```

## Recommended Direction

```text
recommended=A-lite
reason=join_id is the first missing positive owner for promoted carrier
lookup. Without this producer boundary, resolver diagnostics can only report
that snapshots exist, not whether PHI carrier values are lifecycle-complete.
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
do_not_start_join_id_probe_before_selection=1
do_not_start_merge_from_probe_before_selection=1
do_not_start_general_resolver_before_selection=1
```
