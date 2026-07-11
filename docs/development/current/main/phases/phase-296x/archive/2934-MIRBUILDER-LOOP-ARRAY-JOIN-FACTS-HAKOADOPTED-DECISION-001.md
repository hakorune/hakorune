---
Status: Landed
Date: 2026-07-05
Scope: scoped HakoAdopted decision for LoopArrayJoinFacts authority facade.
---

# MIRBUILDER-LOOP-ARRAY-JOIN-FACTS-HAKOADOPTED-DECISION-001

## Decision

Adopt the authority facade for `LoopArrayJoinFacts`.

```text
decision=HakoAdoptedScoped
adopted_owner=loop_array_join_facts.authority_facade
input_contract=BackendSafeLoopArrayJoinFactsTokenSnapshotV1
native_edit_authority=lang/src/compiler/lib/loop_array_join_facts.hako
```

This does not adopt separator-guard AST construction, array-append AST
construction, CondProfile construction, ScanConditionObservation construction,
loop increment extraction, recipe construction, route selection, backend
lowering, MIR mutation, or ID allocation.

## Evidence

```text
rust_oracle_fixture=docs/development/current/main/design/fixtures/rust-lifecycle/mirbuilder-loop-array-join-facts-rust-oracle-v0.json
hako_source=lang/src/compiler/lib/loop_array_join_facts.hako
parity_gate=tools/checks/rust_lifecycle_mirbuilder_loop_array_join_facts_parity_gate.sh
adoption_guard=tools/checks/rust_lifecycle_mirbuilder_loop_array_join_facts_hako_adoption_decision_guard.sh
oracle_rows=11
parity_status=green
```

## Adopted Semantics

```text
loop_array_join_acceptance
loop_var_token
array_var_token
result_var_token
separator_var_token
reject_reason_token
```

## Non-Claims

```text
source_selfhost_claim=0
full_ast_traversal_adopted=0
separator_guard_ast_construction_migrated=0
array_append_ast_construction_migrated=0
cond_profile_construction_migrated=0
scan_condition_observation_migrated=0
loop_increment_extraction_migrated=0
recipe_construction_migrated=0
route_selection_migrated=0
backend_lowering_migrated=0
mir_mutation_migrated=0
id_allocation_migrated=0
hako_generation=0
runtime_fallback=0
new_backend_route=0
new_abi=0
```

## Next

```text
MIRBUILDER-FACT-OWNER-PARITY-PILOT-SELECTION-017
```
