# 296x-1429 POST-PROMOTED-BODY-LOCALS-INVENTORY-OWNER-SELECTION-001

Status: closed
Date: 2026-06-20

## Purpose

Select the next lifecycle owner after `CarrierInfo.promoted_body_locals`
inventory names its producers, merge behavior, and join_id-dependent
consumers.

## Selected By

```text
296x-1428-PROMOTED-BODY-LOCALS-LIFECYCLE-INVENTORY-001
```

## Candidate Owners

```text
A. promoted_body_locals producer probe
   value: fixture-guard trim/digitpos producers as name recorders only
   risk: can expand into body-local promotion route design

B. Expand emitter probe to parser/MIR-checkable surface
   value: moves beyond comment-level fixture after denied owners are named
   risk: can become generated-program claim or converter rewrite

C. trim route lowering inventory
   value: documents the next layer after trim helper and promoted-name
   metadata production
   risk: can reopen route lowering semantics too early
```

## Recommended Direction

```text
recommended=A-lite
reason=promoted_body_locals now has a named inventory. The smallest next owner
is a producer-only fixture that records trim/digitpos name producers without
claiming join_id production or route lowering semantics.
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
selected_next_task=PROMOTED-BODY-LOCALS-PRODUCER-PROBE-001
selected_reason=promoted_body_locals inventory names trim/digitpos producers
as name recorders only. The next smallest owner is a producer fixture that
keeps join_id resolution and route lowering out of scope.
```

Parked:

```text
emitter parser/MIR-checkable surface:
  parked until promoted-name producer fixtures are fixed

trim route lowering inventory:
  parked; route lowering remains a later layer after producer-only facts
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
296x-1430-PROMOTED-BODY-LOCALS-PRODUCER-PROBE-001
```

Checks:

```bash
git diff --check
bash tools/checks/current_state_pointer_guard.sh
```

## Stop Line

```text
do_not_mix_promoted_name_producer_with_join_id_resolution=1
do_not_claim_trim_route_lowering_complete=1
do_not_make_converter_core_policy_owner=1
```
