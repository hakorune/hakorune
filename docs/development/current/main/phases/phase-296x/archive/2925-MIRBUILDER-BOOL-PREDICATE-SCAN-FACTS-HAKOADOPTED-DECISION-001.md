---
Status: Landed
Date: 2026-07-05
Scope: scoped HakoAdopted decision for BoolPredicateScanFacts authority facade.
---

# MIRBUILDER-BOOL-PREDICATE-SCAN-FACTS-HAKOADOPTED-DECISION-001

## Decision

Adopt the authority facade for `BoolPredicateScanFacts`.

```text
decision=HakoAdoptedScoped
adopted_owner=bool_predicate_scan_facts.authority_facade
input_contract=BackendSafeBoolPredicateScanFactsTokenSnapshotV1
native_edit_authority=lang/src/compiler/lib/bool_predicate_scan_facts.hako
```

This does not adopt CondProfile construction, ScanConditionObservation
construction, full AST traversal, route selection, backend lowering, MIR
mutation, or ID allocation.

## Evidence

```text
rust_oracle_fixture=docs/development/current/main/design/fixtures/rust-lifecycle/mirbuilder-bool-predicate-scan-facts-rust-oracle-v0.json
hako_source=lang/src/compiler/lib/bool_predicate_scan_facts.hako
parity_gate=tools/checks/rust_lifecycle_mirbuilder_bool_predicate_scan_facts_parity_gate.sh
adoption_guard=tools/checks/rust_lifecycle_mirbuilder_bool_predicate_scan_facts_hako_adoption_decision_guard.sh
oracle_rows=7
parity_status=green
```

## Adopted Semantics

```text
bool_predicate_scan_acceptance
loop_var_token
haystack_token
predicate_receiver_token
predicate_method_token
step_lit_token
reject_reason_token
```

## Non-Claims

```text
source_selfhost_claim=0
cond_profile_construction_migrated=0
scan_condition_observation_migrated=0
full_ast_traversal_adopted=0
substring_expression_materialization=0
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
MIRBUILDER-FACT-OWNER-PARITY-PILOT-SELECTION-014
```
