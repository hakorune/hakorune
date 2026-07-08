# 2144 - MIRBUILDER-CARRIER-TYPE-SCALAR-KNOWN-WRITE-SET-MAPSTORE-ANY-HAKO-ADOPTION-DECISION-001

## Token

```text
MIRBUILDER-CARRIER-TYPE-SCALAR-KNOWN-WRITE-SET-MAPSTORE-ANY-HAKO-ADOPTION-DECISION-001
```

## Purpose

Adopt `write_set_mapstore_any_policy_classifier` as a narrow `.hako` parity
pilot owner after the green 1-row Rust-oracle parity gate.

This decision adopts only the `SetSurfacePolicy / MapStoreAny`
classifier/policy mirror. The Any write boundary remains declared metadata
only. This card does not adopt SetSurfacePolicy as a whole and does not
materialize the MapStoreAny direct closeout.

## Evidence

```text
hako_source:
  lang/src/compiler/lib/write_set_mapstore_any_policy_classifier.hako

parity_gate:
  tools/checks/
    rust_lifecycle_mirbuilder_carrier_type_scalar_known_write_set_mapstore_any_parity_gate.sh

adoption_fixture:
  docs/development/current/main/design/fixtures/rust-lifecycle/
    mirbuilder-carrier-type-scalar-known-write-set-mapstore-any-hako-adoption-decision-v0.json
```

## Acceptance

```text
parity_gate = green
parity_rows = 1
decision = Adopt
write_set_mapstore_any_hako_adopted = 1
hako_adopted_decision = 1
rust_bootstrap_retained = 1
rust_oracle_retained = 1
any_write_boundary_declared = 1

any_write_boundary_opened = 0
write_direct_closeout_materialized = 0
write_result_policy_ready = 0
write_scalar_i64_routes_closeout = 0
scalar_known_transport_axis_closeout = 0
source_selfhost_claim = 0
runtime_mutation_authority = 0
publication_execution = 0
runtime_fallback = 0
new_backend_route = 0
new_abi = 0
```

## Decision

```text
decision:
  Adopt

reason_token:
  WriteSetMapStoreAnyRustOracleParityGateGreen

selected_next_card:
  MIRBUILDER-CARRIER-TYPE-SCALAR-KNOWN-WRITE-SET-MAPSTORE-ANY-POST-ADOPTION-RERUN-001
```

## Guard

```text
tools/checks/
  rust_lifecycle_mirbuilder_carrier_type_scalar_known_write_set_mapstore_any_hako_adoption_decision_guard.sh
```

## Non-Claims

```text
any_write_boundary_declared = 1
any_write_boundary_opened = 0
write_subsurface_selected = 0
write_direct_closeout_materialized = 0
write_result_policy_ready = 0
write_scalar_i64_routes_closeout = 0
scalar_known_transport_axis_closeout = 0
component_specific_direct_contract_materialized = 0
source_selfhost_claim = 0
new_route_authority = 0
behavior_change = 0
runtime_mutation_authority = 0
publication_execution = 0
runtime_fallback = 0
new_backend_route = 0
new_abi = 0
native_seed_materialization = 0
hako_generation = 0
rust_deletion = 0
manual_subsurface_selection = 0
route_count_as_proof = 0
apparent_simplicity_as_proof = 0
accepted_read_contract_similarity_as_proof = 0
```
