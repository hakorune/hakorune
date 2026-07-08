# 2131 - MIRBUILDER-CARRIER-TYPE-SCALAR-KNOWN-WRITE-SET-SURFACE-TYPED-VALUE-SPLIT-BASIS-001

## Token

```text
MIRBUILDER-CARRIER-TYPE-SCALAR-KNOWN-WRITE-SET-SURFACE-TYPED-VALUE-SPLIT-BASIS-001
```

## Purpose

Define the basis-only split for `SetSurfacePolicy` before any Set `.hako`
pilot.

This card records the consultation-approved proof axis for splitting
`MapStoreI64` from `MapStoreAny`. It does not select or implement a `.hako`
pilot.

## Proof Axis

```text
PriorHakoAdoptedWriteSurfaceMetadataCoverage
AND
TypedScalarWriteBeforeAnyWrite
```

Already covered by Push/Delete:

```text
MutatesReceiverOrContainer metadata
NonePublication metadata
```

New for Set:

```text
NoneResult metadata
TypedVsAnyWriteValueBoundary
```

## Split Plan

```text
SetSurfacePolicy:
  MapStoreI64:
    first_candidate = true
    typed_scalar_write = true
    write_value_boundary = ScalarI64
    scalar_known_lane_local = true

  MapStoreAny:
    deferred = true
    typed_scalar_write = false
    write_value_boundary = Any
    requires_any_write_boundary = true
```

## Result

```text
set_surface_typed_value_split_basis = 1
set_surface_policy_remaining = 1
mapstore_i64_first_candidate = 1
mapstore_any_deferred = 1
typed_scalar_write_before_any_write = 1
prior_hako_adopted_write_surface_metadata_coverage = 1
basis_only = 1
rerun_or_fixture_required_before_hako_pilot = 1

set_hako_pilot_selected = 0
mapstore_i64_hako_pilot_selected = 0
mapstore_any_hako_pilot_selected = 0

selected_next_card:
  MIRBUILDER-CARRIER-TYPE-SCALAR-KNOWN-WRITE-SET-MAPSTORE-I64-RUST-ORACLE-PARITY-FIXTURE-001
```

## Guard

```text
tools/checks/
  rust_lifecycle_mirbuilder_carrier_type_scalar_known_write_set_surface_typed_value_split_basis_guard.sh
```

## Non-Claims

```text
set_hako_pilot_selected = 0
mapstore_i64_hako_pilot_selected = 0
mapstore_any_hako_pilot_selected = 0
set_split_unnecessary = 0
write_direct_closeout_materialized = 0
write_scalar_i64_routes_closeout = 0
scalar_known_transport_axis_closeout = 0
source_selfhost_claim = 0
runtime_mutation_authority = 0
publication_execution = 0
new_route_authority = 0
new_backend_route = 0
new_abi = 0
behavior_change = 0
runtime_fallback = 0
native_seed_materialization = 0
hako_generation = 0
new_python_semantic_projector = 0
route_count_as_proof = 0
apparent_simplicity_as_proof = 0
manual_subsurface_selection = 0
accepted_read_contract_similarity_as_proof = 0
owner_name_as_proof = 0
source_path_as_authority = 0
route_membership_alone_as_proof = 0
```
