# 2099 - MIRBUILDER-CARRIER-TYPE-TRANSPORT-REMAINING-AXIS-COMPONENT-REQUIREMENT-RERUN-003

## Token

```text
MIRBUILDER-CARRIER-TYPE-TRANSPORT-REMAINING-AXIS-COMPONENT-REQUIREMENT-RERUN-003
```

## Purpose

Rerun the seven remaining carrier/type component requirements after the
consultation-approved narrow `MapLoadScalarI64` typed direct closeout contract
basis.

This card materializes exactly one accepted component evidence source and
therefore selects the component-specific ScalarKnown closeout basis card.

It does not select a concrete carrier/type axis.

## Input

```text
previous rerun:
  MIRBUILDER-CARRIER-TYPE-TRANSPORT-REMAINING-AXIS-COMPONENT-REQUIREMENT-RERUN-002

accepted-source basis:
  MIRBUILDER-CARRIER-TYPE-SCALAR-KNOWN-MAP-LOAD-I64-TYPED-DIRECT-CLOSEOUT-CONTRACT-BASIS-001
```

## Result

```text
component_requirement_count = 7
accepted_component_evidence_source_count = 1
component_authority_source_count = 1
root_component_requirement_count = 1
selection_eligible_component_requirement_count = 1
component_specific_card_selection_eligible_count = 1
concrete_carrier_type_axis_selection = 0

selected_component_requirement:
  ScalarKnownCloseoutAuthority

decision:
  SelectComponentSpecificCard

reason:
  ExactlyOneCarrierTypeComponentRequirementRoot

selected_next_card:
  MIRBUILDER-CARRIER-TYPE-SCALAR-KNOWN-TRANSPORT-CLOSEOUT-BASIS-001
```

## Accepted Source

```text
source_kind:
  TypedDirectCloseoutContract

closeout_contract_id:
  MapLoadScalarI64ScalarKnownTypedDirectCloseoutContract

route_kind:
  MapLoadScalarI64

return_shape:
  ScalarI64OrMissingZero

proof_function:
  prove_scalar_i64_map_get_store_fact

value_demand:
  ScalarI64

publication_policy:
  NoPublication
```

## Guard

```text
tools/checks/rust_lifecycle_mirbuilder_carrier_type_transport_remaining_axis_component_requirement_rerun_003_guard.sh
```

## Output

```text
fixture:
  docs/development/current/main/design/fixtures/rust-lifecycle/
    mirbuilder-carrier-type-transport-remaining-axis-component-requirement-rerun-003-v0.json

tool:
  tools/rust_lifecycle/
    mirbuilder_carrier_type_transport_remaining_axis_component_requirement_rerun_003.py

guard:
  tools/checks/
    rust_lifecycle_mirbuilder_carrier_type_transport_remaining_axis_component_requirement_rerun_003_guard.sh
```

## Non-Claims

```text
concrete_carrier_type_axis_selection = 0
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
hardcoded_carrier_axis_priority = 0
row_count_as_proof = 0
source_path_as_authority = 0
owner_name_as_proof = 0
route_membership_alone_as_proof = 0
return_type_string_mapping_as_proof = 0
observed_subaxis_set_as_proof = 0
```
