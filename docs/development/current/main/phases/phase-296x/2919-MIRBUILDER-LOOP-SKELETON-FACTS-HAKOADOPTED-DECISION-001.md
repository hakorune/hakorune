---
Status: Landed
Date: 2026-07-05
Scope: scoped HakoAdopted decision for LoopSkeletonFacts authority facade.
---

# MIRBUILDER-LOOP-SKELETON-FACTS-HAKOADOPTED-DECISION-001

## Decision

Adopt the authority facade for `LoopSkeletonFacts`.

```text
decision=HakoAdoptedScoped
adopted_owner=loop_skeleton_facts.authority_facade
input_contract=BackendSafeLoopSkeletonFactsTokenSnapshotV1
native_edit_authority=lang/src/compiler/lib/loop_skeleton_facts.hako
```

This does not adopt feature-slot inference, full AST traversal, broad crate
splitting, route selection, backend lowering, MIR mutation, or ID allocation.

## Evidence

```text
rust_oracle_fixture=docs/development/current/main/design/fixtures/rust-lifecycle/mirbuilder-loop-skeleton-facts-rust-oracle-v0.json
hako_source=lang/src/compiler/lib/loop_skeleton_facts.hako
parity_gate=tools/checks/rust_lifecycle_mirbuilder_loop_skeleton_facts_parity_gate.sh
adoption_guard=tools/checks/rust_lifecycle_mirbuilder_loop_skeleton_facts_hako_adoption_decision_guard.sh
oracle_rows=3
parity_status=green
```

## Adopted Semantics

```text
skeleton_acceptance
skeleton_kind_loop
empty_feature_slots
```

## Non-Claims

```text
source_selfhost_claim=0
feature_slot_inference=0
full_ast_traversal_adopted=0
broad_crate_split=0
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
MIRBUILDER-FACT-OWNER-PARITY-PILOT-SELECTION-012
```
