# 296x-1443 POST-PROMOTED-CARRIER-IDENTITY-POLICY-OWNER-SELECTION-001

Status: closed
Date: 2026-06-20

## Purpose

Select the next owner after choosing condition-binding identity as the promoted
carrier identity policy.

## Selected By

```text
296x-1442-PROMOTED-CARRIER-IDENTITY-POLICY-DECISION-001
```

## Candidate Owners

```text
A. condition-binding promoted identity proof probe
   value: proves or denies the selected policy over existing facts
   risk: can become resolver rewrite if not kept read-only

B. second lifecycle emitter surface
   value: proves another verified-plan surface can be rendered
   risk: can avoid the selected identity proof

C. rustc lifecycle facts adapter design/probe
   value: begins external facts production
   risk: premature while internal identity proof is unresolved
```

## Recommended Direction

```text
recommended=A-lite
reason=the selected policy still needs a read-only proof probe before any
resolution rewrite or trim route lowering.
```

## Decision

```text
selected_next_task=CONDITION-BINDING-PROMOTED-IDENTITY-PROOF-PROBE-001
selected_scope=read-only proof fixtures over existing ConditionBinding data
selected_reason=condition-binding identity is selected as policy, but it must
be proven as a candidate before any resolution rewrite or trim lowering.
implementation_started=0
```

## Parked Owners

```text
second lifecycle emitter surface:
  parked until identity proof is fixture-guarded.

rustc lifecycle facts adapter design/probe:
  parked until internal identity proof is fixture-guarded.
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
do_not_implement_condition_binding_resolution_in_selection=1
do_not_emit_trim_route_lowering_in_selection=1
do_not_start_rustc_adapter_without_design_card=1
do_not_claim_generated_program_execution=1
```
