---
Status: Active
Decision: accepted
Date: 2026-06-29
Scope: Materialize the explicit mutation API surface selected by
  MIRBUILDER-VARIABLE-CONTEXT-EXPLICIT-MUTATION-SURFACE-SELECTION-001.
Related:
  - docs/development/current/main/phases/phase-296x/1790-MIRBUILDER-VARIABLE-CONTEXT-EXPLICIT-MUTATION-SURFACE-SELECTION-001.md
  - docs/development/current/main/design/fixtures/rust-lifecycle/variable-context-explicit-mutation-api-projection-v0.json
  - tools/checks/rust_lifecycle_variable_context_explicit_mutation_api_projection_guard.sh
---

# MIRBUILDER-VARIABLE-CONTEXT-EXPLICIT-MUTATION-API-PROJECTION-001

## Goal

Materialize the selected `ExplicitMutationApiOnly` policy as a bounded native
VariableContext surface. The raw `variable_map_mut()` returned borrow stays
denied; explicit mutation APIs own the mutation surface instead.

```text
docs_only_closeout = forbidden
code_or_guard_delta_required = 1
```

## Projection

```text
source_method:
  VariableContext::variable_map_mut

rust_return:
  &mut BTreeMap<String, ValueId>

selected_hako_apis:
  VariableContextNativeApi.insert
  VariableContextNativeApi.remove
  VariableContextNativeApi.restore
  VariableContextNativeApi.replace_owned_map

projection_policy:
  ExplicitMutationApiOnly

raw_mutable_alias:
  denied
```

## Oracle Vectors

```text
replace_owned_map_overwrites_seed:
  source before = {seed: 1}
  owned input = {a: 10, b: 20}
  source after = {a: 10, b: 20}

owned_alias_isolation_after_replace:
  owned after mutation = {a: 10, b: 20, c: 30}
  source remains = {a: 10, b: 20}

restore_alias_isolation_after_snapshot:
  snapshot after mutation = {a: 10, b: 20, e: 50}
  source after restore = {a: 10, b: 20, e: 50}
  snapshot after remove = {b: 20, e: 50}
  source after snapshot remove = {a: 10, b: 20, e: 50}
```

## Acceptance

```text
output_contract=rust-lifecycle-variable-context-explicit-mutation-api-projection-v0
selected_policy=ExplicitMutationApiOnly
owner_kind=VariableContextReturnedMutableBorrowPolicyDecision
selected_hako_apis=insert,remove,restore,replace_owned_map
raw_variable_map_mut_alias_emitted=0
variable_map_mut_selected=0
replace_owned_map_native_api=1
insert_native_api=1
remove_native_api=1
restore_native_api=1
candidate_pool_state_after_this_card=BlockedUntilRouteMatrixRerun
next_action=MIRBUILDER-VARIABLE-CONTEXT-ROUTE-MATRIX-RERUN-002
manual_family_selection=0
full_variable_context_claim=0
returned_mutable_borrow_selected=0
mut_lease_selected=0
runtime_fallback=0
new_backend_route=0
new_abi=0
new_python_semantic_projector=0
source_selfhost_claim=0
```

## Non-Claims

```text
raw mutable alias transport = 0
MutLease = 0
full VariableContext = 0
Source Selfhost = 0
runtime fallback = 0
new backend route = 0
new ABI = 0
```

## Closeout

```text
output_contract=rust-lifecycle-variable-context-explicit-mutation-api-projection-v0
selected_policy=ExplicitMutationApiOnly
owner_kind=VariableContextReturnedMutableBorrowPolicyDecision
selected_hako_apis=insert,remove,restore,replace_owned_map
raw_variable_map_mut_alias_emitted=0
variable_map_mut_selected=0
replace_owned_map_native_api=1
candidate_pool_state_after_this_card=BlockedUntilRouteMatrixRerun
next_action=MIRBUILDER-VARIABLE-CONTEXT-ROUTE-MATRIX-RERUN-002
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
