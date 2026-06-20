# 296x-1413 POST-MERGE-FROM-LIFECYCLE-OWNER-SELECTION-001

Status: open
Date: 2026-06-20

## Purpose

Select the next lifecycle owner after `CarrierInfo::merge_from` is
fixture-guarded as an owned mutation boundary.

## Selected By

```text
296x-1412-CARRIER-INFO-MERGE-FROM-LIFECYCLE-PROBE-001
```

## Candidate Owners

```text
A. join_id vocabulary retirement/design decision
   value: decide stale/test-only vs real producer before resolver consumes it
   risk: can expand into JoinIR carrier value-space redesign

B. trim_helper lifecycle inventory/probe
   value: isolates route-specific owned metadata cloned by merge_from
   risk: can expand into all trim route semantics

C. HakoLifecycleResolver read-only skeleton
   value: can now consume proven snapshot + merge fixtures while denying
          join_id-dependent paths
   risk: must remain diagnostic-only and not become selection owner
```

## Recommended Direction

```text
recommended=C-lite
reason=CarrierInfo snapshots and merge_from now have focused fixture evidence.
The resolver can start as read-only diagnostics if it explicitly denies
join_id-dependent paths and does not select backend behavior.
```

## Acceptance

```text
next_owner_selected=1
selected_owner_scope_documented=1
non_selected_owners_parked=1
implementation_started=0
resolver_selection_owner=0
converter_emission_added=0
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
do_not_start_resolver_selection_owner_before_selection=1
do_not_mix_trim_helper_probe_with_resolver_skeleton=1
```
