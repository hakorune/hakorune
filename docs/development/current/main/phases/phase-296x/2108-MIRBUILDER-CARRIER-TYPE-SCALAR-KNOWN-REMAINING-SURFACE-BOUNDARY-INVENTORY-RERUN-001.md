# 2108 - MIRBUILDER-CARRIER-TYPE-SCALAR-KNOWN-REMAINING-SURFACE-BOUNDARY-INVENTORY-RERUN-001

## Token

```text
MIRBUILDER-CARRIER-TYPE-SCALAR-KNOWN-REMAINING-SURFACE-BOUNDARY-INVENTORY-RERUN-001
```

## Purpose

Rerun the remaining ScalarKnown surface boundary inventory after the Rust
`ScalarKnownTypedDirectCloseoutContract` boundary refactor.

This card selects the next basis card only. It does not materialize the
Collection direct closeout contract and does not mark the full
`ScalarKnownTransportAxis` closed.

## Evaluated Surfaces

```text
CollectionScalarI64Routes:
  candidate_contract_id = CollectionLenScalarI64TypedDirectCloseoutContract
  rust_boundary_status = CandidateNeedsPolicy
  selection_eligible = true
  collection_boundary_separated_from_map_load = true
  selected_next_card_if_eligible =
    MIRBUILDER-CARRIER-TYPE-SCALAR-KNOWN-COLLECTION-LEN-SCALAR-I64-CONTRACT-BASIS-001

WriteScalarI64Routes:
  candidate_contract_id = WriteResultScalarI64ClassificationOnly
  rust_boundary_status = CandidateNeedsPolicy
  selection_eligible = false
  blocked_by = WriteResultPolicyRequiredBeforeDirectCloseout
```

## Result

```text
remaining_surface_boundary_inventory_rerun = 1
evaluated_surface_count = 2
selection_eligible_surface_count = 1
selected_surface_id = CollectionScalarI64Routes
selected_contract_id = CollectionLenScalarI64TypedDirectCloseoutContract
collection_boundary_separated_from_map_load = 1
write_result_policy_ready = 0
direct_contract_materialized = 0
collection_direct_closeout_ready = 0
write_direct_closeout_ready = 0
scalar_known_transport_axis_closeout = 0

decision:
  SelectCollectionLenScalarI64ContractBasis

reason:
  ExactlyOneRemainingScalarKnownSurfaceBoundaryEligible

selected_next_card:
  MIRBUILDER-CARRIER-TYPE-SCALAR-KNOWN-COLLECTION-LEN-SCALAR-I64-CONTRACT-BASIS-001
```

## Guard

```text
tools/checks/rust_lifecycle_mirbuilder_carrier_type_scalar_known_remaining_surface_boundary_inventory_rerun_guard.sh
```

## Output

```text
fixture:
  docs/development/current/main/design/fixtures/rust-lifecycle/
    mirbuilder-carrier-type-scalar-known-remaining-surface-boundary-inventory-rerun-v0.json

tool:
  tools/rust_lifecycle/
    mirbuilder_carrier_type_scalar_known_remaining_surface_boundary_inventory_rerun.py

guard:
  tools/checks/
    rust_lifecycle_mirbuilder_carrier_type_scalar_known_remaining_surface_boundary_inventory_rerun_guard.sh
```

## Non-Claims

```text
direct_contract_materialized = 0
collection_direct_closeout_ready = 0
write_direct_closeout_ready = 0
write_result_policy_ready = 0
scalar_known_transport_axis_closeout = 0
source_selfhost_claim = 0
hako_adoption = 0
new_route_authority = 0
behavior_change = 0
runtime_fallback = 0
new_backend_route = 0
new_abi = 0
native_seed_materialization = 0
hako_generation = 0
hako_adopted_decision = 0
new_python_semantic_projector = 0
manual_axis_selection = 0
manual_carrier_selection = 0
row_count_as_proof = 0
source_path_as_authority = 0
owner_name_as_proof = 0
route_membership_alone_as_proof = 0
```
