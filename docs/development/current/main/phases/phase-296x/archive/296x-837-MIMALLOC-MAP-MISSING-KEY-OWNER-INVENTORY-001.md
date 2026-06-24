# 296x-837 MIMALLOC-MAP-MISSING-KEY-OWNER-INVENTORY-001

Status: Landed
Date: 2026-06-16

## Purpose

Inventory the owner behind the selected `kilo_leaf_map_get_missing` front
before changing MapBox, RoutePlan, backend lowering, or product runtime
behavior.

This row is report/docs only. It classifies the currently observed runtime
boundary and selects the next proof row.

## Evidence

Selected front from 296x-836:

```text
selected_front=kilo_leaf_map_get_missing
selected_owner_family=map_missing_key_string_lookup_runtime_boundary
asm_top_symbol_0=nyash_rust::boxes::map_box::MapBox::get_opt_key_str
asm_top_symbol_0_percent=58.32
asm_top_symbol_1=<i64 as alloc::string::SpecToString>::spec_to_string
asm_top_symbol_1_percent=41.67
```

Source shape:

```hako
local map = new MapBox()

loop (i < ops) {
  local v = map.get(0)
  if (v == null) {
    sum = sum + 1
  }
}
```

The current C pair is a missing-key proof shape, but it is not a cost-equivalent
map lookup implementation:

```c
if (key == 0) {
  sum += 1;
}
```

This means the selected front is useful as a Hakorune runtime-boundary
diagnostic, but it must not be treated as a direct C-equivalent MapBox
algorithm comparison.

## Owner Inventory

MapBox currently stores string keys behind a lock:

```text
map_storage_key_type=String
map_storage_shape=Arc<RwLock<HashMap<String, Box<dyn NyashBox>>>>
```

The visible `get` / `get_opt` path converts the key to a string before lookup:

```text
map_key_source=i64_const_zero
map_key_runtime_conversion=i64_to_string
map_get_visible_path_uses_to_string_box=1
map_get_opt_visible_path_uses_to_string_box=1
map_raw_lookup_helper=MapBox::get_opt_key_str
```

The exact-AOT collection lowering currently falls back to the runtime map slot
load when no metadata decision is selected:

```text
map_collection_runtime_call=nyash.map.slot_load_hh
```

There is already a typed map lookup fusion seam:

```text
map_lookup_fusion_route_seam_exists=1
map_lookup_fusion_route_owner=src/mir/map_lookup_fusion_plan.rs
map_lookup_fusion_metadata_key=map_lookup_fusion_routes
map_lookup_fusion_backend_consumer=collection_method_call.py
```

However, the existing fusion route is scoped to same-receiver same-i64-key
`MapGet`/`MapHas` pairs:

```text
map_lookup_fusion_existing_scope=same_receiver_same_i64_key_get_has_pair
current_front_has_map_get=1
current_front_has_map_has=0
current_front_matches_existing_fusion_scope=0
```

The next row must therefore inspect whether a missing-empty-map proof should be
added as a separate route, or whether the correct owner is the wider MapBox
string-key / runtime boundary.

## Result

```text
output_contract=hako-mimalloc-map-missing-key-owner-inventory-v0
source_evidence=296x-836
row_kind=inventory
implementation_started=0
perf_first_required=1

target_front=kilo_leaf_map_get_missing
target_source=benchmarks/bench_kilo_leaf_map_get_missing.hako
c_pair_source=benchmarks/c/bench_kilo_leaf_map_get_missing.c
c_pair_semantic_cost_mismatch_visible=1

ny_loop_call_symbol=nyash.runtime_data.get_hh
asm_top_symbol_0=nyash_rust::boxes::map_box::MapBox::get_opt_key_str
asm_top_symbol_0_percent=58.32
asm_top_symbol_1=<i64 as alloc::string::SpecToString>::spec_to_string
asm_top_symbol_1_percent=41.67

map_key_source=i64_const_zero
map_key_runtime_conversion=i64_to_string
map_storage_key_type=String
map_storage_shape=Arc<RwLock<HashMap<String, Box<dyn NyashBox>>>>
map_visible_get_missing_allocates_string_error=1

map_lookup_fusion_route_seam_exists=1
map_lookup_fusion_existing_scope=same_receiver_same_i64_key_get_has_pair
current_front_has_map_get=1
current_front_has_map_has=0
current_front_matches_existing_fusion_scope=0

selected_owner=missing_empty_map_route_trigger_probe
selected_owner_confidence=medium
selected_next=MIMALLOC-MAP-MISSING-EMPTY-ROUTE-TRIGGER-PROBE-001
summary=ok
```

## Stop Line

```text
do not patch MapBox before missing-empty-map route trigger probe
do not add key-specific special case for literal 0
do not change MapBox public key semantics
do not replace String-key storage in this row
do not treat the C pair as a cost-equivalent HashMap/RwLock/string-conversion implementation
do not infer keeper status from nyash.map.slot_load_hh or MapBox::get_opt_key_str alone
```

