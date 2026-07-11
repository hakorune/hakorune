---
Status: Landed
Date: 2026-07-05
Scope: scoped HakoAdopted decision for LoopTrueEarlyExitFacts authority facade.
---

# MIRBUILDER-LOOP-TRUE-EARLY-EXIT-FACTS-HAKOADOPTED-DECISION-001

## Decision

Adopt the authority facade for `LoopTrueEarlyExitFacts`.

```text
decision=HakoAdoptedScoped
adopted_owner=loop_true_early_exit_facts.authority_facade
input_contract=BackendSafeLoopTrueEarlyExitFactsTokenSnapshotV1
native_edit_authority=lang/src/compiler/lib/loop_true_early_exit_facts.hako
```

This does not adopt control-flow traversal, exit-condition AST construction,
exit-value AST construction, carrier-update AST construction, loop increment
extraction, recipe construction, route selection, backend lowering, MIR
mutation, or ID allocation.

## Evidence

```text
rust_oracle_fixture=docs/development/current/main/design/fixtures/rust-lifecycle/mirbuilder-loop-true-early-exit-facts-rust-oracle-v0.json
hako_source=lang/src/compiler/lib/loop_true_early_exit_facts.hako
parity_gate=tools/checks/rust_lifecycle_mirbuilder_loop_true_early_exit_facts_parity_gate.sh
adoption_guard=tools/checks/rust_lifecycle_mirbuilder_loop_true_early_exit_facts_hako_adoption_decision_guard.sh
oracle_rows=10
parity_status=green
```

## Adopted Semantics

```text
loop_true_early_exit_acceptance
loop_condition_kind_token
exit_kind_token
loop_var_token
carrier_var_token
control_flow_count_token
reject_reason_token
```

## Non-Claims

```text
source_selfhost_claim=0
full_ast_traversal_adopted=0
control_flow_traversal_migrated=0
exit_condition_ast_construction_migrated=0
exit_value_ast_construction_migrated=0
carrier_update_ast_construction_migrated=0
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
MIRBUILDER-FACT-OWNER-PARITY-PILOT-SELECTION-018
```
