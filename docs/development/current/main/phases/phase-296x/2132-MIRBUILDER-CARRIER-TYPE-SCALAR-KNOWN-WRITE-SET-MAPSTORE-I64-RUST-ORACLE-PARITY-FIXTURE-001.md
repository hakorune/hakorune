# 2132 - MIRBUILDER-CARRIER-TYPE-SCALAR-KNOWN-WRITE-SET-MAPSTORE-I64-RUST-ORACLE-PARITY-FIXTURE-001

## Token

```text
MIRBUILDER-CARRIER-TYPE-SCALAR-KNOWN-WRITE-SET-MAPSTORE-I64-RUST-ORACLE-PARITY-FIXTURE-001
```

## Purpose

Freeze the Rust oracle row for the first split Set Write `.hako` pilot:
`SetSurfacePolicy / MapStoreI64`.

This card consumes the typed-value split basis. It selects the MapStoreI64
`.hako` parity pilot, but does not implement the pilot, adopt it, or materialize
direct closeout.

## Oracle Row

```text
case_id = map_store_i64_set_surface
subsurface_id = SetSurfacePolicy
route_kind = MapStoreI64
proof_or_policy_source = SetSurfacePolicy
core_method_op = MapSet
core_method_lowering_tier = ColdFallback
result_class = NoneResult
return_shape = None
value_demand = WriteAny
write_value_boundary = ScalarI64
publication_policy = NonePublication
effect_class = mutate
mutation_class = MutatesReceiverOrContainer
hako_role = classifier_policy_mirror_only
```

## Result

```text
write_set_mapstore_i64_hako_implementation_candidate = 1
set_surface_policy_scope = 1
mapstore_i64_scope = 1
typed_scalar_write_before_any_write = 1
mapstore_any_deferred = 1
none_result_metadata_declared = 1
none_publication_metadata_reused = 1
mutate_effect_metadata_boundary_reused = 1
rust_oracle_fixture_defined = 1
next_hako_parity_pilot_selected = 1

any_write_boundary_opened = 0
write_direct_closeout_materialized = 0
write_scalar_i64_routes_closeout = 0
scalar_known_transport_axis_closeout = 0
runtime_mutation_authority = 0
publication_execution = 0
source_selfhost_claim = 0

selected_next_card:
  MIRBUILDER-CARRIER-TYPE-SCALAR-KNOWN-WRITE-SET-MAPSTORE-I64-HAKO-PARITY-PILOT-001
```

## Guard

```text
tools/checks/
  rust_lifecycle_mirbuilder_carrier_type_scalar_known_write_set_mapstore_i64_rust_oracle_parity_fixture_guard.sh
```

## Non-Claims

```text
any_write_boundary_opened = 0
mapstore_any_hako_pilot_selected = 0
write_subsurface_selected_for_closeout = 0
write_direct_closeout_materialized = 0
write_scalar_i64_routes_closeout = 0
scalar_known_transport_axis_closeout = 0
hako_adoption = 0
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
hako_adopted_decision = 0
manual_subsurface_selection = 0
route_count_as_proof = 0
apparent_simplicity_as_proof = 0
accepted_read_contract_similarity_as_proof = 0
source_path_as_authority = 0
owner_name_as_proof = 0
route_membership_alone_as_proof = 0
```
