---
Status: Landed
Date: 2026-07-05
Scope: LoopCondContinueWithReturnFacts authority-facade parity slice.
---

# MIRBUILDER-LOOP-COND-CONTINUE-WITH-RETURN-FACTS-AUTHORITY-FACADE-PARITY-001

## Decision

Land parity for `try_extract_loop_cond_continue_with_return_facts` as an
authority-facade Fact owner.

```text
selected_owner=loop_cond_continue_with_return_facts.authority_facade
rust_oracle_symbol=try_extract_loop_cond_continue_with_return_facts
input_contract=BackendSafeLoopCondContinueWithReturnFactsTokenSnapshotV1
native_edit_authority=lang/src/compiler/lib/loop_cond_continue_with_return_facts.hako
```

This is not a HakoAdopted decision yet.

## Evidence

```text
rust_oracle_fixture=docs/development/current/main/design/fixtures/rust-lifecycle/mirbuilder-loop-cond-continue-with-return-facts-rust-oracle-v0.json
hako_source=lang/src/compiler/lib/loop_cond_continue_with_return_facts.hako
parity_gate=tools/checks/rust_lifecycle_mirbuilder_loop_cond_continue_with_return_facts_parity_gate.sh
oracle_rows=10
parity_status=green
```

## Non-Claims

```text
source_selfhost_claim=0
hako_adopted_decision=0
full_ast_traversal_adopted=0
recursive_hetero_return_traversal_migrated=0
condition_ast_payload_migrated=0
recipe_body_construction_migrated=0
recipe_item_construction_migrated=0
route_selection_migration=0
backend_lowering_migration=0
mir_mutation_migration=0
id_allocation_migration=0
new_backend_route=0
new_abi=0
```

## Next

```text
MIRBUILDER-LOOP-COND-CONTINUE-WITH-RETURN-FACTS-HAKOADOPTED-DECISION-001
```
