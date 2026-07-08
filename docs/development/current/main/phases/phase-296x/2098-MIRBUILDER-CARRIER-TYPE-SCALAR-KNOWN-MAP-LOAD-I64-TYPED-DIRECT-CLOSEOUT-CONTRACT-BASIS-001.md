# 2098 - MIRBUILDER-CARRIER-TYPE-SCALAR-KNOWN-MAP-LOAD-I64-TYPED-DIRECT-CLOSEOUT-CONTRACT-BASIS-001

## Token

```text
MIRBUILDER-CARRIER-TYPE-SCALAR-KNOWN-MAP-LOAD-I64-TYPED-DIRECT-CLOSEOUT-CONTRACT-BASIS-001
```

## Purpose

Define the consultation-approved narrow typed direct closeout contract basis
for the existing `MapLoadScalarI64` / `ScalarI64OrMissingZero` Rust evidence.

This card does not select `ScalarKnownTransportAxis`, does not mark
`ScalarKnownCloseoutAuthority` as accepted/root, and does not select a
component-specific card.

The purpose is to turn one existing narrow Rust-owner evidence slice into an
accepted-source candidate for the next component requirement rerun.

## Input State

```text
previous:
  MIRBUILDER-CARRIER-TYPE-TRANSPORT-REMAINING-AXIS-COMPONENT-REQUIREMENT-RERUN-002

previous result:
  accepted_component_evidence_source_count = 0
  root_component_requirement_count = 0
  selection_eligible_component_requirement_count = 0
  decision = KeepStopped
  selected_next_card = SOURCE-SELFHOST-WIDER-ROUTE-SELECTION-DESIGN-STOP-001

consultation result:
  do not choose one of the seven blockers directly
  do not reopen same-shape machine inventory/rerun loops
  define one narrow TypedDirectCloseoutContract basis first
```

## Contract Basis

```text
contract_id:
  MapLoadScalarI64ScalarKnownTypedDirectCloseoutContract

target requirement:
  ScalarKnownCloseoutAuthority

candidate axis:
  ScalarKnownTransportAxis

accepted source kind:
  TypedDirectCloseoutContract

existing Rust evidence:
  GenericMethodRouteKind::MapLoadScalarI64
  GenericMethodReturnShape::ScalarI64OrMissingZero
  prove_scalar_i64_map_get_store_fact
  GenericMethodValueDemand::ScalarI64
  GenericMethodPublicationPolicy::NoPublication
```

## Result

```text
typed_direct_closeout_contract_basis = 1
map_load_scalar_i64_existing_rust_owner_evidence = 1
scalar_i64_or_missing_zero_return_shape_evidence = 1
scalar_i64_value_demand_evidence = 1
no_publication_policy_evidence = 1
basis_only = 1
rerun_required_before_component_selection = 1

decision:
  SelectCarrierTypeRemainingAxisComponentRequirementRerun003

selected_next_card:
  MIRBUILDER-CARRIER-TYPE-TRANSPORT-REMAINING-AXIS-COMPONENT-REQUIREMENT-RERUN-003
```

## Guard

```text
tools/checks/rust_lifecycle_mirbuilder_carrier_type_scalar_known_map_load_i64_typed_direct_closeout_contract_basis_guard.sh
```

## Output

```text
fixture:
  docs/development/current/main/design/fixtures/rust-lifecycle/
    mirbuilder-carrier-type-scalar-known-map-load-i64-typed-direct-closeout-contract-basis-v0.json

tool:
  tools/rust_lifecycle/
    mirbuilder_carrier_type_scalar_known_map_load_i64_typed_direct_closeout_contract_basis.py

guard:
  tools/checks/
    rust_lifecycle_mirbuilder_carrier_type_scalar_known_map_load_i64_typed_direct_closeout_contract_basis_guard.sh
```

## Non-Claims

```text
scalar_known_transport_axis_closeout = 0
scalar_known_closeout_authority_accepted_root = 0
target_requirement_acceptance_claim = 0
root_component_requirement_selected = 0
component_specific_card_selection = 0
concrete_carrier_type_axis_selection = 0

source_selfhost_claim = 0
native_seed_materialization = 0
hako_generation = 0
hako_adopted_decision = 0
runtime_fallback = 0
new_backend_route = 0
new_abi = 0
new_python_semantic_projector = 0

source_path_as_authority = 0
owner_name_as_proof = 0
row_count_as_proof = 0
route_membership_alone_as_proof = 0
return_type_string_mapping_as_proof = 0
observed_subaxis_set_as_proof = 0
hardcoded_carrier_axis_priority = 0
manual_axis_selection = 0
manual_carrier_selection = 0
```
