# 296x-1427 POST-TRIM-HELPER-PRODUCER-OWNER-SELECTION-001

Status: closed
Date: 2026-06-20

## Purpose

Select the next lifecycle owner after `TrimRouteInfo::to_carrier_info` is
fixture-guarded as the trim helper producer.

## Selected By

```text
296x-1426-TRIM-HELPER-CARRIER-PRODUCER-PROBE-001
```

## Candidate Owners

```text
A. promoted_body_locals lifecycle inventory
   value: separates owned promoted-name metadata from trim helper payload now
   that the trim producer records promoted_body_locals
   risk: can expand into body-local promotion route design

B. Expand emitter probe to parser/MIR-checkable surface
   value: moves beyond comment-level fixture after producer boundaries are named
   risk: can become generated-program claim or converter rewrite

C. trim route lowering inventory
   value: documents the next layer after trim helper production
   risk: can reopen route lowering semantics too early
```

## Recommended Direction

```text
recommended=A-lite
reason=the trim helper producer also records promoted_body_locals, but this
row deliberately did not claim promoted-name ownership. Inventory that owner
before expanding emitter acceptance or trim route lowering.
```

## Acceptance

```text
next_owner_selected=1
selected_owner_scope_documented=1
non_selected_owners_parked=1
implementation_started=0
backend_behavior_changed=0
full_VariableContext_parity_claim=0
MirBuilder_wide_lifecycle_claim=0
```

## Selection

```text
selected_owner=A-lite
selected_next_task=PROMOTED-BODY-LOCALS-LIFECYCLE-INVENTORY-001
selected_reason=the trim helper producer records promoted_body_locals but does
not claim promoted-name ownership. Inventory producers, merge behavior, and
consumers before emitter expansion or trim route lowering.
```

Parked:

```text
emitter parser/MIR-checkable surface:
  parked until promoted_body_locals owner boundary is explicit

trim route lowering inventory:
  parked; route lowering must not be reopened while promoted-name ownership is
  still only inventory-level
```

## Closeout

```text
next_owner_selected=1
selected_owner_scope_documented=1
non_selected_owners_parked=1
implementation_started=0
backend_behavior_changed=0
full_VariableContext_parity_claim=0
MirBuilder_wide_lifecycle_claim=0
```

Next:

```text
296x-1428-PROMOTED-BODY-LOCALS-LIFECYCLE-INVENTORY-001
```

Checks:

```bash
git diff --check
bash tools/checks/current_state_pointer_guard.sh
```

## Stop Line

```text
do_not_mix_promoted_body_locals_with_emitter_expansion=1
do_not_claim_trim_route_lowering_complete=1
do_not_make_converter_core_policy_owner=1
```
