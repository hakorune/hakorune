---
Status: Landed
Date: 2026-07-05
Scope: Fact-owner selection for the next MirBuilder authority-facade pilot.
---

# MIRBUILDER-FACT-OWNER-PARITY-PILOT-SELECTION-011

## Decision

Select `try_extract_loop_skeleton_facts` for the next authority-facade parity
pilot.

```text
selected_owner=loop_skeleton_facts.authority_facade
rust_oracle_symbol=try_extract_loop_skeleton_facts
rust_source=src/mir/builder/control_flow/plan/facts/skeleton_facts.rs
next_card=MIRBUILDER-LOOP-SKELETON-FACTS-AUTHORITY-FACADE-PARITY-001
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
  SkeletonFacts {
    kind = Loop
    feature_slots = []
  }
```

This is an authority-facade pilot, not a broad crate split.

## Rejected / Held Candidates

```text
try_extract_loop_feature_facts:
  already HakoAdoptedScoped for backend-safe token snapshot reducer
  full AST body traversal remains consultation-gated

try_extract_loop_true_early_exit_facts:
  held; depends on control-flow counting, loop increment extraction, and AST
  payload return values

try_extract_nested_loop_minimal_facts:
  held; composes multiple Fact owners and is plan-adjacent

build_plan_with_facts_ctx / try_build_outcome:
  held until the next Fact-owner facade lands
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
MIRBUILDER-LOOP-SKELETON-FACTS-AUTHORITY-FACADE-PARITY-001
```
