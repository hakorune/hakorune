---
Status: Landed
Date: 2026-07-05
Scope: scoped HakoAdopted decision for AccumConstLoopFacts authority facade.
---

# MIRBUILDER-ACCUM-CONST-LOOP-FACTS-HAKOADOPTED-DECISION-001

## Decision

Adopt the authority facade for `AccumConstLoopFacts`.

```text
decision=HakoAdoptedScoped
adopted_owner=accum_const_loop_facts.authority_facade
input_contract=BackendSafeAccumConstLoopFactsTokenSnapshotV1
native_edit_authority=lang/src/compiler/lib/accum_const_loop_facts.hako
```

This does not adopt AST payload construction, CondProfile construction,
ScanConditionObservation construction, loop increment extraction, recipe
construction, route selection, backend lowering, MIR mutation, or ID allocation.

## Evidence

```text
rust_oracle_fixture=docs/development/current/main/design/fixtures/rust-lifecycle/mirbuilder-accum-const-loop-facts-rust-oracle-v0.json
hako_source=lang/src/compiler/lib/accum_const_loop_facts.hako
parity_gate=tools/checks/rust_lifecycle_mirbuilder_accum_const_loop_facts_parity_gate.sh
adoption_guard=tools/checks/rust_lifecycle_mirbuilder_accum_const_loop_facts_hako_adoption_decision_guard.sh
oracle_rows=9
parity_status=green
```

## Adopted Semantics

```text
accum_const_loop_acceptance
loop_var_token
accumulator_var_token
accumulator_literal_token
reject_reason_token
```

## Non-Claims

```text
source_selfhost_claim=0
full_ast_traversal_adopted=0
ast_payload_construction_migrated=0
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
MIRBUILDER-FACT-OWNER-PARITY-PILOT-SELECTION-015
```
