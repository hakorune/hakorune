# 3367 - MIRBUILDER-SCALAR-KNOWN-FASTPATH-HAKO-GENERATED-TYPED-ARTIFACT-SHADOW-CONSUME-SET-MAPSTORE-ANY-BASIS-001

## Token

```text
MIRBUILDER-SCALAR-KNOWN-FASTPATH-HAKO-GENERATED-TYPED-ARTIFACT-SHADOW-CONSUME-SET-MAPSTORE-ANY-BASIS-001
```

## Purpose

Define the basis for connecting SetSurfacePolicy / MapStoreAny to the live
Rust fast path through a checked-in generated typed `.hako` artifact consumed
as shadow evidence.

This card is basis-only. It does not generate the artifact, does not connect
MapStoreAny in `write_routes`, and does not switch route authority to `.hako`.

## Basis

```text
surface:
  WriteScalarI64Routes / SetSurfacePolicy / MapStoreAny

prior connected surface:
  WriteScalarI64Routes / SetSurfacePolicy / MapStoreI64

proof axis:
  PriorGeneratedTypedArtifactSameSetSurfacePolicyMinimalDeltaV1

next mechanism:
  CheckedInGeneratedTypedHakoArtifactShadowConsume
```

MapStoreAny shares the same SetSurfacePolicy fast-path boundary as the already
connected MapStoreI64 shadow handoff. The new field is the Any write boundary,
which remains declared metadata only.

## Selected Next

```text
selected_next_card:
  MIRBUILDER-SCALAR-KNOWN-FASTPATH-HAKO-GENERATED-TYPED-ARTIFACT-SHADOW-CONSUME-SET-MAPSTORE-ANY-001
```

## Claims

```text
mapstore_any_generated_typed_artifact_shadow_consume_basis = 1
checked_in_generated_typed_artifact_allowed_next = 1
fastpath_shadow_consume_allowed_next = 1
same_set_surface_policy_minimal_delta = 1
basis_only = 1
```

## Non-Claims

```text
generated_typed_hako_artifact_shadow_consumed = 0
checked_in_generated_typed_artifact = 0
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
  rust_lifecycle_mirbuilder_scalar_known_fastpath_hako_generated_typed_artifact_shadow_consume_set_mapstore_any_basis_guard.sh
```
