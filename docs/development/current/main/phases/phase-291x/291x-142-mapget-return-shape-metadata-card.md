---
Status: Landed
Date: 2026-04-24
Scope: Emit MapGet return-shape metadata without changing codegen.
Related:
  - docs/development/current/main/CURRENT_STATE.toml
  - docs/development/current/main/design/hotline-core-method-contract-ssot.md
  - docs/development/current/main/phases/phase-291x/291x-141-maphas-i64-route-card.md
  - src/mir/generic_method_route_plan.rs
  - src/mir/generic_method_route_facts.rs
  - src/runner/mir_json_emit/root.rs
---

# 291x-142 MapGet Return-Shape Metadata Card

## Goal

Make `MapGet` visible as compiler metadata before trying any new hot lowering.
This is a compiler-boundary keeper, not a performance keeper.

## Design

`RuntimeDataBox.get(key)` may only become MapGet metadata when MIR proves the
receiver comes from `MapBox`.

Default metadata remains conservative:

```text
route_id = generic_method.get
core_method = MapGet
receiver_origin_box = MapBox
key_route = i64_const | unknown_any
route_kind = runtime_data_load_any
helper_symbol = nyash.runtime_data.get_hh
return_shape = mixed_runtime_i64_or_handle
value_demand = runtime_i64_or_handle
publication_policy = runtime_data_facade
lowering_tier = cold_fallback
```

`cold_fallback` is the effective lowering tier for this facade route. The
CoreMethodContract row may still describe direct MapBox carrier rows separately;
this card does not claim the RuntimeData facade can use that direct ABI.

## Implementation

- Added `GenericMethodReturnShape::MixedRuntimeI64OrHandle`.
- Added `GenericMethodPublicationPolicy::RuntimeDataFacade`.
- Added `GenericMethodValueDemand::RuntimeI64OrHandle`.
- Added `GenericMethodRouteKind::RuntimeDataLoadAny`.
- Added `GenericMethodRouteProof::GetSurfacePolicy`.
- Added `RuntimeDataBox.get` MapBox-origin matching in MIR route planning.
- Made MIR JSON route emission use route-owned `route_id`, `emit_kind`, and
  effect tags instead of hard-coded `has` values.
- Did not modify `.inc` get lowering.

## Observed Metadata

For `bench_kilo_leaf_map_getset_has.hako`:

```text
route_id = generic_method.get
box_name = RuntimeDataBox
method = get
receiver_origin_box = MapBox
key_route = i64_const
core_method = MapGet
lowering_tier = cold_fallback
route_kind = runtime_data_load_any
helper_symbol = nyash.runtime_data.get_hh
return_shape = mixed_runtime_i64_or_handle
value_demand = runtime_i64_or_handle
publication_policy = runtime_data_facade
effects = read.key
```

## Boundary

- Codegen stays unchanged.
- `ny_main` must still call `nyash.runtime_data.get_hh`.
- No direct `nyash.map.slot_load_hi`.
- No `nyash.map.runtime_load_hi`.
- No `.inc` MapGet metadata reader in this card.
- No scalar proof in this card.

## Why

`MapGet` return publication is the semantic boundary:

```text
IntegerBox value -> immediate i64
BoolBox value    -> immediate i64
object value     -> handle / borrowed alias handling
missing key      -> 0
```

`key_route=i64_const` alone is not sufficient to leave the RuntimeData facade.
The next lowering keeper needs a separate return-shape proof.

## Acceptance

- MIR JSON emits a `generic_method.get` route for the measured RuntimeData
  MapBox facade route.
- The metadata uses `cold_fallback` and `nyash.runtime_data.get_hh`.
- `.inc` lowering remains unchanged.
- Focused route/JSON tests cover the new metadata fields.
- Perf is no-regression because lowering is unchanged.

## Next

Add a separate scalar proof card:

```text
return_shape = scalar_i64_or_missing_zero
value_demand = scalar_i64
publication_policy = no_publication
```

The first scalar proof should be straight-line only: same receiver, same i64
const key, dominating scalar store, no unknown mutation, no escape.
