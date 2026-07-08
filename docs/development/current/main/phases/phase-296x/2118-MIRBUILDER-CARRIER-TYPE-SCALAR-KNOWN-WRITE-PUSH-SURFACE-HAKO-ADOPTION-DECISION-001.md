# 2118 - MIRBUILDER-CARRIER-TYPE-SCALAR-KNOWN-WRITE-PUSH-SURFACE-HAKO-ADOPTION-DECISION-001

## Token

```text
MIRBUILDER-CARRIER-TYPE-SCALAR-KNOWN-WRITE-PUSH-SURFACE-HAKO-ADOPTION-DECISION-001
```

## Purpose

Adopt `write_push_surface_policy_classifier` as a narrow `.hako` parity pilot
owner after the green 1-row Rust-oracle parity gate.

This decision adopts only the `PushSurfacePolicy / ArrayAppendAny`
classifier/policy mirror. It does not adopt WriteScalarI64Routes direct
closeout, ScalarKnownTransportAxis closeout, runtime mutation authority, Source
Selfhost, or full MirBuilder conversion.

## Evidence

```text
hako_source:
  lang/src/compiler/lib/write_push_surface_policy_classifier.hako

parity_gate:
  tools/checks/
    rust_lifecycle_mirbuilder_carrier_type_scalar_known_write_push_surface_parity_gate.sh

adoption_fixture:
  docs/development/current/main/design/fixtures/rust-lifecycle/
    mirbuilder-carrier-type-scalar-known-write-push-surface-hako-adoption-decision-v0.json
```

## Acceptance

```text
parity_gate = green
parity_rows = 1
decision = Adopt
write_push_surface_hako_adopted = 1
hako_adopted_decision = 1
rust_bootstrap_retained = 1
rust_oracle_retained = 1

write_direct_closeout_materialized = 0
write_result_policy_ready = 0
write_scalar_i64_routes_closeout = 0
scalar_known_transport_axis_closeout = 0
source_selfhost_claim = 0
runtime_mutation_authority = 0
runtime_fallback = 0
new_backend_route = 0
new_abi = 0
```

## Decision

```text
decision:
  Adopt

reason_token:
  WritePushSurfaceRustOracleParityGateGreen

selected_next_card:
  MIRBUILDER-CARRIER-TYPE-SCALAR-KNOWN-WRITE-SURFACE-POST-PUSH-ADOPTION-RERUN-001
```

## Guard

```text
tools/checks/
  rust_lifecycle_mirbuilder_carrier_type_scalar_known_write_push_surface_hako_adoption_decision_guard.sh
```

## Non-Claims

```text
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
