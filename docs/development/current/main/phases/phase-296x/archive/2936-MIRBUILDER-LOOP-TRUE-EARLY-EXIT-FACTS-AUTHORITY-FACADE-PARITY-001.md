---
Status: Landed
Date: 2026-07-05
Scope: LoopTrueEarlyExitFacts authority-facade parity slice.
---

# MIRBUILDER-LOOP-TRUE-EARLY-EXIT-FACTS-AUTHORITY-FACADE-PARITY-001

## Decision

Land parity for `try_extract_loop_true_early_exit_facts` as an authority-facade
Fact owner.

```text
selected_owner=loop_true_early_exit_facts.authority_facade
rust_oracle_symbol=try_extract_loop_true_early_exit_facts
input_contract=BackendSafeLoopTrueEarlyExitFactsTokenSnapshotV1
native_edit_authority=lang/src/compiler/lib/loop_true_early_exit_facts.hako
```

This is not a HakoAdopted decision yet.

## Evidence

```text
rust_oracle_fixture=docs/development/current/main/design/fixtures/rust-lifecycle/mirbuilder-loop-true-early-exit-facts-rust-oracle-v0.json
hako_source=lang/src/compiler/lib/loop_true_early_exit_facts.hako
parity_gate=tools/checks/rust_lifecycle_mirbuilder_loop_true_early_exit_facts_parity_gate.sh
oracle_rows=10
parity_status=green
```

## Non-Claims

```text
source_selfhost_claim=0
hako_adopted_decision=0
full_ast_traversal_adopted=0
control_flow_traversal_migrated=0
exit_condition_ast_construction_migrated=0
exit_value_ast_construction_migrated=0
carrier_update_ast_construction_migrated=0
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
MIRBUILDER-LOOP-TRUE-EARLY-EXIT-FACTS-HAKOADOPTED-DECISION-001
```
