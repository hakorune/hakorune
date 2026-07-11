---
Status: Landed
Date: 2026-07-05
Scope: LoopSkeletonFacts authority-facade parity slice.
---

# MIRBUILDER-LOOP-SKELETON-FACTS-AUTHORITY-FACADE-PARITY-001

## Decision

Land parity for `try_extract_loop_skeleton_facts` as the next authority-facade
Fact owner.

```text
selected_owner=loop_skeleton_facts.authority_facade
rust_oracle_symbol=try_extract_loop_skeleton_facts
input_contract=BackendSafeLoopSkeletonFactsTokenSnapshotV1
native_edit_authority=lang/src/compiler/lib/loop_skeleton_facts.hako
```

This is not a HakoAdopted decision yet.

## Evidence

```text
rust_oracle_fixture=docs/development/current/main/design/fixtures/rust-lifecycle/mirbuilder-loop-skeleton-facts-rust-oracle-v0.json
hako_source=lang/src/compiler/lib/loop_skeleton_facts.hako
parity_gate=tools/checks/rust_lifecycle_mirbuilder_loop_skeleton_facts_parity_gate.sh
oracle_rows=3
parity_status=green
```

## Adopted Contract Candidate

```text
accepted=1
kind=Loop
feature_slots=0
```

## Non-Claims

```text
source_selfhost_claim=0
hako_adopted_decision=0
broad_crate_split=0
feature_slot_inference=0
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
MIRBUILDER-LOOP-SKELETON-FACTS-HAKOADOPTED-DECISION-001
```
