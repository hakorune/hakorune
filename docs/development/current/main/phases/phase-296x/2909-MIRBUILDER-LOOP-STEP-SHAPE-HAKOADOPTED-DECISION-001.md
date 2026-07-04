---
Status: Landed
Date: 2026-07-05
Scope: scoped HakoAdopted decision for LoopStepShape token snapshot reducer.
---

# MIRBUILDER-LOOP-STEP-SHAPE-HAKOADOPTED-DECISION-001

## Decision

Adopt the backend-safe token snapshot reducer for `LoopStepShape`.

```text
decision=HakoAdoptedScoped
adopted_owner=loop_step_shape.backend_safe_token_snapshot_reducer
input_contract=BackendSafeLoopStepShapeTokenSnapshotV1
native_edit_authority=lang/src/compiler/lib/loop_step_shape.hako
```

This does not adopt full AST traversal, loop builder composition,
`ConditionShape`, `CondProfile`, scan shape matching, route selection, backend
lowering, MIR mutation, or ID allocation.

## Evidence

```text
rust_oracle_fixture=docs/development/current/main/design/fixtures/rust-lifecycle/mirbuilder-loop-step-shape-rust-oracle-v0.json
hako_source=lang/src/compiler/lib/loop_step_shape.hako
parity_gate=tools/checks/rust_lifecycle_mirbuilder_loop_step_shape_parity_gate.sh
adoption_guard=tools/checks/rust_lifecycle_mirbuilder_loop_step_shape_hako_adoption_decision_guard.sh
oracle_rows=11
parity_status=green
```

## Adopted Semantics

```text
last_stmt_assignment_shape_acceptance
step_shape_kind_token
step_var_token
step_delta_token
reject_reason_token
```

## Non-Claims

```text
source_selfhost_claim=0
full_try_extract_step_shape_ast_owner_adopted=0
loop_builder_adopted=0
condition_shape_adopted=0
cond_profile_migration=0
scan_shape_matching_adopted=0
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
MIRBUILDER-FACT-OWNER-PARITY-PILOT-SELECTION-008
```
