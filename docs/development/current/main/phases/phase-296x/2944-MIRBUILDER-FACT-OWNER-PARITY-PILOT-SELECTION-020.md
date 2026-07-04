---
Status: Landed
Date: 2026-07-05
Scope: Fact-owner selection for the next MirBuilder authority-facade pilot.
---

# MIRBUILDER-FACT-OWNER-PARITY-PILOT-SELECTION-020

## Decision

Select `try_extract_loop_cond_return_in_body_facts` for the next
authority-facade parity pilot.

```text
selected_owner=loop_cond_return_in_body_facts.authority_facade
rust_oracle_symbol=try_extract_loop_cond_return_in_body_facts
rust_source=src/mir/builder/control_flow/facts/loop_cond_return_in_body.rs
next_card=MIRBUILDER-LOOP-COND-RETURN-IN-BODY-FACTS-AUTHORITY-FACADE-PARITY-001
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

This is the next-smallest remaining standalone Fact owner after
`loop_cond_continue_with_return_facts.authority_facade`. The facade may cover
condition support, control-flow counts, matched return-in-body shape tokens,
and reject reasons. It must not migrate shape-specific AST traversal,
balanced-depth policy evaluation, RecipeBody/RecipeItem construction, route
selection, backend lowering, MIR mutation, or ID allocation.

## Held Candidates

```text
loop_cond_break_continue_facts:
  held; multi-variant route facts and nested-loop policy are larger

loop_break_facts:
  held; dispatches many subset extractors before generic extraction

nested_loop_minimal_facts:
  held; composes condition/step/accum Fact owners and is plan-adjacent

build_plan_with_facts_ctx / try_build_outcome:
  held until more standalone Fact-owner facades land
```

## Non-Claims

```text
source_selfhost_claim=0
full_ast_traversal_adopted=0
shape_specific_ast_traversal_migrated=0
balanced_depth_policy_migrated=0
condition_ast_payload_migrated=0
recipe_body_construction_migrated=0
recipe_item_construction_migrated=0
route_selection_migrated=0
backend_lowering_migrated=0
mir_mutation_migrated=0
id_allocation_migrated=0
new_backend_route=0
new_abi=0
```

## Next

```text
MIRBUILDER-LOOP-COND-RETURN-IN-BODY-FACTS-AUTHORITY-FACADE-PARITY-001
```
