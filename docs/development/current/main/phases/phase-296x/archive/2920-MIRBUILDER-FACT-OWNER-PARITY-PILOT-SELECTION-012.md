---
Status: Landed
Date: 2026-07-05
Scope: Fact-owner selection for the next MirBuilder authority-facade pilot.
---

# MIRBUILDER-FACT-OWNER-PARITY-PILOT-SELECTION-012

## Decision

Select `try_extract_string_is_integer_facts` for the next authority-facade
parity pilot.

```text
selected_owner=string_is_integer_facts.authority_facade
rust_oracle_symbol=try_extract_string_is_integer_facts
rust_source=src/mir/builder/control_flow/plan/facts/string_is_integer_facts.rs
next_card=MIRBUILDER-STRING-IS-INTEGER-FACTS-AUTHORITY-FACADE-PARITY-001
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

The contract is intentionally narrow:

```text
input:
  condition/body tokens

output:
  accepted/reason DTO for StringIsIntegerFacts
```

This is an authority-facade pilot, not a broad crate split or full AST
traversal adoption.

## Rejected / Held Candidates

```text
bool_predicate_scan_facts:
  held; depends on ScanConditionObservation / CondProfile and predicate payloads

accum_const_loop_facts:
  held; depends on ScanConditionObservation, control-flow helpers, and loop
  increment extraction

loop_array_join_facts:
  held; depends on ScanConditionObservation and a broader payload DTO

loop_true_early_exit_facts:
  held; depends on control-flow counting, loop increment extraction, and AST
  payload return values

build_plan_with_facts_ctx / try_build_outcome:
  held until more Fact-owner facades land
```

## Non-Claims

```text
source_selfhost_claim=0
broad_crate_split=0
full_ast_traversal_adopted=0
backend_lowering_migrated=0
mir_mutation_migrated=0
id_allocation_migrated=0
route_selection_migrated=0
new_backend_route=0
new_abi=0
```

## Next

```text
MIRBUILDER-STRING-IS-INTEGER-FACTS-AUTHORITY-FACADE-PARITY-001
```
