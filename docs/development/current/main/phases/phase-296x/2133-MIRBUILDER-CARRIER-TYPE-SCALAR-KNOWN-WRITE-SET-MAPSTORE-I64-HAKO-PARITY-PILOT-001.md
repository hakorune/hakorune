# 2133 - MIRBUILDER-CARRIER-TYPE-SCALAR-KNOWN-WRITE-SET-MAPSTORE-I64-HAKO-PARITY-PILOT-001

## Token

```text
MIRBUILDER-CARRIER-TYPE-SCALAR-KNOWN-WRITE-SET-MAPSTORE-I64-HAKO-PARITY-PILOT-001
```

## Purpose

Land the narrow Set Write `.hako` implementation pilot for
`SetSurfacePolicy / MapStoreI64`.

This card adds the hand-authored classifier/policy mirror. It is an
implementation pilot only: parity gate, direct closeout, and adoption remain
separate cards. `MapStoreAny` remains deferred.

## Implementation

```text
hako_source:
  lang/src/compiler/lib/write_set_mapstore_i64_policy_classifier.hako

box:
  WriteSetMapStoreI64PolicyClassifierBox

method:
  classify(route_kind)
```

## Result

```text
write_set_mapstore_i64_hako_parity_pilot = 1
hako_implementation_landed = 1
hako_source_verifies = 1
mapstore_i64_scope = 1
set_surface_policy_scope = 1
none_result_metadata_declared = 1
none_publication_metadata_reused = 1
classifier_policy_mirror_only = 1
mapstore_any_deferred = 1
parity_gate_required = 1

any_write_boundary_opened = 0
publication_execution = 0
runtime_mutation_authority = 0
write_direct_closeout_materialized = 0
write_scalar_i64_routes_closeout = 0
scalar_known_transport_axis_closeout = 0
hako_adoption = 0
source_selfhost_claim = 0

selected_next_card:
  MIRBUILDER-CARRIER-TYPE-SCALAR-KNOWN-WRITE-SET-MAPSTORE-I64-PARITY-GATE-001
```

## Guard

```text
tools/checks/
  rust_lifecycle_mirbuilder_carrier_type_scalar_known_write_set_mapstore_i64_hako_parity_pilot_guard.sh
```

## Output

```text
hako:
  lang/src/compiler/lib/write_set_mapstore_i64_policy_classifier.hako

fixture:
  docs/development/current/main/design/fixtures/rust-lifecycle/
    mirbuilder-carrier-type-scalar-known-write-set-mapstore-i64-hako-parity-pilot-v0.json

tool:
  tools/rust_lifecycle/
    mirbuilder_carrier_type_scalar_known_write_set_mapstore_i64_hako_parity_pilot.py
```

## Non-Claims

```text
any_write_boundary_opened = 0
publication_execution = 0
runtime_mutation_authority = 0
write_direct_closeout_materialized = 0
write_scalar_i64_routes_closeout = 0
scalar_known_transport_axis_closeout = 0
hako_adoption = 0
source_selfhost_claim = 0
new_route_authority = 0
behavior_change = 0
runtime_fallback = 0
new_backend_route = 0
new_abi = 0
native_seed_materialization = 0
hako_generation = 0
hako_adopted_decision = 0
```
