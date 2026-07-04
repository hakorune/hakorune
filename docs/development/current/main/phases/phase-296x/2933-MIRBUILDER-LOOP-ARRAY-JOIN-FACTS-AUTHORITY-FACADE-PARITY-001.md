---
Status: Landed
Date: 2026-07-05
Scope: LoopArrayJoinFacts authority-facade parity slice.
---

# MIRBUILDER-LOOP-ARRAY-JOIN-FACTS-AUTHORITY-FACADE-PARITY-001

## Decision

Land parity for `try_extract_loop_array_join_facts` as an authority-facade Fact
owner.

```text
selected_owner=loop_array_join_facts.authority_facade
rust_oracle_symbol=try_extract_loop_array_join_facts
input_contract=BackendSafeLoopArrayJoinFactsTokenSnapshotV1
native_edit_authority=lang/src/compiler/lib/loop_array_join_facts.hako
```

This is not a HakoAdopted decision yet.

## Evidence

```text
rust_oracle_fixture=docs/development/current/main/design/fixtures/rust-lifecycle/mirbuilder-loop-array-join-facts-rust-oracle-v0.json
hako_source=lang/src/compiler/lib/loop_array_join_facts.hako
parity_gate=tools/checks/rust_lifecycle_mirbuilder_loop_array_join_facts_parity_gate.sh
oracle_rows=11
parity_status=green
```

## Non-Claims

```text
source_selfhost_claim=0
hako_adopted_decision=0
full_ast_traversal_adopted=0
separator_guard_ast_construction_migrated=0
array_append_ast_construction_migrated=0
cond_profile_construction_migrated=0
scan_condition_observation_migrated=0
loop_increment_extraction_migrated=0
recipe_construction_migrated=0
route_selection_migration=0
backend_lowering_migration=0
mir_mutation_migration=0
id_allocation_migration=0
new_backend_route=0
new_abi=0
```

## Next

```text
MIRBUILDER-LOOP-ARRAY-JOIN-FACTS-HAKOADOPTED-DECISION-001
```
