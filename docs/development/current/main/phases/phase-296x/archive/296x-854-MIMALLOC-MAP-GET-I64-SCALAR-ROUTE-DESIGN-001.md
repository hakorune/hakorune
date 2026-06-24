# 296x-854 MIMALLOC-MAP-GET-I64-SCALAR-ROUTE-DESIGN-001

Status: Landed
Date: 2026-06-16

## Purpose

Design the narrow `MapGetI64` scalar route after 296x-853 inventoried the gap.

This row is design-only. It does not add route enum variants, substrate helpers,
LLVM shim cases, benchmark changes, or backend behavior.

## Decision

v0 selects a helper-backed scalar route:

```text
route_kind=MapLoadScalarI64
route_tag=map_load_scalar_i64
helper=nyash.map.scalar_load_hi
route_id=generic_method.get
emit_kind=get
effect=read.key
core_op=MapGet
lowering_tier=WarmDirectAbi
return_shape=scalar_i64_or_missing_zero
value_demand=scalar_i64
publication_policy=no_publication
```

The route is allowed only for proof-positive `RuntimeDataBox.get` from a MapBox
receiver:

```text
receiver_origin_box=MapBox
key_route=i64_const
proof in:
  map_set_scalar_i64_same_key_no_escape
  map_set_scalar_i64_dominates_no_escape
```

Unproven mixed-return `RuntimeDataBox.get` remains on:

```text
route_kind=RuntimeDataLoadAny
helper=nyash.runtime_data.get_hh
lowering_tier=ColdFallback
return_shape=mixed_runtime_i64_or_handle
publication_policy=runtime_data_facade
```

Direct `MapBox.get` with generic handle return remains on:

```text
route_kind=MapLoadAny
helper=nyash.map.slot_load_hh
```

## Why Not `nyash.map.slot_load_hi`

`nyash.map.slot_load_hi` is an i64-key slot-load alias, but it returns a handle
and uses the current String-key MapBox storage. It is not the scalar/no-
publication route.

The scalar helper must have a distinct name and contract:

```text
nyash.map.scalar_load_hi(handle, key_i64) -> i64
```

Contract:

```text
missing_or_invalid -> 0
scalar IntegerBox/BoolBox value -> immediate i64
non-scalar value -> 0 or fail-safe scalar-missing result
no handle publication
no mixed runtime return
```

v0 may still use the existing String-key storage internally. Removing
`key_i64.to_string()` requires a later typed/numeric key map storage row.

## Deferred Alternative

Constant emission from `ScalarI64MapGetStoreFact.stored_value` is a separate
route-decision design. It requires carrying the stored value through route
metadata and must not be mixed into this helper-backed v0 row.

## Result

```text
output_contract=hako-mimalloc-map-get-i64-scalar-route-design-v0
source_evidence=296x-853
row_kind=design
implementation_started=0

selected_route_kind=MapLoadScalarI64
selected_route_tag=map_load_scalar_i64
selected_helper=nyash.map.scalar_load_hi
selected_route_id=generic_method.get
selected_emit_kind=get
selected_effect=read.key
selected_core_op=MapGet
selected_lowering_tier=WarmDirectAbi
selected_return_shape=scalar_i64_or_missing_zero
selected_value_demand=scalar_i64
selected_publication_policy=no_publication

allowed_receiver_origin_box=MapBox
allowed_key_route=i64_const
allowed_proof_same_key=map_set_scalar_i64_same_key_no_escape
allowed_proof_dominates=map_set_scalar_i64_dominates_no_escape

runtime_data_mixed_get_preserved=1
direct_mapbox_handle_get_preserved=1
slot_load_hi_is_not_scalar_route=1
string_key_storage_remains=1
stored_value_constant_emission_deferred=1

benchmark_source_changed=0
compiler_lowering_changed=0
runtime_helper_changed=0
product_default_changed=0

selected_next=MIMALLOC-MAP-GET-I64-SCALAR-ROUTE-GUARD-SURFACE-001
summary=ok
```

## Stop Line

```text
do not implement the route in this design row
do not use nyash.map.slot_load_hi as the scalar helper
do not reroute mixed RuntimeDataBox.get calls
do not reroute direct MapBox.get handle-return calls
do not remove String-key storage in this route row
do not add stored-value constant emission in the helper-backed route row
do not claim benchmark parity until the C pair is repaired
```

## Proof Bundle

```bash
bash tools/checks/k2_wide_phase296x_map_get_i64_scalar_route_design_guard.sh
bash tools/checks/current_state_pointer_guard.sh
```
