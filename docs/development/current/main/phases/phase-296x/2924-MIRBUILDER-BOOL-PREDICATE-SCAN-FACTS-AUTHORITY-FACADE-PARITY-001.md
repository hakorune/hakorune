---
Status: Landed
Date: 2026-07-05
Scope: BoolPredicateScanFacts authority-facade parity slice.
---

# MIRBUILDER-BOOL-PREDICATE-SCAN-FACTS-AUTHORITY-FACADE-PARITY-001

## Decision

Land parity for `try_extract_bool_predicate_scan_facts` as an
authority-facade Fact owner.

```text
selected_owner=bool_predicate_scan_facts.authority_facade
rust_oracle_symbol=try_extract_bool_predicate_scan_facts
input_contract=BackendSafeBoolPredicateScanFactsTokenSnapshotV1
native_edit_authority=lang/src/compiler/lib/bool_predicate_scan_facts.hako
```

This is not a HakoAdopted decision yet.

## Evidence

```text
rust_oracle_fixture=docs/development/current/main/design/fixtures/rust-lifecycle/mirbuilder-bool-predicate-scan-facts-rust-oracle-v0.json
hako_source=lang/src/compiler/lib/bool_predicate_scan_facts.hako
parity_gate=tools/checks/rust_lifecycle_mirbuilder_bool_predicate_scan_facts_parity_gate.sh
oracle_rows=7
parity_status=green
```

## Non-Claims

```text
source_selfhost_claim=0
hako_adopted_decision=0
cond_profile_construction_migrated=0
scan_condition_observation_migrated=0
full_ast_traversal_adopted=0
route_selection_migration=0
backend_lowering_migration=0
mir_mutation_migration=0
id_allocation_migration=0
new_backend_route=0
new_abi=0
```

## Next

```text
MIRBUILDER-BOOL-PREDICATE-SCAN-FACTS-HAKOADOPTED-DECISION-001
```
