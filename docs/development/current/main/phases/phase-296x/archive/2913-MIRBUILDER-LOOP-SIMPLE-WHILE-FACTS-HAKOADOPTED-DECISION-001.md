---
Status: Landed
Date: 2026-07-05
Scope: scoped HakoAdopted decision for LoopSimpleWhileFacts token snapshot reducer.
---

# MIRBUILDER-LOOP-SIMPLE-WHILE-FACTS-HAKOADOPTED-DECISION-001

## Decision

Adopt the backend-safe token snapshot reducer for `LoopSimpleWhileFacts`.

```text
decision=HakoAdoptedScoped
adopted_owner=loop_simple_while_facts.backend_safe_token_snapshot_reducer
input_contract=BackendSafeLoopSimpleWhileFactsTokenSnapshotV1
native_edit_authority=lang/src/compiler/lib/loop_simple_while_facts.hako
```

This does not adopt full AST traversal, step-only body policy, loop increment
plan extraction, recipe construction, loop builder composition, route
selection, backend lowering, MIR mutation, or ID allocation.

## Evidence

```text
rust_oracle_fixture=docs/development/current/main/design/fixtures/rust-lifecycle/mirbuilder-loop-simple-while-facts-rust-oracle-v0.json
hako_source=lang/src/compiler/lib/loop_simple_while_facts.hako
parity_gate=tools/checks/rust_lifecycle_mirbuilder_loop_simple_while_facts_parity_gate.sh
adoption_guard=tools/checks/rust_lifecycle_mirbuilder_loop_simple_while_facts_hako_adoption_decision_guard.sh
oracle_rows=13
parity_status=green
```

## Adopted Semantics

```text
simple_while_acceptance
loop_var_token
increment_token
reject_reason_token
```

## Non-Claims

```text
source_selfhost_claim=0
full_try_extract_loop_simple_while_facts_ast_owner_adopted=0
loop_builder_adopted=0
loop_simple_while_recipe_adopted=0
step_only_body_policy_migrated=0
loop_increment_plan_migrated=0
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
MIRBUILDER-FACT-OWNER-PARITY-PILOT-SELECTION-010
```
