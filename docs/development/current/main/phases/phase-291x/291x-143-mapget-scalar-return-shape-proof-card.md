---
Status: Landed
Date: 2026-04-24
Scope: Add a narrow MapGet scalar return-shape proof without changing codegen.
Related:
  - docs/development/current/main/CURRENT_STATE.toml
  - docs/development/current/main/design/hotline-core-method-contract-ssot.md
  - docs/development/current/main/phases/phase-291x/291x-142-mapget-return-shape-metadata-card.md
  - src/mir/generic_method_route_plan.rs
  - src/mir/generic_method_route_facts.rs
  - src/runner/mir_json_emit/root.rs
---

# 291x-143 MapGet Scalar Return-Shape Proof Card

## Goal

Prove the first conservative `MapGet` scalar return shape in MIR metadata.
This is a proof preflight keeper, not a lowering keeper.

## Design

`RuntimeDataBox.get(key)` may publish scalar metadata only when the current MIR
block proves all of the following locally:

```text
receiver_origin_box = MapBox
key_route = i64_const
same receiver root
same i64 const key
dominating same-block MapBox.set / RuntimeDataBox.set
stored value is scalar i64
no same-receiver mutation between set and get
no same-receiver escape between set and get
```

When the proof succeeds, route metadata becomes:

```text
return_shape = scalar_i64_or_missing_zero
value_demand = scalar_i64
publication_policy = no_publication
proof = map_set_scalar_i64_same_key_no_escape
```

The route still uses:

```text
route_kind = runtime_data_load_any
helper_symbol = nyash.runtime_data.get_hh
lowering_tier = cold_fallback
```

## Boundary

- Codegen stays unchanged.
- `ny_main` must still call `nyash.runtime_data.get_hh`.
- No direct `nyash.map.slot_load_hi`.
- No `nyash.map.runtime_load_hi`.
- No `.inc` MapGet proof reader in this card.
- No cross-block proof.
- No loop-carried proof.
- No alias-aware proof.
- No object / borrowed-alias / handle-return promotion.

## Implementation

- Added `GenericMethodReturnShape::ScalarI64OrMissingZero`.
- Added `GenericMethodValueDemand::ScalarI64`.
- Added `GenericMethodPublicationPolicy::NoPublication`.
- Added `GenericMethodRouteProof::MapSetScalarI64SameKeyNoEscape`.
- Added MIR-side same-block proof for:
  `MapBox.set` / `RuntimeDataBox.set` -> `RuntimeDataBox.get`.
- Kept `route_kind=runtime_data_load_any` and
  `helper_symbol=nyash.runtime_data.get_hh`.
- Did not modify `.inc` lowering.

## Observed Metadata

Focused MIR route tests now prove the same-block scalar case as:

```text
route_id = generic_method.get
box_name = RuntimeDataBox
method = get
receiver_origin_box = MapBox
key_route = i64_const
proof = map_set_scalar_i64_same_key_no_escape
route_kind = runtime_data_load_any
helper_symbol = nyash.runtime_data.get_hh
return_shape = scalar_i64_or_missing_zero
value_demand = scalar_i64
publication_policy = no_publication
lowering_tier = cold_fallback
```

For `bench_kilo_leaf_map_getset_has.hako`, metadata remains conservative
because the landed proof is same-block only and the dominating store is in the
preheader:

```text
proof = get_surface_policy
return_shape = mixed_runtime_i64_or_handle
value_demand = runtime_i64_or_handle
publication_policy = runtime_data_facade
helper_symbol = nyash.runtime_data.get_hh
```

## Rejection

The proof must reject when:

- the receiver is not proven to originate from `MapBox`
- the key is not an i64 const
- the same-key dominating store is missing
- the same-key stored value is not scalar i64
- any same-receiver mutation appears between store and get
- the receiver escapes to an unknown call between store and get

## Why

`MapGet` cannot leave the RuntimeData facade based on key shape alone.
The missing fact is the stored value return shape. This card records the
smallest safe proof in MIR so a later lowering card can consume metadata
instead of asking `.inc` or Rust to rediscover semantics.

## Acceptance

- Focused MIR route tests cover scalar proof success and reject cases.
- MIR JSON emits the scalar proof fields for a same-block scalar case.
- Existing `bench_kilo_leaf_map_getset_has.hako` remains codegen-compatible.
- `.inc` lowering remains unchanged.
- Perf/codegen checks show no lowering change.

## Next

After this proof exists, a separate H144 lowering card may try:

```text
MapGet + i64_const key + scalar_i64_or_missing_zero + scalar_i64 demand
  -> scalar MapGet ABI route
```

That card must still prove cycles/IPC improvement before becoming keeper.
