---
Status: Landed
Date: 2026-07-05
Scope: AccumConstLoopFacts authority-facade parity slice.
---

# MIRBUILDER-ACCUM-CONST-LOOP-FACTS-AUTHORITY-FACADE-PARITY-001

## Decision

Land parity for `try_extract_accum_const_loop_facts` as an authority-facade
Fact owner.

```text
selected_owner=accum_const_loop_facts.authority_facade
rust_oracle_symbol=try_extract_accum_const_loop_facts
input_contract=BackendSafeAccumConstLoopFactsTokenSnapshotV1
native_edit_authority=lang/src/compiler/lib/accum_const_loop_facts.hako
```

This is not a HakoAdopted decision yet.

## Evidence

```text
rust_oracle_fixture=docs/development/current/main/design/fixtures/rust-lifecycle/mirbuilder-accum-const-loop-facts-rust-oracle-v0.json
hako_source=lang/src/compiler/lib/accum_const_loop_facts.hako
parity_gate=tools/checks/rust_lifecycle_mirbuilder_accum_const_loop_facts_parity_gate.sh
oracle_rows=9
parity_status=green
```

## Non-Claims

```text
source_selfhost_claim=0
hako_adopted_decision=0
full_ast_traversal_adopted=0
ast_payload_construction_migrated=0
cond_profile_construction_migrated=0
scan_condition_observation_migrated=0
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
MIRBUILDER-ACCUM-CONST-LOOP-FACTS-HAKOADOPTED-DECISION-001
```
