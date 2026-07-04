---
Status: Landed
Date: 2026-07-05
Scope: scoped HakoAdopted decision for LoopContinueOnlyFacts token snapshot reducer.
---

# MIRBUILDER-LOOP-CONTINUE-ONLY-FACTS-HAKOADOPTED-DECISION-001

## Decision

Adopt the backend-safe token snapshot reducer for `LoopContinueOnlyFacts`.

```text
decision=HakoAdoptedScoped
adopted_owner=loop_continue_only_facts.backend_safe_token_snapshot_reducer
input_contract=BackendSafeLoopContinueOnlyFactsTokenSnapshotV1
native_edit_authority=lang/src/compiler/lib/loop_continue_only_facts.hako
```

This does not adopt full AST traversal, control-flow counting, carrier update
map extraction, loop increment plan extraction, recipe construction, loop
builder composition, route selection, backend lowering, MIR mutation, or ID
allocation.

## Evidence

```text
rust_oracle_fixture=docs/development/current/main/design/fixtures/rust-lifecycle/mirbuilder-loop-continue-only-facts-rust-oracle-v0.json
hako_source=lang/src/compiler/lib/loop_continue_only_facts.hako
parity_gate=tools/checks/rust_lifecycle_mirbuilder_loop_continue_only_facts_parity_gate.sh
adoption_guard=tools/checks/rust_lifecycle_mirbuilder_loop_continue_only_facts_hako_adoption_decision_guard.sh
oracle_rows=11
parity_status=green
```

## Adopted Semantics

```text
continue_only_acceptance
loop_var_token
continue_if_index_token
carrier_count_token
increment_token
reject_reason_token
```

## Non-Claims

```text
source_selfhost_claim=0
full_try_extract_loop_continue_only_facts_ast_owner_adopted=0
control_flow_counting_migrated=0
carrier_update_map_migrated=0
loop_increment_plan_migrated=0
loop_builder_adopted=0
loop_continue_only_recipe_adopted=0
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
MIRBUILDER-FACT-OWNER-PARITY-PILOT-SELECTION-011
```
