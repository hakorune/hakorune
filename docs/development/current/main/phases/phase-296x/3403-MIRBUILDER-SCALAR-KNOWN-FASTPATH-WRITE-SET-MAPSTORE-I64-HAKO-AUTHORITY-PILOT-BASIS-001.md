# 3403 - MIRBUILDER-SCALAR-KNOWN-FASTPATH-WRITE-SET-MAPSTORE-I64-HAKO-AUTHORITY-PILOT-BASIS-001

## Token

```text
MIRBUILDER-SCALAR-KNOWN-FASTPATH-WRITE-SET-MAPSTORE-I64-HAKO-AUTHORITY-PILOT-BASIS-001
```

## Purpose

Select `SetSurfacePolicy / MapStoreI64` as the first scoped Write surface
`.hako` route-decision authority pilot after read-surface authority closeout.

This is basis-only. It records the proof axis and selects the implementation
card. It does not switch Write route-decision authority yet.

## Proof Axis

```text
ReadSurfaceAuthorityCloseoutPrecedesWriteAuthority
+
TypedScalarWriteBeforeAnyWrite
+
PriorGeneratedTypedArtifactShadowConsumed
+
RustOracleCompatFailFastRetained
```

`MapStoreI64` is selected because it is the typed scalar write boundary inside
SetSurfacePolicy and already has a checked-in generated typed `.hako` artifact
shadow-consumed by the live Rust fast path.

This is not route-count proof, apparent-simplicity proof, source-path proof, or
manual surface selection.

## Shape

```text
surface = SetSurfacePolicy/MapStoreI64
route_kind = MapStoreI64
core_op = MapSet
lowering_tier = ColdFallback
result_class = NoneResult
return_shape = None
value_demand = WriteAny
value_boundary = ScalarI64
publication_policy = NonePublication
effect_class = mutate
mutation_class = MutatesReceiverOrContainer
```

## Decision

```text
decision:
  SelectWriteSetMapStoreI64RouteDecisionAuthorityPilotImplementation

reason_token:
  TypedScalarWriteBoundaryBeforeAnyWrite

selected_next_card:
  MIRBUILDER-SCALAR-KNOWN-FASTPATH-WRITE-SET-MAPSTORE-I64-HAKO-ROUTE-DECISION-AUTHORITY-PILOT-001
```

## Result

```text
write_set_mapstore_i64_hako_authority_pilot_basis = 1
selected_surface = SetSurfacePolicy/MapStoreI64
typed_scalar_write_before_any_write = 1
prior_generated_typed_artifact_shadow_consumed = 1
rust_oracle_compat_fail_fast_retained = 1
basis_only = 1

write_surface_authority_pilot = 0
mapstore_authority = 0
runtime_mutation_authority = 0
publication_execution = 0
scalar_known_hako_runtime_route_authority = 0
source_selfhost_claim = 0
```

## Non-Claims

```text
write_surface_authority_pilot = 0
mapstore_authority = 0
mapdelete_authority = 0
arrayappend_authority = 0
write_mutation_authority = 0
write_publication_authority = 0
runtime_mutation_authority = 0
publication_execution = 0
scalar_known_hako_runtime_route_authority = 0
scalar_known_transport_axis_authority_switch = 0
rust_fastpath_rewired = 0
route_selection_authority_switch = 0
backend_lowering_authority = 0
caller_orientation_runtime_path = 0
build_rs_hako_compiler_invocation = 0
live_hako_authority = 0
new_backend_route = 0
new_abi = 0
runtime_fallback = 0
source_selfhost_claim = 0

route_count_as_proof = 0
apparent_simplicity_as_proof = 0
source_path_as_authority = 0
owner_name_as_proof = 0
route_membership_alone_as_proof = 0
manual_surface_selection = 0
```

## Guard

```text
tools/checks/
  rust_lifecycle_mirbuilder_scalar_known_fastpath_write_set_mapstore_i64_hako_authority_pilot_basis_guard.sh
```
