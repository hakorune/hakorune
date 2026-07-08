# 2117 - MIRBUILDER-CARRIER-TYPE-SCALAR-KNOWN-WRITE-PUSH-SURFACE-PARITY-GATE-001

## Token

```text
MIRBUILDER-CARRIER-TYPE-SCALAR-KNOWN-WRITE-PUSH-SURFACE-PARITY-GATE-001
```

## Purpose

Add the Rust-oracle parity gate for the hand-authored
`write_push_surface_policy_classifier` `.hako` implementation.

This card proves the 2116 `.hako` classifier output matches the 2115 Rust
oracle fixture for the scoped `PushSurfacePolicy / ArrayAppendAny` pilot.

## Gate

```text
tools/checks/
  rust_lifecycle_mirbuilder_carrier_type_scalar_known_write_push_surface_parity_gate.sh
```

## Evidence

```text
rust_oracle_fixture:
  docs/development/current/main/design/fixtures/rust-lifecycle/
    mirbuilder-carrier-type-scalar-known-write-push-surface-hako-parity-pilot-v0.json

hako_source:
  lang/src/compiler/lib/write_push_surface_policy_classifier.hako
```

## Acceptance

```text
output_contract =
  rust-lifecycle-mirbuilder-carrier-type-scalar-known-write-push-surface-parity-gate-v0

parity_rows = 1
parity_status = green
write_direct_closeout_materialized = 0
write_result_policy_ready = 0
write_scalar_i64_routes_closeout = 0
scalar_known_transport_axis_closeout = 0
source_selfhost_claim = 0
hako_adopted_decision = 0
runtime_mutation_authority = 0
runtime_fallback = 0
new_backend_route = 0
new_abi = 0
```

## Decision

```text
decision:
  SelectHakoAdoptionDecision

reason_token:
  WritePushSurfaceParityGateGreen

selected_next_card:
  MIRBUILDER-CARRIER-TYPE-SCALAR-KNOWN-WRITE-PUSH-SURFACE-HAKO-ADOPTION-DECISION-001
```

## Non-Claims

```text
write_subsurface_selected = 0
write_direct_closeout_materialized = 0
write_result_policy_ready = 0
write_scalar_i64_routes_closeout = 0
scalar_known_transport_axis_closeout = 0
component_specific_direct_contract_materialized = 0
hako_adoption = 0
source_selfhost_claim = 0
new_route_authority = 0
behavior_change = 0
runtime_mutation_authority = 0
runtime_fallback = 0
new_backend_route = 0
new_abi = 0
native_seed_materialization = 0
hako_generation = 0
hako_adopted_decision = 0
manual_subsurface_selection = 0
route_count_as_proof = 0
apparent_simplicity_as_proof = 0
accepted_read_contract_similarity_as_proof = 0
```
