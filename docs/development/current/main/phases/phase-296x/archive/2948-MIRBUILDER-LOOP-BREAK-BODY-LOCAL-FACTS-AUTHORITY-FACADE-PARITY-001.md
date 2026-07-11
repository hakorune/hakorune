---
Status: Landed
Date: 2026-07-05
Scope: LoopBreakBodyLocalFacts authority-facade parity slice.
---

# MIRBUILDER-LOOP-BREAK-BODY-LOCAL-FACTS-AUTHORITY-FACADE-PARITY-001

## Decision

Land parity for `try_extract_loop_break_body_local_facts` as an
authority-facade Fact owner.

```text
selected_owner=loop_break_body_local_facts.authority_facade
rust_oracle_symbol=try_extract_loop_break_body_local_facts
input_contract=BackendSafeLoopBreakBodyLocalFactsTokenSnapshotV1
native_edit_authority=lang/src/compiler/lib/loop_break_body_local_facts.hako
```

This is not a HakoAdopted decision yet.

## Evidence

```text
rust_oracle_fixture=docs/development/current/main/design/fixtures/rust-lifecycle/mirbuilder-loop-break-body-local-facts-rust-oracle-v0.json
hako_source=lang/src/compiler/lib/loop_break_body_local_facts.hako
parity_gate=tools/checks/rust_lifecycle_mirbuilder_loop_break_body_local_facts_parity_gate.sh
oracle_rows=6
parity_status=green
```

## Non-Claims

```text
source_selfhost_claim=0
hako_adopted_decision=0
full_ast_traversal_adopted=0
loop_break_subset_dispatch_migrated=0
break_if_analysis_migrated=0
loop_increment_extraction_migrated=0
synthetic_break_condition_construction_migrated=0
route_selection_migration=0
backend_lowering_migration=0
mir_mutation_migration=0
id_allocation_migration=0
new_backend_route=0
new_abi=0
```

## Next

```text
MIRBUILDER-LOOP-BREAK-BODY-LOCAL-FACTS-HAKOADOPTED-DECISION-001
```
