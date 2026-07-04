---
Status: Landed
Date: 2026-07-05
Scope: scoped HakoAdopted decision for IfPhiJoinFacts authority facade.
---

# MIRBUILDER-IF-PHI-JOIN-FACTS-HAKOADOPTED-DECISION-001

## Decision

Adopt the authority facade for `IfPhiJoinFacts`.

```text
decision=HakoAdoptedScoped
adopted_owner=if_phi_join_facts.authority_facade
input_contract=BackendSafeIfPhiJoinFactsTokenSnapshotV1
native_edit_authority=lang/src/compiler/lib/if_phi_join_facts.hako
```

This does not adopt full AST traversal, `extract_loop_with_if_phi_parts`,
condition AST payload construction, then/else update AST payload construction,
loop increment extraction, recipe construction, route selection, backend
lowering, MIR mutation, or ID allocation.

## Evidence

```text
rust_oracle_fixture=docs/development/current/main/design/fixtures/rust-lifecycle/mirbuilder-if-phi-join-facts-rust-oracle-v0.json
hako_source=lang/src/compiler/lib/if_phi_join_facts.hako
parity_gate=tools/checks/rust_lifecycle_mirbuilder_if_phi_join_facts_parity_gate.sh
adoption_guard=tools/checks/rust_lifecycle_mirbuilder_if_phi_join_facts_hako_adoption_decision_guard.sh
oracle_rows=11
parity_status=green
```

## Adopted Semantics

```text
if_phi_join_acceptance
loop_var_token
carrier_var_token
then_update_kind_token
else_update_kind_token
reject_reason_token
```

## Non-Claims

```text
source_selfhost_claim=0
full_ast_traversal_adopted=0
if_phi_parts_extractor_migrated=0
condition_ast_payload_migrated=0
then_update_ast_payload_migrated=0
else_update_ast_payload_migrated=0
loop_increment_extraction_migrated=0
recipe_construction_migrated=0
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
MIRBUILDER-FACT-OWNER-PARITY-PILOT-SELECTION-019
```
