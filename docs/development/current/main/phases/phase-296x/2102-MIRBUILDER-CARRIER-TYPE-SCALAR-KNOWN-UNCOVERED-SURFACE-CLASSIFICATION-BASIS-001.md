# 2102 - MIRBUILDER-CARRIER-TYPE-SCALAR-KNOWN-UNCOVERED-SURFACE-CLASSIFICATION-BASIS-001

## Token

```text
MIRBUILDER-CARRIER-TYPE-SCALAR-KNOWN-UNCOVERED-SURFACE-CLASSIFICATION-BASIS-001
```

## Purpose

Define a basis-only classification for the three uncovered scalar-known
surfaces left after 2101.

This card does not select `StringScalarI64Routes`,
`CollectionScalarI64Routes`, or `WriteScalarI64Routes` directly. It defines
the dimensions needed for a rerun to choose the next narrow
`TypedDirectCloseoutContract` without using source path, owner name, row count,
or route membership alone as proof.

## Classification Dimensions

```text
surface_id
route_kind_set
method_surface
return_shape
value_demand
publication_policy
proof_or_policy_source
core_method_op
core_method_lowering_tier
effect_class
receiver_key_value_result_origin_evidence
test_anchor
```

## Surface Classes

```text
StringScalarI64Routes:
  candidate_contract_id = StringSearchScalarI64TypedDirectCloseoutContract
  route_kind_set = StringIndexOf, StringLastIndexOf, StringContains
  effect_class = read
  core_method_lowering_tier = WarmDirectAbi
  priority_hint = lowest_risk_candidate

CollectionScalarI64Routes:
  candidate_contract_id = CollectionLenScalarI64TypedDirectCloseoutContract
  route_kind_set = MapEntryCount, ArraySlotLen, StringLen, AnyLength
  effect_class = observe
  priority_hint = mixed_with_already_closed_map_load

WriteScalarI64Routes:
  candidate_contract_id = WriteResultScalarI64ClassificationOnly
  route_kind_set = ArrayAppendAny, MapDeleteAny, MapStoreI64, MapStoreAny
  effect_class = mutate
  priority_hint = do_not_select_before_write_result_policy
```

## Result

```text
classification_basis = 1
classified_surface_count = 3
direct_contract_selection = 0
scalar_known_transport_axis_closeout = 0

decision:
  SelectScalarKnownUncoveredSurfaceClassificationRerun

selected_next_card:
  MIRBUILDER-CARRIER-TYPE-SCALAR-KNOWN-UNCOVERED-SURFACE-CLASSIFICATION-RERUN-001
```

## Guard

```text
tools/checks/rust_lifecycle_mirbuilder_carrier_type_scalar_known_uncovered_surface_classification_basis_guard.sh
```

## Output

```text
fixture:
  docs/development/current/main/design/fixtures/rust-lifecycle/
    mirbuilder-carrier-type-scalar-known-uncovered-surface-classification-basis-v0.json

tool:
  tools/rust_lifecycle/
    mirbuilder_carrier_type_scalar_known_uncovered_surface_classification_basis.py

guard:
  tools/checks/
    rust_lifecycle_mirbuilder_carrier_type_scalar_known_uncovered_surface_classification_basis_guard.sh
```

## Non-Claims

```text
direct_contract_selection = 0
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
