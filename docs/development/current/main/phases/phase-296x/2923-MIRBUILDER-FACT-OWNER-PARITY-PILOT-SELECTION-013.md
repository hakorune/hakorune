---
Status: Landed
Date: 2026-07-05
Scope: Fact-owner selection for the next MirBuilder authority-facade pilot.
---

# MIRBUILDER-FACT-OWNER-PARITY-PILOT-SELECTION-013

## Decision

Select `try_extract_bool_predicate_scan_facts` for the next authority-facade
parity pilot.

```text
selected_owner=bool_predicate_scan_facts.authority_facade
rust_oracle_symbol=try_extract_bool_predicate_scan_facts
rust_source=src/mir/builder/control_flow/plan/facts/bool_predicate_scan_facts.rs
next_card=MIRBUILDER-BOOL-PREDICATE-SCAN-FACTS-AUTHORITY-FACADE-PARITY-001
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

This is the first ScanConditionObservation-backed Fact owner in this run. The
parity fixture must pass precomputed observation tokens and must not migrate
CondProfile construction.

## Held Candidates

```text
accum_const_loop_facts:
  held; depends on control-flow helpers and loop increment extraction

loop_char_map_facts / loop_array_join_facts:
  held; broader payload DTOs after one ScanObservation-backed facade is green

loop_true_early_exit_facts:
  held; depends on control-flow counting and AST payload return values

build_plan_with_facts_ctx / try_build_outcome:
  held until more Fact-owner facades land
```

## Non-Claims

```text
source_selfhost_claim=0
cond_profile_construction_migrated=0
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
MIRBUILDER-BOOL-PREDICATE-SCAN-FACTS-AUTHORITY-FACADE-PARITY-001
```
