---
Status: Landed
Date: 2026-07-05
Scope: scoped HakoAdopted decision for LoopBreakStepBeforeBreakFacts authority facade.
---

# MIRBUILDER-LOOP-BREAK-STEP-BEFORE-BREAK-FACTS-HAKOADOPTED-DECISION-001

## Decision

Adopt the authority facade for `LoopBreakStepBeforeBreakFacts`.

```text
decision=HakoAdoptedScoped
adopted_owner=loop_break_step_before_break_facts.authority_facade
input_contract=BackendSafeLoopBreakStepBeforeBreakFactsTokenSnapshotV1
native_edit_authority=lang/src/compiler/lib/loop_break_step_before_break_facts.hako
```

This does not adopt full AST traversal, dev/planner environment reads,
break-if AST payload extraction, loop increment AST payload extraction,
carrier update AST payload extraction, full loop-break subset dispatch, route
selection, backend lowering, MIR mutation, or ID allocation.

## Evidence

```text
rust_oracle_fixture=docs/development/current/main/design/fixtures/rust-lifecycle/mirbuilder-loop-break-step-before-break-facts-rust-oracle-v0.json
hako_source=lang/src/compiler/lib/loop_break_step_before_break_facts.hako
parity_gate=tools/checks/rust_lifecycle_mirbuilder_loop_break_step_before_break_facts_parity_gate.sh
adoption_guard=tools/checks/rust_lifecycle_mirbuilder_loop_break_step_before_break_facts_hako_adoption_decision_guard.sh
oracle_rows=9
parity_status=green
```

## Adopted Semantics

```text
loop_break_step_before_break_acceptance
planner_gate_token
condition_plan_subset_token
control_flow_count_token
body_shape_token
loop_var_token
carrier_var_token
step_placement_token
reject_reason_token
```

## Non-Claims

```text
source_selfhost_claim=0
full_ast_traversal_adopted=0
dev_planner_gate_migrated=0
break_if_ast_payload_migrated=0
loop_increment_ast_payload_migrated=0
carrier_update_ast_payload_migrated=0
loop_break_subset_dispatch_migrated=0
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
MIRBUILDER-FACT-OWNER-PARITY-PILOT-SELECTION-023
```
