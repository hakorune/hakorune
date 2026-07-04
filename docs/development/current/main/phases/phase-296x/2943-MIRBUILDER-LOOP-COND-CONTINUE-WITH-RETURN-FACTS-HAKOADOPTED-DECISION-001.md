---
Status: Landed
Date: 2026-07-05
Scope: scoped HakoAdopted decision for LoopCondContinueWithReturnFacts authority facade.
---

# MIRBUILDER-LOOP-COND-CONTINUE-WITH-RETURN-FACTS-HAKOADOPTED-DECISION-001

## Decision

Adopt the authority facade for `LoopCondContinueWithReturnFacts`.

```text
decision=HakoAdoptedScoped
adopted_owner=loop_cond_continue_with_return_facts.authority_facade
input_contract=BackendSafeLoopCondContinueWithReturnFactsTokenSnapshotV1
native_edit_authority=lang/src/compiler/lib/loop_cond_continue_with_return_facts.hako
```

This does not adopt full AST traversal, recursive hetero-return traversal,
condition AST payload construction, RecipeBody/RecipeItem construction, route
selection, backend lowering, MIR mutation, or ID allocation.

## Evidence

```text
rust_oracle_fixture=docs/development/current/main/design/fixtures/rust-lifecycle/mirbuilder-loop-cond-continue-with-return-facts-rust-oracle-v0.json
hako_source=lang/src/compiler/lib/loop_cond_continue_with_return_facts.hako
parity_gate=tools/checks/rust_lifecycle_mirbuilder_loop_cond_continue_with_return_facts_parity_gate.sh
adoption_guard=tools/checks/rust_lifecycle_mirbuilder_loop_cond_continue_with_return_facts_hako_adoption_decision_guard.sh
oracle_rows=10
parity_status=green
```

## Adopted Semantics

```text
loop_cond_continue_with_return_acceptance
entry_gate_token
condition_kind_token
control_flow_count_token
continue_return_body_shape_token
reject_reason_token
```

## Non-Claims

```text
source_selfhost_claim=0
full_ast_traversal_adopted=0
recursive_hetero_return_traversal_migrated=0
condition_ast_payload_migrated=0
recipe_body_construction_migrated=0
recipe_item_construction_migrated=0
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
MIRBUILDER-FACT-OWNER-PARITY-PILOT-SELECTION-020
```
