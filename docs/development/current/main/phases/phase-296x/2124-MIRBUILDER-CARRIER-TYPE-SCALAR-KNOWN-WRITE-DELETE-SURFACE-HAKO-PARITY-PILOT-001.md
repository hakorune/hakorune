# 2124 - MIRBUILDER-CARRIER-TYPE-SCALAR-KNOWN-WRITE-DELETE-SURFACE-HAKO-PARITY-PILOT-001

## Token

```text
MIRBUILDER-CARRIER-TYPE-SCALAR-KNOWN-WRITE-DELETE-SURFACE-HAKO-PARITY-PILOT-001
```

## Purpose

Land the narrow Delete Write `.hako` implementation pilot for
`DeleteSurfacePolicy / MapDeleteAny`.

This card adds the hand-authored classifier/policy mirror. It is an
implementation pilot only: parity gate, direct closeout, and adoption remain
separate cards.

## Implementation

```text
hako_source:
  lang/src/compiler/lib/write_delete_surface_policy_classifier.hako

box:
  WriteDeleteSurfacePolicyClassifierBox

method:
  classify(route_kind)
```

## Result

```text
write_delete_surface_hako_parity_pilot = 1
hako_implementation_landed = 1
hako_source_verifies = 1
map_delete_any_scope = 1
delete_surface_policy_scope = 1
none_publication_metadata_declared = 1
classifier_policy_mirror_only = 1
parity_gate_required = 1

publication_execution = 0
runtime_mutation_authority = 0
write_direct_closeout_materialized = 0
write_scalar_i64_routes_closeout = 0
scalar_known_transport_axis_closeout = 0
hako_adoption = 0
source_selfhost_claim = 0

selected_next_card:
  MIRBUILDER-CARRIER-TYPE-SCALAR-KNOWN-WRITE-DELETE-SURFACE-PARITY-GATE-001
```

## Guard

```text
tools/checks/
  rust_lifecycle_mirbuilder_carrier_type_scalar_known_write_delete_surface_hako_parity_pilot_guard.sh
```

## Output

```text
hako:
  lang/src/compiler/lib/write_delete_surface_policy_classifier.hako

fixture:
  docs/development/current/main/design/fixtures/rust-lifecycle/
    mirbuilder-carrier-type-scalar-known-write-delete-surface-hako-parity-pilot-v0.json

tool:
  tools/rust_lifecycle/
    mirbuilder_carrier_type_scalar_known_write_delete_surface_hako_parity_pilot.py
```

## Non-Claims

```text
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
