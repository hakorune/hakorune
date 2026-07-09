# 3374 - MIRBUILDER-SCALAR-KNOWN-FASTPATH-HAKO-GENERATED-TYPED-ARTIFACT-BASIS-MAPLOAD-SCALAR-I64-001

## Token

```text
MIRBUILDER-SCALAR-KNOWN-FASTPATH-HAKO-GENERATED-TYPED-ARTIFACT-BASIS-MAPLOAD-SCALAR-I64-001
```

## Purpose

Define the basis for connecting `MapLoadScalarI64Routes` to the live Rust fast
path through a checked-in generated typed `.hako` artifact consumed as shadow
evidence.

This card is basis-only. It does not create the `.hako` source, does not create
the generated Rust artifact, does not connect `collection_read_routes`, and does
not switch route authority to `.hako`.

## Basis

```text
surface:
  MapLoadScalarI64Routes

route:
  MapLoadScalarI64

core op:
  MapGet

lowering:
  WarmDirectAbi

return/value/publication/effect:
  ScalarI64OrMissingZero / ScalarI64 / NoPublication / read

proof family:
  ScalarI64MapGetStoreFact

allowed existing proofs:
  MapSetScalarI64SameKeyNoEscape
  MapSetScalarI64DominatesNoEscape
  MapSetScalarI64CoveredDynamicI64KeyNoEscape

next mechanism:
  CheckedInGeneratedTypedHakoArtifactShadowConsume
```

## Selected Next

```text
selected_next_card:
  MIRBUILDER-SCALAR-KNOWN-FASTPATH-HAKO-GENERATED-TYPED-ARTIFACT-SHADOW-CONSUME-MAPLOAD-SCALAR-I64-001
```

## Claims

```text
mapload_scalar_i64_generated_typed_artifact_basis = 1
checked_in_generated_typed_artifact_allowed_next = 1
fastpath_shadow_consume_allowed_next = 1
basis_only = 1
```

## Non-Claims

```text
generated_typed_hako_artifact_created = 0
generated_typed_hako_artifact_shadow_consumed = 0
mapload_fastpath_shadow_consumed = 0
read_surface_connection_complete = 0
fastpath_connected_closeout = 0
runtime_hako_source_text_parsing = 0
hako_runtime_route_authority = 0
rust_fastpath_rewired = 0
route_selection_authority_switch = 0
backend_lowering_authority = 0
runtime_mutation_authority = 0
publication_execution = 0
build_rs_hako_compiler_invocation = 0
live_hako_authority = 0
caller_orientation_runtime_path = 0
new_backend_route = 0
new_abi = 0
runtime_fallback = 0
source_selfhost_claim = 0
```

## Guard

```text
tools/checks/
  rust_lifecycle_mirbuilder_scalar_known_fastpath_hako_generated_typed_artifact_basis_mapload_scalar_i64_guard.sh
```
