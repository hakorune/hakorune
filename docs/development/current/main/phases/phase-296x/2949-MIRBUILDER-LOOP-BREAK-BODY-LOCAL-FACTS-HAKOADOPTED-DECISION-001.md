---
Status: Landed
Date: 2026-07-05
Scope: scoped HakoAdopted decision for LoopBreakBodyLocalFacts authority facade.
---

# MIRBUILDER-LOOP-BREAK-BODY-LOCAL-FACTS-HAKOADOPTED-DECISION-001

## Decision

Adopt the authority facade for `LoopBreakBodyLocalFacts`.

```text
decision=HakoAdoptedScoped
adopted_owner=loop_break_body_local_facts.authority_facade
input_contract=BackendSafeLoopBreakBodyLocalFactsTokenSnapshotV1
native_edit_authority=lang/src/compiler/lib/loop_break_body_local_facts.hako
```

This does not adopt full AST traversal, loop-break subset dispatch, break-if
analysis, loop increment extraction, synthetic break-condition construction,
route selection, backend lowering, MIR mutation, or ID allocation.

## Evidence

```text
rust_oracle_fixture=docs/development/current/main/design/fixtures/rust-lifecycle/mirbuilder-loop-break-body-local-facts-rust-oracle-v0.json
hako_source=lang/src/compiler/lib/loop_break_body_local_facts.hako
parity_gate=tools/checks/rust_lifecycle_mirbuilder_loop_break_body_local_facts_parity_gate.sh
adoption_guard=tools/checks/rust_lifecycle_mirbuilder_loop_break_body_local_facts_hako_adoption_decision_guard.sh
oracle_rows=6
parity_status=green
```

## Adopted Semantics

```text
loop_break_body_local_acceptance
condition_len_loop_token
body_local_shape_token
loop_var_token
body_local_var_token
reject_reason_token
```

## Non-Claims

```text
source_selfhost_claim=0
full_ast_traversal_adopted=0
loop_break_subset_dispatch_migrated=0
break_if_analysis_migrated=0
loop_increment_extraction_migrated=0
synthetic_break_condition_construction_migrated=0
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
MIRBUILDER-FACT-OWNER-PARITY-PILOT-SELECTION-022
```
