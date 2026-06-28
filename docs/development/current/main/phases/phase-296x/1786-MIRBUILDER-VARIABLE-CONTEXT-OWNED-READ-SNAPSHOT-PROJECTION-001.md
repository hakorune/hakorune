---
Status: Active
Decision: accepted
Date: 2026-06-29
Scope: Materialize the owned read snapshot projection selected by
  MIRBUILDER-VARIABLE-CONTEXT-RETURNED-READ-SNAPSHOT-ROUTE-001.
Related:
  - docs/development/current/main/phases/phase-296x/1785-MIRBUILDER-VARIABLE-CONTEXT-RETURNED-READ-SNAPSHOT-ROUTE-001.md
  - docs/development/current/main/design/fixtures/rust-lifecycle/variable-context-owned-read-snapshot-projection-v0.json
  - tools/checks/rust_lifecycle_variable_context_owned_read_snapshot_projection_guard.sh
---

# MIRBUILDER-VARIABLE-CONTEXT-OWNED-READ-SNAPSHOT-PROJECTION-001

## Goal

Make the selected `OwnedReadSnapshotProjection` executable through the native
VariableContext surface. This projection replaces the raw returned read borrow
with an owned map snapshot and proves source/snapshot alias isolation.

```text
docs_only_closeout = forbidden
code_or_guard_delta_required = 1
```

## Projection

```text
source_method:
  VariableContext::variable_map

rust_return:
  &BTreeMap<String, ValueId>

selected_hako_projection:
  VariableContextNativeApi.snapshot(ctx)

result_transport:
  OrderedMapBox owned clone
```

## Oracle Vectors

```text
nonempty_snapshot:
  source = {a: 10, b: 20}
  snapshot = {a: 10, b: 20}

source_mutation_after_snapshot:
  source after insert c = {a: 10, b: 20, c: 30}
  snapshot remains {a: 10, b: 20}

snapshot_mutation_after_snapshot:
  snapshot after remove a = {b: 20}
  source remains {a: 10, b: 20, c: 30}

deterministic_order:
  keys = [a, b]
```

## Acceptance

```text
input_route = MIRBUILDER-VARIABLE-CONTEXT-RETURNED-READ-SNAPSHOT-ROUTE-001
selected_projection = OwnedReadSnapshotProjection
native_api = VariableContextNativeApi.snapshot
result_transport = OrderedMapBox
owned_clone_required = 1
source_to_snapshot_alias = 0
snapshot_to_source_alias = 0
deterministic_order_preserved = 1
raw_variable_map_alias_emitted = 0
variable_map_mut_selected = 0
candidate_pool_state_after_this_card = BlockedUntilRouteMatrixRerun
next_action = MIRBUILDER-VARIABLE-CONTEXT-ROUTE-MATRIX-RERUN-001
manual_family_selection = 0
runtime_fallback = 0
new_backend_route = 0
new_abi = 0
new_python_semantic_projector = 0
source_selfhost_claim = 0
```

## Non-Claims

```text
BorrowView implementation = 0
returned mutable borrow repair = 0
full VariableContext HakoAdopted = 0
candidate pool eligible = 0
Source Selfhost = 0
runtime fallback = 0
new backend route = 0
new ABI = 0
```

## Closeout

```text
output_contract=rust-lifecycle-variable-context-owned-read-snapshot-projection-v0
selected_projection=OwnedReadSnapshotProjection
native_api=VariableContextNativeApi.snapshot
owned_clone_required=1
source_to_snapshot_alias=0
snapshot_to_source_alias=0
deterministic_order_preserved=1
raw_variable_map_alias_emitted=0
variable_map_mut_selected=0
candidate_pool_state_after_this_card=BlockedUntilRouteMatrixRerun
next_action=MIRBUILDER-VARIABLE-CONTEXT-ROUTE-MATRIX-RERUN-001
manual_family_selection=0
runtime_fallback=0
new_backend_route=0
new_abi=0
new_python_semantic_projector=0
source_selfhost_claim=0
summary=ok
```
