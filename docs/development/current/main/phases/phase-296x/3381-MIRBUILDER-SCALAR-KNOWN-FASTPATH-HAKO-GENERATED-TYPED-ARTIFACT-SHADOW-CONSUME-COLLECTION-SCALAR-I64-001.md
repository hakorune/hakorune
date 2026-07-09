# 3381 - MIRBUILDER-SCALAR-KNOWN-FASTPATH-HAKO-GENERATED-TYPED-ARTIFACT-SHADOW-CONSUME-COLLECTION-SCALAR-I64-001

## Token

```text
MIRBUILDER-SCALAR-KNOWN-FASTPATH-HAKO-GENERATED-TYPED-ARTIFACT-SHADOW-CONSUME-COLLECTION-SCALAR-I64-001
```

## Purpose

Connect `CollectionScalarI64Routes` to the live Rust fast path through a
checked-in generated typed `.hako` artifact consumed as shadow evidence.

This card creates the `.hako` policy mirror, checked-in generated Rust artifact,
generator, Rust shadow consumer, and `collection_read_routes` fast-path call for
the len/length/size family. Rust remains route authority.

## Shadowed Rows

```text
MapEntryCount  -> MapLen
ArraySlotLen   -> ArrayLen
StringLen      -> StringLen
AnyLength      -> AnyLen

shared:
  proof = LenSurfacePolicy
  lowering = WarmDirectAbi
  return/value/publication/effect = ScalarI64 / ScalarI64 / NoPublication / observe
```

## Claims

```text
generated_typed_hako_artifact_shadow_consumed = 1
checked_in_generated_typed_artifact = 1
runtime_hako_source_text_parsing = 0
collection_fastpath_shadow_consumed = 1
rust_hako_policy_match = 1
generator_check_guard = 1
rust_authority_retained = 1
read_surface_connection_complete = 1
```

## Selected Next

```text
selected_next_card:
  MIRBUILDER-SCALAR-KNOWN-FASTPATH-CONNECTED-CLOSEOUT-INVENTORY-RERUN-006
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
