# 296x-853 MIMALLOC-MAP-GET-I64-SCALAR-ROUTE-INVENTORY-001

Status: Landed
Date: 2026-06-16

## Purpose

Inventory the real `MapBox.get` i64 scalar route gap after 296x-852 corrected
the invalid `kilo_leaf_map_get_missing` C comparison.

This row does not implement `MapGetI64`. It records the current producer,
consumer, substrate, and missing route vocabulary so the next row can design one
narrow proof-driven implementation seam.

## Current Route Shape

`MapBox.has` already has an i64 scalar route:

```text
MapHas i64:
  GenericMethodRouteKind::MapContainsI64
  route_kind=map_contains_i64
  helper=nyash.map.probe_hi
  key_route=i64_const required by shim policy
```

`MapBox.get` does not:

```text
MapGet direct any:
  GenericMethodRouteKind::MapLoadAny
  route_kind=map_load_any
  helper=nyash.map.slot_load_hh

RuntimeDataBox.get on MapBox:
  GenericMethodRouteKind::RuntimeDataLoadAny
  route_kind=runtime_data_load_any
  helper=nyash.runtime_data.get_hh
```

Scalar proof already exists for some `RuntimeDataBox.get` sites:

```text
proof=map_set_scalar_i64_same_key_no_escape
proof=map_set_scalar_i64_dominates_no_escape
return_shape=scalar_i64_or_missing_zero
value_demand=scalar_i64
publication_policy=no_publication
```

But those proof-positive sites still keep:

```text
route_kind=RuntimeDataLoadAny
helper=nyash.runtime_data.get_hh
lowering_tier=ColdFallback
```

## Substrate Shape

The kernel already exposes a map i64-key load alias:

```text
helper=nyash.map.slot_load_hi
file=crates/nyash_kernel/src/plugin/map_aliases.rs
```

However, it still converts the i64 key into the current string-key map surface:

```text
key_conversion=map_key_string_from_i64
map_storage_key_shape=String
typed_i64_key_map_storage=0
```

Therefore the first scalar route can remove the `runtime_data.get_hh` facade and
mixed-runtime result path, but it does not yet remove String-key storage.

## Result

```text
output_contract=hako-mimalloc-map-get-i64-scalar-route-inventory-v0
source_evidence=296x-852,src/mir/generic_method_route_plan,lang/c-abi/shims,crates/nyash_kernel/src/plugin
row_kind=inventory
implementation_started=0

target_front=kilo_leaf_map_get_missing
old_c_pair_valid=0
semantic_route_still_valid=1

map_has_i64_route_kind=MapContainsI64
map_has_i64_route_tag=map_contains_i64
map_has_i64_helper=nyash.map.probe_hi

map_get_i64_route_kind_present=0
map_get_i64_route_tag_present=0
map_get_i64_helper_present_in_route_model=0
map_get_i64_substrate_helper_present=1
map_get_i64_substrate_helper=nyash.map.slot_load_hi

scalar_map_get_store_proof_present=1
scalar_map_get_store_proof_route_kind=RuntimeDataLoadAny
scalar_map_get_store_proof_helper=nyash.runtime_data.get_hh
scalar_map_get_store_proof_tier=ColdFallback
scalar_map_get_store_proof_return_shape=scalar_i64_or_missing_zero
scalar_map_get_store_proof_publication_policy=no_publication

runtime_data_map_get_facade_visible=1
map_get_i64_key_string_conversion_visible=1
typed_i64_key_map_storage_enabled=0
host_handle_boundary_still_visible=1

benchmark_source_changed=0
compiler_lowering_changed=0
runtime_helper_changed=0
product_default_changed=0

selected_next=MIMALLOC-MAP-GET-I64-SCALAR-ROUTE-DESIGN-001
summary=ok
```

## Stop Line

```text
do not implement MapGetI64 in this inventory row
do not change benchmark source in this inventory row
do not infer route legality from helper names alone
do not claim typed i64-key MapBox storage exists
do not claim String key conversion is removed by nyash.map.slot_load_hi
do not broaden to generic MapBox storage replacement
do not invalidate RuntimeDataBox.get mixed return semantics
do not reopen the old Hako-vs-C winner claim
```

## Proof Bundle

```bash
bash tools/checks/k2_wide_phase296x_map_get_i64_scalar_route_inventory_guard.sh
bash tools/checks/current_state_pointer_guard.sh
```
