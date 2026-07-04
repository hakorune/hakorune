---
Status: Landed
Date: 2026-07-05
Scope: Fact-owner selection for the next MirBuilder authority-facade pilot.
---

# MIRBUILDER-FACT-OWNER-PARITY-PILOT-SELECTION-023

## Decision

Select `try_extract_step_shape` for the next authority-facade parity pilot.

```text
selected_owner=loop_step_shape.authority_facade
rust_oracle_symbol=try_extract_step_shape
rust_source=src/mir/builder/control_flow/plan/facts/loop_step_shape.rs
next_card=MIRBUILDER-LOOP-STEP-SHAPE-AUTHORITY-FACADE-PARITY-001
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

This is smaller than `loop_condition_shape`: it owns only the last-statement
step-shape acceptance for `var = var + 1` and `var = var - 1`. The facade may
cover statement presence, assignment shape, target/lhs variable equality,
operator token, integer step token, and reject reason token. It must not
migrate AST traversal, loop condition shape extraction, nested-loop minimal
composition, route selection, backend lowering, MIR mutation, or ID allocation.

## Held Candidates

```text
loop_condition_shape:
  held; more condition variants and length-call method tokens

nested_loop_minimal_facts:
  held; composes condition/step/accum Fact owners and is plan-adjacent

loop_scan_with_init / loop_split_scan:
  held; larger scan-specific body analysis

loop_break_facts:
  held; dispatches many subset extractors before generic extraction
```

## Non-Claims

```text
source_selfhost_claim=0
full_ast_traversal_adopted=0
loop_condition_shape_migrated=0
nested_loop_minimal_composition_migrated=0
route_selection_migrated=0
backend_lowering_migrated=0
mir_mutation_migrated=0
id_allocation_migrated=0
new_backend_route=0
new_abi=0
```

## Next

```text
MIRBUILDER-LOOP-STEP-SHAPE-AUTHORITY-FACADE-PARITY-001
```
