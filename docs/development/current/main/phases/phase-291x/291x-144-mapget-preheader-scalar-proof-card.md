---
Status: Landed
Date: 2026-04-24
Scope: Add a narrow dominating/preheader MapGet scalar return-shape proof without changing codegen.
Related:
  - docs/development/current/main/CURRENT_STATE.toml
  - docs/development/current/main/design/hotline-core-method-contract-ssot.md
  - docs/development/current/main/phases/phase-291x/291x-142-mapget-return-shape-metadata-card.md
  - docs/development/current/main/phases/phase-291x/291x-143-mapget-scalar-return-shape-proof-card.md
  - src/mir/generic_method_route_plan.rs
  - src/runner/mir_json_emit/root.rs
---

# 291x-144 MapGet Preheader Scalar Proof Card

## Goal

Extend the MapGet scalar return-shape proof from same-block only to the
measured preheader-store / loop-body-get shape.

This is still a proof metadata keeper, not a lowering keeper.

## Design

`RuntimeDataBox.get(key)` may publish scalar metadata from a dominating
preheader store only when MIR proves all of the following:

```text
receiver_origin_box = MapBox
key_route = i64_const
same receiver root
same i64 const key
dominating MapBox.set / RuntimeDataBox.set
stored value is scalar i64
no same-receiver mutation in dominated instructions after the store
no same-receiver escape in dominated instructions after the store
```

When the proof succeeds, route metadata becomes:

```text
return_shape = scalar_i64_or_missing_zero
value_demand = scalar_i64
publication_policy = no_publication
proof = map_set_scalar_i64_dominates_no_escape
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
- No scalar MapGet ABI helper in this card.
- No `.inc` MapGet proof reader in this card.
- No path-sensitive alias analysis.
- No object / borrowed-alias / handle-return promotion.

## Implementation

- Added `GenericMethodRouteProof::MapSetScalarI64DominatesNoEscape`.
- Kept the same-block proof as the first and more precise proof path.
- Added a conservative dominator-based proof for scalar `MapBox.set` /
  `RuntimeDataBox.set` that dominates a later `RuntimeDataBox.get`.
- Rejects the proof if any dominated instruction after the candidate store may
  mutate or escape the same receiver root.
- Kept `route_kind=runtime_data_load_any` and
  `helper_symbol=nyash.runtime_data.get_hh`.
- Did not modify `.inc` lowering.

## Observed Metadata

`bench_kilo_leaf_map_getset_has.hako` now emits the measured loop-body get as:

```text
route_id = generic_method.get
box_name = RuntimeDataBox
method = get
receiver_origin_box = MapBox
key_route = i64_const
proof = map_set_scalar_i64_dominates_no_escape
route_kind = runtime_data_load_any
helper_symbol = nyash.runtime_data.get_hh
return_shape = scalar_i64_or_missing_zero
value_demand = scalar_i64
publication_policy = no_publication
lowering_tier = cold_fallback
```

This means the measured front now carries the scalar proof metadata needed by a
future lowering card, while preserving existing RuntimeData facade behavior.

## Rejection

The proof must reject when:

- the receiver is not proven to originate from `MapBox`
- the key is not an i64 const
- the candidate store block does not dominate the get block
- the same-key dominating store is missing
- the same-key stored value is not scalar i64
- any same-receiver mutation appears in the dominated instruction region after
  the store
- the receiver escapes to an unknown same-receiver call in the dominated
  instruction region after the store

## Why

The real `kilo_leaf_map_getset_has` front stores the scalar map value in a
preheader and reads it from the loop body. Same-block proof was structurally
correct but did not feed the measured front. This card records the next narrow
compiler fact in MIR so a later lowering card can consume metadata instead of
teaching `.inc` or Rust to rediscover semantics.

## Acceptance

- Focused MIR route tests cover dominating scalar proof success.
- Focused MIR route tests reject same-receiver mutation after the preheader
  store.
- MIR JSON emits the scalar proof fields for
  `bench_kilo_leaf_map_getset_has.hako`.
- `.inc` lowering remains unchanged.
- Perf/codegen checks show no lowering change.

## Next

After this proof exists, a separate lowering card may try:

```text
MapGet + i64_const key + scalar_i64_or_missing_zero + scalar_i64 demand
  -> scalar MapGet ABI route
```

That card must still prove cycles/IPC improvement before becoming keeper.
