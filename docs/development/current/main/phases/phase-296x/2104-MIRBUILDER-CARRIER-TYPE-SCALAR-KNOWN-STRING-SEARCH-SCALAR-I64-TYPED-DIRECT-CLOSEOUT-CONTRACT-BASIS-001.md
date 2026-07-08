# 2104 - MIRBUILDER-CARRIER-TYPE-SCALAR-KNOWN-STRING-SEARCH-SCALAR-I64-TYPED-DIRECT-CLOSEOUT-CONTRACT-BASIS-001

## Token

```text
MIRBUILDER-CARRIER-TYPE-SCALAR-KNOWN-STRING-SEARCH-SCALAR-I64-TYPED-DIRECT-CLOSEOUT-CONTRACT-BASIS-001
```

## Purpose

Define the basis-only `StringSearchScalarI64TypedDirectCloseoutContract`
selected by the 2103 classification rerun.

This card does not materialize the typed direct closeout contract. It records
the contract shape and selects a rerun.

## Contract

```text
contract_id:
  StringSearchScalarI64TypedDirectCloseoutContract

routes:
  StringIndexOf
  StringLastIndexOf
  StringContains

return_shape:
  ScalarI64

value_demand:
  ScalarI64

publication_policy:
  NoPublication

core_method_lowering_tier:
  WarmDirectAbi

effect_class:
  read
```

## Result

```text
typed_direct_closeout_contract_basis = 1
string_search_route_count = 3
direct_contract_materialized = 0
scalar_known_transport_axis_closeout = 0

decision:
  SelectStringSearchScalarI64TypedDirectCloseoutContractRerun

selected_next_card:
  MIRBUILDER-CARRIER-TYPE-SCALAR-KNOWN-STRING-SEARCH-SCALAR-I64-TYPED-DIRECT-CLOSEOUT-CONTRACT-RERUN-001
```

## Guard

```text
tools/checks/rust_lifecycle_mirbuilder_carrier_type_scalar_known_string_search_scalar_i64_typed_direct_closeout_contract_basis_guard.sh
```

## Output

```text
fixture:
  docs/development/current/main/design/fixtures/rust-lifecycle/
    mirbuilder-carrier-type-scalar-known-string-search-scalar-i64-typed-direct-closeout-contract-basis-v0.json

tool:
  tools/rust_lifecycle/
    mirbuilder_carrier_type_scalar_known_string_search_scalar_i64_typed_direct_closeout_contract_basis.py

guard:
  tools/checks/
    rust_lifecycle_mirbuilder_carrier_type_scalar_known_string_search_scalar_i64_typed_direct_closeout_contract_basis_guard.sh
```

## Non-Claims

```text
direct_contract_materialized = 0
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
