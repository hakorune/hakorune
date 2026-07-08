# 2100 - MIRBUILDER-CARRIER-TYPE-SCALAR-KNOWN-TRANSPORT-CLOSEOUT-BASIS-001

## Token

```text
MIRBUILDER-CARRIER-TYPE-SCALAR-KNOWN-TRANSPORT-CLOSEOUT-BASIS-001
```

## Purpose

Define the basis-only ScalarKnown transport closeout step after rerun-003
selected `ScalarKnownCloseoutAuthority` as the only root component requirement.

This card consumes the accepted `MapLoadScalarI64` typed direct closeout
contract as the closeout input for the selected component requirement.

It does not close `ScalarKnownTransportAxis` yet and does not select a concrete
carrier/type axis.

## Input

```text
previous:
  MIRBUILDER-CARRIER-TYPE-TRANSPORT-REMAINING-AXIS-COMPONENT-REQUIREMENT-RERUN-003

selected component requirement:
  ScalarKnownCloseoutAuthority

accepted contract:
  MapLoadScalarI64ScalarKnownTypedDirectCloseoutContract
```

## Result

```text
scalar_known_transport_closeout_basis = 1
scalar_known_closeout_authority_root_consumed = 1
map_load_scalar_i64_typed_direct_closeout_contract_consumed = 1
basis_only = 1
rerun_required_before_axis_closeout = 1

decision:
  SelectScalarKnownTransportCloseoutRerun

selected_next_card:
  MIRBUILDER-CARRIER-TYPE-SCALAR-KNOWN-TRANSPORT-CLOSEOUT-RERUN-001
```

## Accepted Contract

```text
source_kind:
  TypedDirectCloseoutContract

contract_id:
  MapLoadScalarI64ScalarKnownTypedDirectCloseoutContract

route_kind:
  MapLoadScalarI64

return_shape:
  ScalarI64OrMissingZero

value_demand:
  ScalarI64

publication_policy:
  NoPublication
```

## Guard

```text
tools/checks/rust_lifecycle_mirbuilder_carrier_type_scalar_known_transport_closeout_basis_guard.sh
```

## Output

```text
fixture:
  docs/development/current/main/design/fixtures/rust-lifecycle/
    mirbuilder-carrier-type-scalar-known-transport-closeout-basis-v0.json

tool:
  tools/rust_lifecycle/
    mirbuilder_carrier_type_scalar_known_transport_closeout_basis.py

guard:
  tools/checks/
    rust_lifecycle_mirbuilder_carrier_type_scalar_known_transport_closeout_basis_guard.sh
```

## Non-Claims

```text
scalar_known_transport_axis_closeout = 0
concrete_carrier_type_axis_selection = 0

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
