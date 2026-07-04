---
Status: Landed
Date: 2026-07-05
Scope: LoopSimpleWhileFacts backend-safe token snapshot parity slice.
---

# MIRBUILDER-LOOP-SIMPLE-WHILE-FACTS-PARITY-001

## Decision

Select `try_extract_loop_simple_while_facts` as the next Fact-owner parity pilot
and land its backend-safe token snapshot reducer.

```text
selected_owner=loop_simple_while_facts.backend_safe_token_snapshot_reducer
rust_oracle_symbol=try_extract_loop_simple_while_facts
input_contract=BackendSafeLoopSimpleWhileFactsTokenSnapshotV1
native_edit_authority=lang/src/compiler/lib/loop_simple_while_facts.hako
```

This is not a HakoAdopted decision yet.

## Why This Slice

- selected by local + worker inventory as the next smallest meaningful Fact owner
- observes simple `while` facts without `CondProfile`, lowering, mutation, or allocation
- keeps step-only body policy, loop increment plan extraction, recipe
  construction, loop builder composition, route selection, backend lowering,
  MIR mutation, and ID allocation in Rust

## Evidence

```text
rust_oracle_fixture=docs/development/current/main/design/fixtures/rust-lifecycle/mirbuilder-loop-simple-while-facts-rust-oracle-v0.json
hako_source=lang/src/compiler/lib/loop_simple_while_facts.hako
parity_gate=tools/checks/rust_lifecycle_mirbuilder_loop_simple_while_facts_parity_gate.sh
oracle_rows=13
parity_status=green
```

## Non-Claims

```text
source_selfhost_claim=0
hako_adopted_decision=0
full_ast_traversal_adopted=0
loop_builder_adopted=0
loop_simple_while_recipe_adopted=0
step_only_body_policy_migrated=0
loop_increment_plan_migrated=0
route_selection_migration=0
backend_lowering_migration=0
mir_mutation_migration=0
id_allocation_migration=0
new_backend_route=0
new_abi=0
```

## Next

```text
MIRBUILDER-LOOP-SIMPLE-WHILE-FACTS-HAKOADOPTED-DECISION-001
```
