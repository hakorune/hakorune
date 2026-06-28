---
Status: Active
Decision: accepted
Date: 2026-06-29
Scope: Select explicit mutation APIs as the replacement policy for
  VariableContext returned mutable borrow.
Related:
  - docs/development/current/main/phases/phase-296x/1789-SOURCE-SELFHOST-POST-VARIABLE-CONTEXT-OWNED-SNAPSHOT-RESOLUTION-001.md
  - docs/development/current/main/design/fixtures/rust-lifecycle/variable-context-explicit-mutation-surface-selection-v0.json
  - tools/checks/rust_lifecycle_variable_context_explicit_mutation_surface_selection_guard.sh
---

# MIRBUILDER-VARIABLE-CONTEXT-EXPLICIT-MUTATION-SURFACE-SELECTION-001

## Goal

Resolve the returned mutable borrow policy by selecting explicit mutation APIs
instead of transporting `variable_map_mut()` as a naked mutable alias.

```text
docs_only_closeout = forbidden
code_or_guard_delta_required = 1
```

## Decision

```text
owner_kind:
  VariableContextReturnedMutableBorrowPolicyDecision

selected_policy:
  ExplicitMutationApiOnly

variable_map_mut:
  rust_return = &mut BTreeMap<String, ValueId>
  selected = false
  deny_reason = ReturnedMutableBorrow
  replacement = ExplicitMutationOperations
```

## Selected Mutation Operations

```text
insert:
  operation = MapSet
  mutates = variable_map

remove:
  operation = MapRemove
  mutates = variable_map

restore:
  operation = ReplaceOwnedMap
  mutates = variable_map

replace_owned_map:
  operation = ReplaceOwnedMap
  mutates = variable_map
```

## Explicit Denials

```text
raw_variable_map_mut_alias
returned_mutable_borrow_escape
borrow_lifetime_inference
implicit_commit_discard_mut_lease
```

## Acceptance

```text
last_adopted_surface = VariableContextNativeSurfaceOwnedReadSnapshotV1
remaining_boundary = VariableContext_mutable_returned_borrow
reason_token = ReturnedMutableBorrowPolicyRequired
owner_kind = VariableContextReturnedMutableBorrowPolicyDecision
selected_policy = ExplicitMutationApiOnly
variable_map_mut_selected = false
variable_map_mut_deny_reason = ReturnedMutableBorrow
selected_mutation_ops = insert, remove, restore, replace_owned_map
restore_operation = ReplaceOwnedMap
replace_owned_map_operation = ReplaceOwnedMap
candidate_pool_state_after_this_card = BlockedUntilExplicitMutationProjection
next_action = MIRBUILDER-VARIABLE-CONTEXT-EXPLICIT-MUTATION-API-PROJECTION-001
manual_family_selection = 0
full_variable_context_claim = 0
returned_mutable_borrow_selected = 0
mut_lease_selected = 0
runtime_fallback = 0
new_backend_route = 0
new_abi = 0
new_python_semantic_projector = 0
source_selfhost_claim = 0
```

## Non-Claims

```text
explicit mutation projection = 0
replace_owned_map native API = 0
MutLease = 0
full VariableContext = 0
Source Selfhost = 0
runtime fallback = 0
new backend route = 0
new ABI = 0
```

## Closeout

```text
output_contract=rust-lifecycle-variable-context-explicit-mutation-surface-selection-v0
selected_policy=ExplicitMutationApiOnly
owner_kind=VariableContextReturnedMutableBorrowPolicyDecision
variable_map_mut_selected=0
variable_map_mut_deny_reason=ReturnedMutableBorrow
selected_mutation_ops=insert,remove,restore,replace_owned_map
candidate_pool_state_after_this_card=BlockedUntilExplicitMutationProjection
next_action=MIRBUILDER-VARIABLE-CONTEXT-EXPLICIT-MUTATION-API-PROJECTION-001
manual_family_selection=0
full_variable_context_claim=0
returned_mutable_borrow_selected=0
mut_lease_selected=0
runtime_fallback=0
new_backend_route=0
new_abi=0
new_python_semantic_projector=0
source_selfhost_claim=0
summary=ok
```
