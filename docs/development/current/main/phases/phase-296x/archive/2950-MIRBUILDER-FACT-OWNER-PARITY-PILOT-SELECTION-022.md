---
Status: Landed
Date: 2026-07-05
Scope: Fact-owner selection for the next MirBuilder authority-facade pilot.
---

# MIRBUILDER-FACT-OWNER-PARITY-PILOT-SELECTION-022

## Decision

Select `try_extract_loop_break_step_before_break_subset` for the next
authority-facade parity pilot.

```text
selected_owner=loop_break_step_before_break_facts.authority_facade
rust_oracle_symbol=try_extract_loop_break_step_before_break_subset
rust_source=src/mir/builder/control_flow/plan/loop_break/facts/step_before_break.rs
next_card=MIRBUILDER-LOOP-BREAK-STEP-BEFORE-BREAK-FACTS-AUTHORITY-FACADE-PARITY-001
```

## Why This Candidate

```text
read_only=1
dto_output=1
rust_oracle_json_fixture_possible=1
symbolic_ids_only=1
no_mir_mutation=1
no_backend_lowering=1
no_id_allocation=1
no_new_hako_backend_capability=1
```

This is smaller than full `loop_break_facts`: it owns only the dev/planner
step-before-break subset acceptance surface. The facade may cover planner gate
token, loop var token, control-flow count token, body shape token, carrier var
token, step placement token, and reject reason token. It must not migrate
break-if AST payload extraction, loop increment AST payload extraction,
carrier update AST payload extraction, full loop-break subset dispatch, route
selection, backend lowering, MIR mutation, or ID allocation.

## Held Candidates

```text
loop_break_facts:
  held; dispatches many subset extractors before generic extraction

loop_break_body_local_subset:
  held; constructs synthetic break conditions and handles Freeze policy

loop_cond_break_continue_facts:
  held; multi-variant route facts and nested-loop policy are larger

nested_loop_minimal_facts:
  held; composes condition/step/accum Fact owners and is plan-adjacent
```

## Non-Claims

```text
source_selfhost_claim=0
full_ast_traversal_adopted=0
dev_planner_gate_migrated=0
break_if_ast_payload_migrated=0
loop_increment_ast_payload_migrated=0
carrier_update_ast_payload_migrated=0
loop_break_subset_dispatch_migrated=0
route_selection_migrated=0
backend_lowering_migrated=0
mir_mutation_migrated=0
id_allocation_migrated=0
new_backend_route=0
new_abi=0
```

## Next

```text
MIRBUILDER-LOOP-BREAK-STEP-BEFORE-BREAK-FACTS-AUTHORITY-FACADE-PARITY-001
```
