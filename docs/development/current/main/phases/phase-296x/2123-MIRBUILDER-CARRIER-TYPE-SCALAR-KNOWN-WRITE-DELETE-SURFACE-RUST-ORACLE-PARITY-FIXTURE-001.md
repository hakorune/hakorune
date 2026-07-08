# 2123 - MIRBUILDER-CARRIER-TYPE-SCALAR-KNOWN-WRITE-DELETE-SURFACE-RUST-ORACLE-PARITY-FIXTURE-001

## Token

```text
MIRBUILDER-CARRIER-TYPE-SCALAR-KNOWN-WRITE-DELETE-SURFACE-RUST-ORACLE-PARITY-FIXTURE-001
```

## Purpose

Freeze the Rust oracle parity fixture for the next narrow Write `.hako`
implementation pilot: `DeleteSurfacePolicy / MapDeleteAny`.

This card consumes the consultation decision to select Delete by
`PriorHakoAdoptedWriteSubsurfaceMinimalSemanticDelta`, not by route count,
apparent simplicity, or manual sub-surface selection.

## Proof Axis

```text
prior_hako_adopted_write_subsurface_minimal_semantic_delta = 1
prior_subsurface = PushSurfacePolicy
selected_subsurface = DeleteSurfacePolicy
stable_scalar_i64_result_preserved = 1
mutate_effect_metadata_boundary_reused = 1
new_policy_dimension = NonePublication
typed_non_typed_write_split = 0
route_count_as_proof = 0
apparent_simplicity_as_proof = 0
accepted_read_contract_similarity_as_proof = 0
manual_subsurface_selection = 0
```

## Oracle Fixture

```text
fixture_id = WriteDeleteSurfaceRustOracleV0
row_count = 1

case_id = map_delete_any_delete_surface
subsurface_id = DeleteSurfacePolicy
route_kind = MapDeleteAny
proof_or_policy_source = DeleteSurfacePolicy
core_method_op = MapDelete
core_method_lowering_tier = ColdFallback
result_class = ScalarI64Result
return_shape = ScalarI64
value_demand = WriteAny
publication_policy = NonePublication
effect_class = mutate
mutation_class = MutatesReceiverOrContainer
hako_role = classifier_policy_mirror_only
```

## Metadata Boundary

```text
none_publication_metadata_declared = 1
publication_execution = 0
mutate_effect_boundary_reused = 1
runtime_mutation_authority = 0
hako_implementation_mirrors_classifier_policy_decision = 1
```

## Result

```text
write_delete_surface_hako_implementation_candidate = 1
delete_surface_policy_scope = 1
map_delete_any_scope = 1
rust_oracle_fixture_defined = 1
next_hako_parity_pilot_selected = 1

write_direct_closeout_materialized = 0
write_scalar_i64_routes_closeout = 0
scalar_known_transport_axis_closeout = 0
runtime_mutation_authority = 0
source_selfhost_claim = 0

decision:
  SelectWriteDeleteSurfaceHakoParityPilot

selected_next_card:
  MIRBUILDER-CARRIER-TYPE-SCALAR-KNOWN-WRITE-DELETE-SURFACE-HAKO-PARITY-PILOT-001
```

## Guard

```text
tools/checks/
  rust_lifecycle_mirbuilder_carrier_type_scalar_known_write_delete_surface_rust_oracle_parity_fixture_guard.sh
```

## Output

```text
fixture:
  docs/development/current/main/design/fixtures/rust-lifecycle/
    mirbuilder-carrier-type-scalar-known-write-delete-surface-rust-oracle-v0.json

tool:
  tools/rust_lifecycle/
    mirbuilder_carrier_type_scalar_known_write_delete_surface_rust_oracle_parity_fixture.py
```

## Non-Claims

```text
write_subsurface_selected_for_closeout = 0
write_direct_closeout_materialized = 0
write_scalar_i64_routes_closeout = 0
scalar_known_transport_axis_closeout = 0
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
source_path_as_authority = 0
owner_name_as_proof = 0
route_membership_alone_as_proof = 0
```
