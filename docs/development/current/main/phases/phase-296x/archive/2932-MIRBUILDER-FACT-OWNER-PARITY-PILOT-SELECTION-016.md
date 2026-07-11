---
Status: Landed
Date: 2026-07-05
Scope: Fact-owner selection for the next MirBuilder authority-facade pilot.
---

# MIRBUILDER-FACT-OWNER-PARITY-PILOT-SELECTION-016

## Decision

Select `try_extract_loop_array_join_facts` for the next authority-facade parity
pilot.

```text
selected_owner=loop_array_join_facts.authority_facade
rust_oracle_symbol=try_extract_loop_array_join_facts
rust_source=src/mir/builder/control_flow/plan/facts/loop_array_join_facts.rs
next_card=MIRBUILDER-LOOP-ARRAY-JOIN-FACTS-AUTHORITY-FACADE-PARITY-001
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

This is the next-smallest ScanConditionObservation-backed Fact owner after
`loop_char_map_facts.authority_facade`. The facade may cover loop var, array
var, result var, separator var, and reject reason tokens. It must not migrate
separator-guard AST construction, array-append AST construction, CondProfile
construction, loop increment extraction, recipe/lowering, or route selection.

## Held Candidates

```text
loop_true_early_exit_facts:
  held; depends on control-flow counting and AST exit/carrier payloads

nested_loop_minimal_facts:
  held; composes condition/step/accum Fact owners and is plan-adjacent

build_plan_with_facts_ctx / try_build_outcome:
  held until more Fact-owner facades land
```

## Non-Claims

```text
source_selfhost_claim=0
full_ast_traversal_adopted=0
separator_guard_ast_construction_migrated=0
array_append_ast_construction_migrated=0
cond_profile_construction_migrated=0
loop_increment_extraction_migrated=0
backend_lowering_migrated=0
mir_mutation_migrated=0
id_allocation_migrated=0
route_selection_migrated=0
new_backend_route=0
new_abi=0
```

## Next

```text
MIRBUILDER-LOOP-ARRAY-JOIN-FACTS-AUTHORITY-FACADE-PARITY-001
```
