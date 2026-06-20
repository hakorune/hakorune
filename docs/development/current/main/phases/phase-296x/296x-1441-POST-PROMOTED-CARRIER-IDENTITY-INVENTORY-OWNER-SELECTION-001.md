# 296x-1441 POST-PROMOTED-CARRIER-IDENTITY-INVENTORY-OWNER-SELECTION-001

Status: closed
Date: 2026-06-20

## Purpose

Select the next owner after promoted carrier identity / join_id design
inventory.

## Selected By

```text
296x-1440-PROMOTED-CARRIER-IDENTITY-JOIN-ID-DESIGN-INVENTORY-001
```

## Candidate Owners

```text
A. promoted carrier identity decision closeout
   value: choose keep-denied vs implement producer vs condition-binding route
   risk: can become implementation if not kept selection-only

B. second lifecycle emitter surface
   value: proves another verified-plan surface can be rendered
   risk: can avoid the actual trim identity blocker

C. rustc lifecycle facts adapter design/probe
   value: begins external facts production
   risk: premature while internal identity policy is unresolved
```

## Recommended Direction

```text
recommended=A-decision
reason=the inventory names multiple viable identity routes. The next row
should choose one policy before any implementation or new emitter surface.
```

## Decision

```text
selected_next_task=PROMOTED-CARRIER-IDENTITY-POLICY-DECISION-001
selected_policy=condition_binding_identity
selected_scope=decision closeout only; no implementation
selected_reason=CarrierVar.join_id is parked/test vocabulary with no
production producer, while ConditionBinding already owns a JoinIR-local
join_value for condition-only identities. Prefer a future proof probe over
reviving stale join_id vocabulary.
implementation_started=0
```

## Parked Owners

```text
CarrierVar.join_id producer:
  parked; do not implement without a separate producer design and proof.

keep denied forever:
  parked; current executable trim route lowering remains denied until the
  selected condition-binding identity policy is proven.

second lifecycle emitter surface:
  parked until identity policy proof is stable.

rustc lifecycle facts adapter:
  parked until internal identity policy proof is stable.
```

## Acceptance

```text
next_owner_selected=1
selected_owner_scope_documented=1
non_selected_owners_parked=1
implementation_started=0
backend_behavior_changed=0
generated_program_execution_claim=0
```

Checks:

```bash
git diff --check
bash tools/checks/current_state_pointer_guard.sh
```

## Stop Line

```text
do_not_implement_join_id_producer_in_selection=1
do_not_emit_trim_route_lowering_in_selection=1
do_not_start_rustc_adapter_without_design_card=1
do_not_claim_generated_program_execution=1
```
