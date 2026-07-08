# 3348 - MIRBUILDER-SCALAR-KNOWN-FASTPATH-HAKO-SHADOW-CONSUME-SET-MAPSTORE-I64-001

## Token

```text
MIRBUILDER-SCALAR-KNOWN-FASTPATH-HAKO-SHADOW-CONSUME-SET-MAPSTORE-I64-001
```

## Purpose

Connect the scoped `SetSurfacePolicy / MapStoreI64` `.hako` classifier mirror to
the Rust fast-path decision point as a shadow-consumed artifact.

Rust still chooses the route. The `.hako` artifact is read by the Rust route
planner and compared against the already-owned Rust decision tuple.

## Implementation

```text
lang/src/compiler/lib/write_set_mapstore_i64_policy_classifier.hako
  -> include_str! in scalar_known_hako_shadow.rs
  -> write_routes.rs MapStoreI64 branch
  -> compare .hako policy row with Rust decision tuple
  -> return the existing Rust GenericMethodRouteDecision
```

The Rust contract boundary is also consumed through
`candidate_scalar_known_surfaces` to keep the shadow check tied to the existing
ScalarKnown Write contract box.

## Result

```text
hako_artifact_fastpath_shadow_consumed = 1
surface = SetSurfacePolicy/MapStoreI64
rust_fastpath_decision_observed = 1
hako_policy_result_observed = 1
rust_hako_policy_match = 1
mismatch_policy = fail_fast_guard
rust_authority_retained = 1
source_selfhost_claim = 0
```

## Guard

```text
tools/checks/
  rust_lifecycle_mirbuilder_scalar_known_fastpath_hako_shadow_consume_set_mapstore_i64_guard.sh
```

## Selected Next

```text
selected_next_card:
  MIRBUILDER-SCALAR-KNOWN-FASTPATH-HAKO-SHADOW-CONSUME-MISMATCH-GUARD-EXPANSION-001
```

## Non-Claims

```text
hako_runtime_route_authority = 0
hako_backend_lowering_authority = 0
route_selection_authority = 0
runtime_mutation_authority = 0
publication_execution = 0
runtime_fallback = 0
new_backend_route = 0
new_abi = 0
source_selfhost_claim = 0
```
