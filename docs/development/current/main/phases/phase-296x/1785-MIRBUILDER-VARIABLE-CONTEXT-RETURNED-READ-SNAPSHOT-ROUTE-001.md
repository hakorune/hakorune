---
Status: Active
Decision: accepted
Date: 2026-06-29
Scope: Select an owned read snapshot route for `VariableContext::variable_map`
  instead of raw returned-borrow transport.
Related:
  - docs/development/current/main/phases/phase-296x/1784-SOURCE-SELFHOST-POST-VARIABLE-CONTEXT-SURFACE-RESOLUTION-001.md
  - docs/development/current/main/phases/phase-296x/1783-VARIABLE-CONTEXT-NATIVE-SURFACE-HAKO-ADOPTION-DECISION-001.md
  - docs/development/current/main/design/fixtures/rust-lifecycle/returned-read-borrow-read-view-decision-v0.json
  - docs/development/current/main/design/fixtures/rust-lifecycle/variable-context-returned-read-snapshot-route-v0.json
  - tools/checks/rust_lifecycle_variable_context_returned_read_snapshot_route_guard.sh
---

# MIRBUILDER-VARIABLE-CONTEXT-RETURNED-READ-SNAPSHOT-ROUTE-001

## Goal

Repair the source-selfhost stop line by selecting the replacement route for
`VariableContext::variable_map()`. The Rust method returns a shared map borrow,
but the selected Hako route must not expose a naked alias. It uses
`OwnedReadSnapshotProjection` as the formal replacement.

```text
docs_only_closeout = forbidden
code_or_guard_delta_required = 1
```

## Route Decision

```text
owner_kind:
  VariableContextReturnedBorrowRepairDecision

selected_repair:
  OwnedReadSnapshotProjection

raw_alias_transport:
  Denied

candidate_pool_state_after_this_card:
  BlockedUntilRouteMatrixRerun
```

## Selected Surface

```text
variable_map:
  rust_return = &BTreeMap<String, ValueId>
  selected = false
  deny_reason = ReturnedReadBorrow
  replacement = VariableMapOwnedReadSnapshot

variable_map_mut:
  rust_return = &mut BTreeMap<String, ValueId>
  selected = false
  deny_reason = ReturnedMutableBorrow
  replacement = ExplicitMutationOperationsOnly

snapshot:
  selected = true
  hako_operation = CloneOwnedMap

restore:
  selected = true
  hako_operation = ReplaceOwnedMap
```

## Acceptance

```text
input_candidate_pool_state = Blocked
input_reason_token = NoRemainingMachineDerivedNativeSurfaceCandidate
input_parked_reason = ReturnedReadBorrow
replacement_policy = OwnedReadSnapshotProjection
variable_map_raw_alias_selected = 0
variable_map_replacement = VariableMapOwnedReadSnapshot
variable_map_mut_selected = 0
variable_map_mut_deny_reason = ReturnedMutableBorrow
snapshot_clone_owned_selected = 1
restore_replace_owned_selected = 1
candidate_pool_state_after_this_card = BlockedUntilRouteMatrixRerun
next_action = MIRBUILDER-VARIABLE-CONTEXT-OWNED-READ-SNAPSHOT-PROJECTION-001
manual_family_selection = 0
runtime_fallback = 0
new_backend_route = 0
new_abi = 0
new_python_semantic_projector = 0
source_selfhost_claim = 0
```

## Non-Claims

```text
owned read snapshot artifact = 0
candidate pool eligible = 0
BorrowView implementation = 0
returned mutable borrow repair = 0
full VariableContext HakoAdopted = 0
Source Selfhost = 0
runtime fallback = 0
new backend route = 0
new ABI = 0
```

## Closeout

```text
output_contract=rust-lifecycle-variable-context-returned-read-snapshot-route-v0
selected_repair=OwnedReadSnapshotProjection
variable_map_raw_alias_selected=0
variable_map_replacement=VariableMapOwnedReadSnapshot
variable_map_mut_selected=0
variable_map_mut_deny_reason=ReturnedMutableBorrow
candidate_pool_state_after_this_card=BlockedUntilRouteMatrixRerun
next_action=MIRBUILDER-VARIABLE-CONTEXT-OWNED-READ-SNAPSHOT-PROJECTION-001
manual_family_selection=0
runtime_fallback=0
new_backend_route=0
new_abi=0
new_python_semantic_projector=0
source_selfhost_claim=0
summary=ok
```
