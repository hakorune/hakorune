---
Status: Landed
Date: 2026-07-05
Scope: LoopCondReturnInBodyFacts authority-facade parity slice.
---

# MIRBUILDER-LOOP-COND-RETURN-IN-BODY-FACTS-AUTHORITY-FACADE-PARITY-001

## Decision

Land parity for `try_extract_loop_cond_return_in_body_facts` as an
authority-facade Fact owner.

```text
selected_owner=loop_cond_return_in_body_facts.authority_facade
rust_oracle_symbol=try_extract_loop_cond_return_in_body_facts
input_contract=BackendSafeLoopCondReturnInBodyFactsTokenSnapshotV1
native_edit_authority=lang/src/compiler/lib/loop_cond_return_in_body_facts.hako
```

This is not a HakoAdopted decision yet.

## Evidence

```text
rust_oracle_fixture=docs/development/current/main/design/fixtures/rust-lifecycle/mirbuilder-loop-cond-return-in-body-facts-rust-oracle-v0.json
hako_source=lang/src/compiler/lib/loop_cond_return_in_body_facts.hako
parity_gate=tools/checks/rust_lifecycle_mirbuilder_loop_cond_return_in_body_facts_parity_gate.sh
oracle_rows=13
parity_status=green
```

## Non-Claims

```text
source_selfhost_claim=0
hako_adopted_decision=0
full_ast_traversal_adopted=0
shape_specific_ast_traversal_migrated=0
balanced_depth_policy_migrated=0
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
MIRBUILDER-LOOP-COND-RETURN-IN-BODY-FACTS-HAKOADOPTED-DECISION-001
```
