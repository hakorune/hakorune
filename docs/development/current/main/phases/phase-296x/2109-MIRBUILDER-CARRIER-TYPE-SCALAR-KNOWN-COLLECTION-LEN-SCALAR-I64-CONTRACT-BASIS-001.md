# 2109 - MIRBUILDER-CARRIER-TYPE-SCALAR-KNOWN-COLLECTION-LEN-SCALAR-I64-CONTRACT-BASIS-001

## Token

```text
MIRBUILDER-CARRIER-TYPE-SCALAR-KNOWN-COLLECTION-LEN-SCALAR-I64-CONTRACT-BASIS-001
```

## Purpose

Define a basis-only typed direct closeout contract for the Collection len/count
ScalarI64 surface after 2108 selected it as the exactly-one eligible remaining
ScalarKnown surface.

This card does not materialize the contract and does not close
`ScalarKnownTransportAxis`.

## Contract

```text
contract_id = CollectionLenScalarI64TypedDirectCloseoutContract
surface_id = CollectionScalarI64Routes
rust_boundary_status = CandidateNeedsPolicy
route_kind_set = MapEntryCount, ArraySlotLen, StringLen, AnyLength
proof_or_policy_source = LenSurfacePolicy
return_shape = ScalarI64
value_demand = ScalarI64
publication_policy = NoPublication
core_method_lowering_tier = WarmDirectAbi
effect_class = observe
separate_from_map_load_contract = true
write_result_policy_required = false
```

## Result

```text
collection_len_scalar_i64_contract_basis = 1
collection_len_route_count = 4
direct_contract_materialized = 0
collection_direct_closeout_ready = 0
scalar_known_transport_axis_closeout = 0

decision:
  SelectCollectionLenScalarI64ContractRerun

selected_next_card:
  MIRBUILDER-CARRIER-TYPE-SCALAR-KNOWN-COLLECTION-LEN-SCALAR-I64-CONTRACT-RERUN-001
```

## Guard

```text
tools/checks/rust_lifecycle_mirbuilder_carrier_type_scalar_known_collection_len_scalar_i64_contract_basis_guard.sh
```

## Output

```text
fixture:
  docs/development/current/main/design/fixtures/rust-lifecycle/
    mirbuilder-carrier-type-scalar-known-collection-len-scalar-i64-contract-basis-v0.json

tool:
  tools/rust_lifecycle/
    mirbuilder_carrier_type_scalar_known_collection_len_scalar_i64_contract_basis.py

guard:
  tools/checks/
    rust_lifecycle_mirbuilder_carrier_type_scalar_known_collection_len_scalar_i64_contract_basis_guard.sh
```

## Non-Claims

```text
direct_contract_materialized = 0
collection_direct_closeout_ready = 0
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
