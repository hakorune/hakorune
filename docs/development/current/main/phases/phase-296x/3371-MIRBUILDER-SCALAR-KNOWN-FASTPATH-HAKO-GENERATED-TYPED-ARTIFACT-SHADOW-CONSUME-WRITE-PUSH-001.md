# 3371 - MIRBUILDER-SCALAR-KNOWN-FASTPATH-HAKO-GENERATED-TYPED-ARTIFACT-SHADOW-CONSUME-WRITE-PUSH-001

## Token

```text
MIRBUILDER-SCALAR-KNOWN-FASTPATH-HAKO-GENERATED-TYPED-ARTIFACT-SHADOW-CONSUME-WRITE-PUSH-001
```

## Purpose

Connect WriteScalarI64Routes / PushSurfacePolicy to the live Rust fast path
through a checked-in generated typed `.hako` artifact consumed as shadow
evidence.

Rust remains the route authority. The runtime path reads a typed Rust const
generated from the `.hako` policy row; it does not parse `.hako` source text.

## Implementation

```text
generated artifact:
  src/mir/generic_method_route_plan/generated/
    write_push_hako_policy.rs

generator / check source:
  tools/rust_lifecycle/generate_write_push_hako_policy.py

runtime consumer:
  src/mir/generic_method_route_plan/scalar_known_hako_shadow.rs

fast path:
  src/mir/generic_method_route_plan/write_routes.rs
```

## Selected Next

```text
selected_next_card:
  MIRBUILDER-SCALAR-KNOWN-FASTPATH-CONNECTED-CLOSEOUT-INVENTORY-RERUN-003
```

## Claims

```text
generated_typed_hako_artifact_shadow_consumed = 1
checked_in_generated_typed_artifact = 1
runtime_hako_source_text_parsing = 0
write_push_fastpath_shadow_consumed = 1
rust_hako_policy_match = 1
generator_check_guard = 1
rust_authority_retained = 1
```

## Non-Claims

```text
fastpath_connected_closeout = 0
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
  rust_lifecycle_mirbuilder_scalar_known_fastpath_hako_generated_typed_artifact_shadow_consume_write_push_guard.sh
```
