# 296x-1439 POST-TRIM-ROUTE-LOWERING-DECISION-OWNER-SELECTION-001

Status: open
Date: 2026-06-20

## Purpose

Select the next owner after trim route lowering has a read-only decision
fixture and remains denied for executable lowering.

## Selected By

```text
296x-1438-TRIM-ROUTE-LOWERING-DECISION-PROBE-001
```

## Candidate Owners

```text
A. promoted carrier identity / join_id producer design
   value: addresses the current DenyMissingPromotedCarrierIdentity reason
   risk: can reopen PHI join_id design too broadly

B. second lifecycle emitter surface
   value: proves another verified-plan surface can be rendered
   risk: can become converter rewrite without a new verified plan

C. rustc lifecycle facts adapter design/probe
   value: begins external facts production
   risk: design-heavy; requires strict schema/facts boundary
```

## Recommended Direction

```text
recommended=A-design
reason=trim route lowering is now blocked on promoted carrier identity /
join_id proof, not on metadata presence. The next row should decide whether
to design a production join_id producer or keep the boundary denied.
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
