---
Status: Landed
Date: 2026-07-05
Scope: LoopConditionShape backend-safe token snapshot parity slice.
---

# MIRBUILDER-LOOP-CONDITION-SHAPE-PARITY-001

## Decision

Select `try_extract_condition_shape` as the next Fact-owner parity pilot and
land its backend-safe token snapshot reducer.

```text
selected_owner=loop_condition_shape.backend_safe_token_snapshot_reducer
rust_oracle_symbol=try_extract_condition_shape
input_contract=BackendSafeLoopConditionShapeTokenSnapshotV1
native_edit_authority=lang/src/compiler/lib/loop_condition_shape.hako
```

This is not a HakoAdopted decision yet.

## Why This Slice

- follows the adopted `LoopStepShape` token snapshot reducer
- observes only condition-root `ConditionShape` variants and reject reasons
- keeps `CondProfile` migration, observation unification, scan matching, loop
  builder composition, route selection, backend lowering, MIR mutation, and ID
  allocation in Rust

## Evidence

```text
rust_oracle_fixture=docs/development/current/main/design/fixtures/rust-lifecycle/mirbuilder-loop-condition-shape-rust-oracle-v0.json
hako_source=lang/src/compiler/lib/loop_condition_shape.hako
parity_gate=tools/checks/rust_lifecycle_mirbuilder_loop_condition_shape_parity_gate.sh
oracle_rows=15
parity_status=green
```

## Non-Claims

```text
source_selfhost_claim=0
hako_adopted_decision=0
full_ast_traversal_adopted=0
cond_profile_migration=0
condition_observation_unification=0
scan_shape_matching_adopted=0
loop_builder_adopted=0
route_selection_migration=0
backend_lowering_migration=0
mir_mutation_migration=0
id_allocation_migration=0
new_backend_route=0
new_abi=0
```

## Next

```text
MIRBUILDER-LOOP-CONDITION-SHAPE-HAKOADOPTED-DECISION-001
```
