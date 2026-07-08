# 2112 - MIRBUILDER-CARRIER-TYPE-SCALAR-KNOWN-WRITE-RESULT-POLICY-RERUN-001

## Token

```text
MIRBUILDER-CARRIER-TYPE-SCALAR-KNOWN-WRITE-RESULT-POLICY-RERUN-001
```

## Purpose

Rerun the WriteResultPolicy basis from 2111 and decide whether the remaining
`WriteScalarI64Routes` surface can become a whole direct closeout contract.

This rerun rejects whole-surface direct closeout because the write surface still
has multiple result/publication signatures. It selects a priority basis for the
Push/Delete/Set sub-surface split instead of selecting any sub-surface directly.

## Evaluation

```text
policy_id = WriteResultPolicyV1
target_surface_id = WriteScalarI64Routes
subsurface_count = 3
normalized_signature_count = 3
whole_direct_contract_allowed = false

whole_direct_contract_blocked_by:
  MixedReturnPublicationNotStableDirectContract
  MultipleWriteSubsurfaceResultPublicationSignatures
```

## Sub-Surface Candidates

```text
PushSurfacePolicy:
  routes = ArrayAppendAny
  selection_eligible_without_priority_basis = false
  blocked_by = NoWriteSubsurfacePriorityBasis

DeleteSurfacePolicy:
  routes = MapDeleteAny
  selection_eligible_without_priority_basis = false
  blocked_by = NoWriteSubsurfacePriorityBasis

SetSurfacePolicy:
  routes = MapStoreI64, MapStoreAny
  selection_eligible_without_priority_basis = false
  blocked_by = NoWriteSubsurfacePriorityBasis
```

## Result

```text
write_result_policy_rerun = 1
write_result_policy_basis_consumed = 1
write_surface_whole_direct_contract_rejected = 1
write_subsurface_split_required = 1
write_subsurface_priority_basis_selected = 1
write_subsurface_candidate_count = 3
whole_direct_contract_candidate_count = 0

write_direct_closeout_materialized = 0
write_result_policy_ready = 0
write_scalar_i64_routes_closeout = 0
scalar_known_transport_axis_closeout = 0

decision:
  SelectWriteSubsurfacePriorityBasis

selected_next_card:
  MIRBUILDER-CARRIER-TYPE-SCALAR-KNOWN-WRITE-SUBSURFACE-PRIORITY-BASIS-001
```

## Guard

```text
tools/checks/rust_lifecycle_mirbuilder_carrier_type_scalar_known_write_result_policy_rerun_guard.sh
```

## Output

```text
fixture:
  docs/development/current/main/design/fixtures/rust-lifecycle/
    mirbuilder-carrier-type-scalar-known-write-result-policy-rerun-v0.json

tool:
  tools/rust_lifecycle/
    mirbuilder_carrier_type_scalar_known_write_result_policy_rerun.py

guard:
  tools/checks/
    rust_lifecycle_mirbuilder_carrier_type_scalar_known_write_result_policy_rerun_guard.sh
```

## Non-Claims

```text
write_direct_closeout_materialized = 0
write_result_policy_ready = 0
write_scalar_i64_routes_closeout = 0
scalar_known_transport_axis_closeout = 0
direct_whole_write_contract_basis = 0
component_specific_direct_contract_materialized = 0
hako_adoption = 0
source_selfhost_claim = 0
new_route_authority = 0
behavior_change = 0
runtime_fallback = 0
new_backend_route = 0
new_abi = 0
native_seed_materialization = 0
hako_generation = 0
hako_adopted_decision = 0
new_python_semantic_projector = 0
manual_subsurface_selection = 0
manual_axis_selection = 0
manual_carrier_selection = 0
route_count_as_proof = 0
source_path_as_authority = 0
owner_name_as_proof = 0
route_membership_alone_as_proof = 0
```
