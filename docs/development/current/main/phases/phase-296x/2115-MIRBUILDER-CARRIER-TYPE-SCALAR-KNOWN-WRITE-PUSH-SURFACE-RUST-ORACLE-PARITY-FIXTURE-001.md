# 2115 - MIRBUILDER-CARRIER-TYPE-SCALAR-KNOWN-WRITE-PUSH-SURFACE-RUST-ORACLE-PARITY-FIXTURE-001

## Token

```text
MIRBUILDER-CARRIER-TYPE-SCALAR-KNOWN-WRITE-PUSH-SURFACE-RUST-ORACLE-PARITY-FIXTURE-001
```

## Purpose

Freeze the Rust oracle parity fixture for the first narrow Write `.hako`
implementation pilot: `PushSurfacePolicy / ArrayAppendAny`.

This card consumes the consultation decision to treat Push as a scoped
implementation pilot, not as direct closeout authority. It fixes the mutation
boundary as classifier/policy metadata and selects the `.hako` parity pilot as
the next task.

## Oracle Fixture

```text
fixture_id = WritePushSurfaceRustOracleV0
row_count = 1

case_id = array_append_any_push_surface
subsurface_id = PushSurfacePolicy
route_kind = ArrayAppendAny
proof_or_policy_source = PushSurfacePolicy
core_method_op = ArrayPush
core_method_lowering_tier = ColdFallback
result_class = ScalarI64Result
return_shape = ScalarI64
value_demand = WriteAny
publication_policy = NoPublication
effect_class = mutate
mutation_class = MutatesReceiverOrContainer
hako_role = classifier_policy_mirror_only
```

## Mutation Boundary

```text
mutate_effect_boundary_declared = 1
runtime_mutation_authority = 0
hako_implementation_mirrors_classifier_policy_decision = 1
receiver_or_container_mutation_observed_as_metadata = 1
```

## Result

```text
write_push_surface_hako_implementation_candidate = 1
push_surface_policy_scope = 1
array_append_any_scope = 1
rust_oracle_fixture_defined = 1
stable_scalar_i64_result_observed = 1
no_publication_observed = 1
mutate_effect_boundary_declared = 1
hako_implementation_candidate = 1
basis_only_or_fixture_only = 1
next_hako_parity_pilot_selected = 1

write_direct_closeout_materialized = 0
write_result_policy_ready = 0
write_scalar_i64_routes_closeout = 0
scalar_known_transport_axis_closeout = 0

decision:
  SelectWritePushSurfaceHakoParityPilot

selected_next_card:
  MIRBUILDER-CARRIER-TYPE-SCALAR-KNOWN-WRITE-PUSH-SURFACE-HAKO-PARITY-PILOT-001
```

## Guard

```text
tools/checks/rust_lifecycle_mirbuilder_carrier_type_scalar_known_write_push_surface_rust_oracle_parity_fixture_guard.sh
```

## Output

```text
fixture:
  docs/development/current/main/design/fixtures/rust-lifecycle/
    mirbuilder-carrier-type-scalar-known-write-push-surface-rust-oracle-v0.json

tool:
  tools/rust_lifecycle/
    mirbuilder_carrier_type_scalar_known_write_push_surface_rust_oracle_parity_fixture.py

guard:
  tools/checks/
    rust_lifecycle_mirbuilder_carrier_type_scalar_known_write_push_surface_rust_oracle_parity_fixture_guard.sh
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
