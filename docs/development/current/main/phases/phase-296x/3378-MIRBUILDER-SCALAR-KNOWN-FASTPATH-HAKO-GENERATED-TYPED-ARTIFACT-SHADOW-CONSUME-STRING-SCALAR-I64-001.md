# 3378 - MIRBUILDER-SCALAR-KNOWN-FASTPATH-HAKO-GENERATED-TYPED-ARTIFACT-SHADOW-CONSUME-STRING-SCALAR-I64-001

## Token

```text
MIRBUILDER-SCALAR-KNOWN-FASTPATH-HAKO-GENERATED-TYPED-ARTIFACT-SHADOW-CONSUME-STRING-SCALAR-I64-001
```

## Purpose

Connect `StringScalarI64Routes` to the live Rust fast path through a checked-in
generated typed `.hako` artifact consumed as shadow evidence.

This card creates the `.hako` policy mirror, checked-in generated Rust artifact,
generator, Rust shadow consumer, and `string_routes` fast-path calls. Rust
remains route authority.

## Implementation

```text
hako source:
  lang/src/compiler/lib/string_search_scalar_i64_policy_classifier.hako

generated artifact:
  src/mir/generic_method_route_plan/generated/string_search_scalar_i64_hako_policy.rs

generator:
  tools/rust_lifecycle/generate_string_search_scalar_i64_hako_policy.py

shadow consumer:
  src/mir/generic_method_route_plan/scalar_known_hako_shadow.rs

fast path:
  src/mir/generic_method_route_plan/string_routes.rs
```

## Shadowed Rows

```text
StringIndexOf:
  proof = IndexOfSurfacePolicy
  core_op = StringIndexOf

StringLastIndexOf:
  proof = LastIndexOfSurfacePolicy
  core_op = StringLastIndexOf

StringContains:
  proof = ContainsSurfacePolicy
  core_op = StringContains

shared:
  lowering = WarmDirectAbi
  return/value/publication/effect = ScalarI64 / ScalarI64 / NoPublication / read
```

## Claims

```text
generated_typed_hako_artifact_shadow_consumed = 1
checked_in_generated_typed_artifact = 1
runtime_hako_source_text_parsing = 0
string_fastpath_shadow_consumed = 1
rust_hako_policy_match = 1
generator_check_guard = 1
rust_authority_retained = 1
```

## Selected Next

```text
selected_next_card:
  MIRBUILDER-SCALAR-KNOWN-FASTPATH-CONNECTED-CLOSEOUT-INVENTORY-RERUN-005
```

## Non-Claims

```text
read_surface_connection_complete = 0
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
  rust_lifecycle_mirbuilder_scalar_known_fastpath_hako_generated_typed_artifact_shadow_consume_string_scalar_i64_guard.sh
```
