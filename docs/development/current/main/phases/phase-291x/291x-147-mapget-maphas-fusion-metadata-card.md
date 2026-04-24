---
Status: Landed
Date: 2026-04-24
Scope: Add metadata-only same-key MapGet/MapHas fusion preflight routes.
Related:
  - docs/development/current/main/CURRENT_STATE.toml
  - docs/development/current/main/design/hotline-core-method-contract-ssot.md
  - docs/development/current/main/phases/phase-291x/291x-144-mapget-preheader-scalar-proof-card.md
  - docs/development/current/main/phases/phase-291x/291x-145-mapget-scalar-lowering-probe-card.md
  - docs/development/current/main/phases/phase-291x/291x-146-mapget-owner-seam-selection-card.md
  - src/mir/generic_method_route_plan.rs
  - src/mir/map_lookup_fusion_plan.rs
  - src/runner/mir_json_emit/root.rs
---

# 291x-147 MapGet/MapHas Fusion Metadata Card

## Goal

Land the smallest compiler-owned fact after the rejected scalar MapGet helper
probe:

```text
MapGet(i64_const key)
MapHas(i64_const same key)
```

This card is metadata-only. It does not lower the pair and it does not change
`.inc`, Rust runtime helpers, or generated code.

## Design

`map_lookup_fusion_routes` is derived from `generic_method_routes`.

A route is emitted only when MIR already proves:

```text
get.core_method = MapGet
has.core_method = MapHas
receiver_origin_box = MapBox
same receiver root
key_route = i64_const
same i64 const key
get_return_shape = scalar_i64_or_missing_zero
get_value_demand = scalar_i64
get_publication_policy = no_publication
same block, get before has
no same-receiver mutation/escape between get and has
```

The route records:

```text
fusion_op = MapLookupSameKey
get_result_value
has_result_value
has_result_shape = presence_bool
stored_value_proof = scalar_i64_nonzero | scalar_i64_const | unknown_scalar
stored_value_const
stored_value_known_nonzero
lowering_tier = cold_fallback
```

## Boundary

- No codegen changes.
- No `.inc` method-name reader or fusion scanner.
- No new runtime helper.
- No native i64-key storage lane.
- No direct `slot_load_hi`.
- No lowering unless a later card proves cycles/IPC improvement.
- Keep H144 scalar MapGet proof as the input fact.

## Implementation

- Added `src/mir/map_lookup_fusion_plan.rs` as the MIR owner for
  `MapLookupSameKey` preflight metadata.
- Added `FunctionMetadata::map_lookup_fusion_routes`.
- Refreshed fusion metadata immediately after `generic_method_routes` in
  `refresh_function_semantic_metadata`.
- Extended MIR JSON emission with `map_lookup_fusion_routes`.
- Kept route JSON construction outside the main metadata `json!` macro to avoid
  growing the already-large root emitter macro.
- Refactored the scalar MapGet proof to expose the proven stored i64 value to
  the fusion metadata pass without changing the existing route decision.

## Observed Metadata

`bench_kilo_leaf_map_getset_has.hako` now emits:

```text
route_id = map_lookup.same_key
fusion_op = MapLookupSameKey
block = 19
get_instruction_index = 3
has_instruction_index = 4
receiver_origin_box = MapBox
key_route = i64_const
key_const = -1
get_return_shape = scalar_i64_or_missing_zero
get_value_demand = scalar_i64
get_publication_policy = no_publication
has_result_shape = presence_bool
stored_value_const = 1
stored_value_known_nonzero = true
stored_value_proof = scalar_i64_nonzero
proof = same_receiver_same_i64_key_scalar_get_has
lowering_tier = cold_fallback
```

This proves the measured get/has pair is visible to MIR as one typed metadata
route while generated code still follows the existing calls.

## Acceptance

- MIR unit tests detect a same-block scalar get/has pair.
- MIR unit tests reject a different-key has pair.
- MIR JSON emits the route fields, including get/has result values and stored
  value nonzero state.
- `bench_kilo_leaf_map_getset_has.hako` carries the fusion metadata.
- `cargo check -q` passes.
- Codegen remains unchanged.

## Next

Next card may probe a fusion lowering only from this metadata:

```text
MapLookupSameKey + scalar_i64_or_missing_zero + presence_bool
```

Keeper criteria remain stricter than instruction count:

```text
cycles improve
IPC does not collapse
top owner family changes or shrinks
RuntimeData mixed semantics remain unchanged
```
