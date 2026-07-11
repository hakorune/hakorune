---
Status: Landed
Date: 2026-07-05
Scope: LoopContinueOnlyFacts backend-safe token snapshot parity slice.
---

# MIRBUILDER-LOOP-CONTINUE-ONLY-FACTS-PARITY-001

## Decision

Select `try_extract_loop_continue_only_facts` as the next Fact-owner parity
pilot and land its backend-safe token snapshot reducer.

```text
selected_owner=loop_continue_only_facts.backend_safe_token_snapshot_reducer
rust_oracle_symbol=try_extract_loop_continue_only_facts
input_contract=BackendSafeLoopContinueOnlyFactsTokenSnapshotV1
native_edit_authority=lang/src/compiler/lib/loop_continue_only_facts.hako
```

This is not a HakoAdopted decision yet.

## Why This Slice

- selected as the next meaningful Fact-owner after `LoopSimpleWhileFacts`
- observes single-continue loop facts by token snapshot only
- keeps control-flow counting implementation, carrier update map extraction,
  loop increment plan extraction, recipe construction, loop builder
  composition, route selection, backend lowering, MIR mutation, and ID
  allocation in Rust

## Evidence

```text
rust_oracle_fixture=docs/development/current/main/design/fixtures/rust-lifecycle/mirbuilder-loop-continue-only-facts-rust-oracle-v0.json
hako_source=lang/src/compiler/lib/loop_continue_only_facts.hako
parity_gate=tools/checks/rust_lifecycle_mirbuilder_loop_continue_only_facts_parity_gate.sh
oracle_rows=11
parity_status=green
```

## Non-Claims

```text
source_selfhost_claim=0
hako_adopted_decision=0
full_ast_traversal_adopted=0
control_flow_counting_migrated=0
carrier_update_map_migrated=0
loop_increment_plan_migrated=0
loop_continue_only_recipe_adopted=0
loop_builder_adopted=0
route_selection_migration=0
backend_lowering_migration=0
mir_mutation_migration=0
id_allocation_migration=0
new_backend_route=0
new_abi=0
```

## Next

```text
MIRBUILDER-LOOP-CONTINUE-ONLY-FACTS-HAKOADOPTED-DECISION-001
```
