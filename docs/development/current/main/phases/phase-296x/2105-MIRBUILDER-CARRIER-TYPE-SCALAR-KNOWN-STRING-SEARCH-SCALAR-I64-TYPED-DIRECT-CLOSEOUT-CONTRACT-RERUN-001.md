# 2105 - MIRBUILDER-CARRIER-TYPE-SCALAR-KNOWN-STRING-SEARCH-SCALAR-I64-TYPED-DIRECT-CLOSEOUT-CONTRACT-RERUN-001

## Token

```text
MIRBUILDER-CARRIER-TYPE-SCALAR-KNOWN-STRING-SEARCH-SCALAR-I64-TYPED-DIRECT-CLOSEOUT-CONTRACT-RERUN-001
```

## Purpose

Materialize the scoped `StringSearchScalarI64TypedDirectCloseoutContract`
after the 2104 basis.

This card closes the string-search scalar i64 surface only. It does not close
`ScalarKnownTransportAxis` as a whole.

## Result

```text
string_search_scalar_i64_typed_direct_closeout_contract_materialized = 1
accepted_scoped_closeout_count = 2
remaining_uncovered_scalar_known_surface_count = 2
scalar_known_transport_axis_closeout = 0

remaining_uncovered_surface_ids:
  CollectionScalarI64Routes
  WriteScalarI64Routes

decision:
  KeepScopedCloseout

reason:
  ScalarKnownTransportAxisStillHasUncoveredSurfaces

selected_next_card:
  SOURCE-SELFHOST-WIDER-ROUTE-SELECTION-DESIGN-STOP-001
```

## Guard

```text
tools/checks/rust_lifecycle_mirbuilder_carrier_type_scalar_known_string_search_scalar_i64_typed_direct_closeout_contract_rerun_guard.sh
```

## Output

```text
fixture:
  docs/development/current/main/design/fixtures/rust-lifecycle/
    mirbuilder-carrier-type-scalar-known-string-search-scalar-i64-typed-direct-closeout-contract-rerun-v0.json

tool:
  tools/rust_lifecycle/
    mirbuilder_carrier_type_scalar_known_string_search_scalar_i64_typed_direct_closeout_contract_rerun.py

guard:
  tools/checks/
    rust_lifecycle_mirbuilder_carrier_type_scalar_known_string_search_scalar_i64_typed_direct_closeout_contract_rerun_guard.sh
```

## Non-Claims

```text
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
