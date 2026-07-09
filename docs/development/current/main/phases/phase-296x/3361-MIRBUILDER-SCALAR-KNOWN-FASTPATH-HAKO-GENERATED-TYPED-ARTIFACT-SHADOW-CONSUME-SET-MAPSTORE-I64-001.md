# 3361 - MIRBUILDER-SCALAR-KNOWN-FASTPATH-HAKO-GENERATED-TYPED-ARTIFACT-SHADOW-CONSUME-SET-MAPSTORE-I64-001

## Token

```text
MIRBUILDER-SCALAR-KNOWN-FASTPATH-HAKO-GENERATED-TYPED-ARTIFACT-SHADOW-CONSUME-SET-MAPSTORE-I64-001
```

## Purpose

Replace the transitional `include_str!` / string-search / `split('|')`
MapStoreI64 `.hako` shadow connection with a checked-in generated typed Rust
artifact consumed by the live Rust fast path.

This keeps the 3360 bridge-plan boundary intact:

```text
short-term:
  Rust fast path consumes typed `.hako` artifact as shadow evidence.

long-term:
  `.hako` caller orientation remains the target.

now:
  Rust route authority is retained.
```

## Implementation

```text
generated artifact:
  src/mir/generic_method_route_plan/generated/
    write_set_mapstore_i64_hako_policy.rs

generator / check source:
  tools/rust_lifecycle/generate_write_set_mapstore_i64_hako_policy.py

runtime consumer:
  src/mir/generic_method_route_plan/scalar_known_hako_shadow.rs
```

The runtime consumer now reads `WRITE_SET_MAPSTORE_I64_HAKO_POLICY`, a typed
Rust const generated from the `.hako` policy row. Runtime builds no longer parse
the `.hako` source text for this handoff.

## Guard

```text
tools/checks/
  rust_lifecycle_mirbuilder_scalar_known_fastpath_hako_generated_typed_artifact_shadow_consume_set_mapstore_i64_guard.sh
```

## Selected Next

```text
selected_next_card:
  MIRBUILDER-CARRIER-TYPE-SCALAR-KNOWN-WRITE-SET-MAPSTORE-ANY-WRITE-BOUNDARY-BASIS-001
```

## Claims

```text
generated_typed_hako_artifact_shadow_consumed = 1
checked_in_generated_typed_artifact = 1
runtime_hako_source_text_parsing = 0
include_str_split_shadow_parsing_removed = 1
mapstore_i64_fastpath_shadow_still_consumed = 1
rust_hako_policy_match = 1
generator_check_guard = 1
rust_authority_retained = 1
```

## Non-Claims

```text
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
