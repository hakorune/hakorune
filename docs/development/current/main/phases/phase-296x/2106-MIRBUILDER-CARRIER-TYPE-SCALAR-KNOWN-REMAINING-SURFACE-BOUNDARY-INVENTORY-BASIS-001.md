# 2106 - MIRBUILDER-CARRIER-TYPE-SCALAR-KNOWN-REMAINING-SURFACE-BOUNDARY-INVENTORY-BASIS-001

## Token

```text
MIRBUILDER-CARRIER-TYPE-SCALAR-KNOWN-REMAINING-SURFACE-BOUNDARY-INVENTORY-BASIS-001
```

## Purpose

Define a basis-only boundary inventory for the two ScalarKnown surfaces left
after 2105.

This card does not select `CollectionScalarI64Routes` or
`WriteScalarI64Routes` directly. It records why each surface still needs a
separate rerun before a narrow `TypedDirectCloseoutContract` can be accepted.

## Boundary Inventory

```text
CollectionScalarI64Routes:
  candidate_contract_id = CollectionLenScalarI64TypedDirectCloseoutContract
  route_kind_set = MapEntryCount, ArraySlotLen, StringLen, AnyLength
  proof_or_policy_source = LenSurfacePolicy
  return_shape = ScalarI64
  value_demand = ScalarI64
  publication_policy = NoPublication
  effect_class = observe
  blocked_by = CollectionBoundarySeparationFromMapLoadRequired

WriteScalarI64Routes:
  candidate_contract_id = WriteResultScalarI64ClassificationOnly
  route_kind_set = ArrayAppendAny, MapDeleteAny, MapStoreI64, MapStoreAny
  proof_or_policy_source = PushSurfacePolicy, DeleteSurfacePolicy, SetSurfacePolicy
  return_shape = ScalarI64OrNoneMixed
  value_demand = WriteAny
  publication_policy = MixedNoPublicationAndNone
  effect_class = mutate
  blocked_by = WriteResultPolicyRequiredBeforeDirectCloseout
```

## Result

```text
remaining_surface_boundary_inventory_basis = 1
collection_surface_inventory = 1
write_surface_inventory = 1
direct_contract_selection = 0
collection_direct_closeout_ready = 0
write_direct_closeout_ready = 0
scalar_known_transport_axis_closeout = 0

decision:
  SelectRemainingSurfaceBoundaryInventoryRerun

reason:
  CollectionMixedWithPriorMapLoadAndWriteResultPolicyUnresolved

selected_next_card:
  MIRBUILDER-CARRIER-TYPE-SCALAR-KNOWN-REMAINING-SURFACE-BOUNDARY-INVENTORY-RERUN-001
```

## Guard

```text
tools/checks/rust_lifecycle_mirbuilder_carrier_type_scalar_known_remaining_surface_boundary_inventory_basis_guard.sh
```

## Output

```text
fixture:
  docs/development/current/main/design/fixtures/rust-lifecycle/
    mirbuilder-carrier-type-scalar-known-remaining-surface-boundary-inventory-basis-v0.json

tool:
  tools/rust_lifecycle/
    mirbuilder_carrier_type_scalar_known_remaining_surface_boundary_inventory_basis.py

guard:
  tools/checks/
    rust_lifecycle_mirbuilder_carrier_type_scalar_known_remaining_surface_boundary_inventory_basis_guard.sh
```

## Non-Claims

```text
direct_contract_selection = 0
collection_direct_closeout_ready = 0
write_direct_closeout_ready = 0
scalar_known_transport_axis_closeout = 0
source_selfhost_claim = 0
native_seed_materialization = 0
hako_generation = 0
hako_adopted_decision = 0
runtime_fallback = 0
new_backend_route = 0
new_abi = 0
new_python_semantic_projector = 0
manual_axis_selection = 0
manual_carrier_selection = 0
row_count_as_proof = 0
source_path_as_authority = 0
owner_name_as_proof = 0
route_membership_alone_as_proof = 0
```
