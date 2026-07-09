# 3370 - MIRBUILDER-SCALAR-KNOWN-FASTPATH-HAKO-GENERATED-TYPED-ARTIFACT-SHADOW-CONSUME-WRITE-PUSH-BASIS-001

## Token

```text
MIRBUILDER-SCALAR-KNOWN-FASTPATH-HAKO-GENERATED-TYPED-ARTIFACT-SHADOW-CONSUME-WRITE-PUSH-BASIS-001
```

## Purpose

Define the basis for connecting WriteScalarI64Routes / PushSurfacePolicy to the
live Rust fast path through a checked-in generated typed `.hako` artifact
consumed as shadow evidence.

This card is basis-only. It does not generate the artifact, does not connect
Push in `write_routes`, and does not switch route authority to `.hako`.

## Basis

```text
surface:
  WriteScalarI64Routes / PushSurfacePolicy

route:
  ArrayAppendAny

prior connected surfaces:
  WriteScalarI64Routes / SetSurfacePolicy / MapStoreI64
  WriteScalarI64Routes / SetSurfacePolicy / MapStoreAny

proof axis:
  PriorWriteRouteGeneratedTypedArtifactContinuationV1

next mechanism:
  CheckedInGeneratedTypedHakoArtifactShadowConsume
```

## Selected Next

```text
selected_next_card:
  MIRBUILDER-SCALAR-KNOWN-FASTPATH-HAKO-GENERATED-TYPED-ARTIFACT-SHADOW-CONSUME-WRITE-PUSH-001
```

## Claims

```text
write_push_generated_typed_artifact_shadow_consume_basis = 1
checked_in_generated_typed_artifact_allowed_next = 1
fastpath_shadow_consume_allowed_next = 1
prior_write_route_generated_typed_artifact_continuation = 1
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
  rust_lifecycle_mirbuilder_scalar_known_fastpath_hako_generated_typed_artifact_shadow_consume_write_push_basis_guard.sh
```
