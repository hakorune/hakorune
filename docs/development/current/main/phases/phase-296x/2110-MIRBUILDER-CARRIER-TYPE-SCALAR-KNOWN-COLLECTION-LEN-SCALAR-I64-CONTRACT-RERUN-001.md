# 2110 - MIRBUILDER-CARRIER-TYPE-SCALAR-KNOWN-COLLECTION-LEN-SCALAR-I64-CONTRACT-RERUN-001

## Token

```text
MIRBUILDER-CARRIER-TYPE-SCALAR-KNOWN-COLLECTION-LEN-SCALAR-I64-CONTRACT-RERUN-001
```

## Purpose

Rerun the CollectionLen ScalarI64 typed direct closeout contract after the
2109 basis card.

This card materializes the scoped CollectionLen contract only. It does not close
the full `ScalarKnownTransportAxis`, because `WriteScalarI64Routes` still needs
a write result policy basis.

## Materialized Contract

```text
contract_id = CollectionLenScalarI64TypedDirectCloseoutContract
surface_id = CollectionScalarI64Routes
route_kind_set = MapEntryCount, ArraySlotLen, StringLen, AnyLength
proof_or_policy_source = LenSurfacePolicy
return_shape = ScalarI64
value_demand = ScalarI64
publication_policy = NoPublication
core_method_lowering_tier = WarmDirectAbi
effect_class = observe
```

## Result

```text
collection_len_scalar_i64_contract_materialized = 1
accepted_scoped_closeout_count = 3
remaining_candidate_surface_count = 1
remaining_candidate_surface_id = WriteScalarI64Routes
write_result_policy_ready = 0
scalar_known_transport_axis_closeout = 0

decision:
  SelectWriteResultPolicyBasis

selected_next_card:
  MIRBUILDER-CARRIER-TYPE-SCALAR-KNOWN-WRITE-RESULT-POLICY-BASIS-001
```

## Guard

```text
tools/checks/rust_lifecycle_mirbuilder_carrier_type_scalar_known_collection_len_scalar_i64_contract_rerun_guard.sh
```

## Output

```text
fixture:
  docs/development/current/main/design/fixtures/rust-lifecycle/
    mirbuilder-carrier-type-scalar-known-collection-len-scalar-i64-contract-rerun-v0.json

tool:
  tools/rust_lifecycle/
    mirbuilder_carrier_type_scalar_known_collection_len_scalar_i64_contract_rerun.py

guard:
  tools/checks/
    rust_lifecycle_mirbuilder_carrier_type_scalar_known_collection_len_scalar_i64_contract_rerun_guard.sh
```

## Non-Claims

```text
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
