# 2119 - MIRBUILDER-CARRIER-TYPE-SCALAR-KNOWN-WRITE-SURFACE-POST-PUSH-ADOPTION-RERUN-001

## Token

```text
MIRBUILDER-CARRIER-TYPE-SCALAR-KNOWN-WRITE-SURFACE-POST-PUSH-ADOPTION-RERUN-001
```

## Purpose

Rerun the Write surface selector after the narrow Push `.hako` adoption
decision.

This card consumes the adopted `PushSurfacePolicy / ArrayAppendAny` parity
pilot and selects the next basis-only direct closeout contract card for Push.
It does not materialize direct closeout and does not close WriteScalarI64Routes
or ScalarKnownTransportAxis.

## Rerun Result

```text
hako_adopted_write_subsurface_count = 1
basis_selection_eligible_subsurface_count = 1
selected_write_subsurface = PushSurfacePolicy

selected_next_card:
  MIRBUILDER-CARRIER-TYPE-SCALAR-KNOWN-WRITE-PUSH-SURFACE-TYPED-DIRECT-CLOSEOUT-CONTRACT-BASIS-001
```

## Guard

```text
tools/checks/
  rust_lifecycle_mirbuilder_carrier_type_scalar_known_write_surface_post_push_adoption_rerun_guard.sh
```

## Output

```text
fixture:
  docs/development/current/main/design/fixtures/rust-lifecycle/
    mirbuilder-carrier-type-scalar-known-write-surface-post-push-adoption-rerun-v0.json

tool:
  tools/rust_lifecycle/
    mirbuilder_carrier_type_scalar_known_write_surface_post_push_adoption_rerun.py
```

## Non-Claims

```text
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
manual_subsurface_selection = 0
route_count_as_proof = 0
apparent_simplicity_as_proof = 0
accepted_read_contract_similarity_as_proof = 0
```
