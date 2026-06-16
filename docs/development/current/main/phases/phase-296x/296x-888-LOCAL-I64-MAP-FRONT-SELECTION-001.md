# 296x-888 LOCAL-I64-MAP-FRONT-SELECTION-001

Status: Landed
Date: 2026-06-16

## Contract

```text
output_contract=hako-local-i64-map-front-selection-v0
source_evidence=296x-887
row_kind=front_selection
target_front=kilo_leaf_map_get_dynamic_covered_i64
target_source=benchmarks/bench_kilo_leaf_map_get_dynamic_covered_i64.hako

local_map_birth_count=1
init_i64_set_count=3
hot_loop_i64_get_count=1
post_loop_i64_get_count=1
dynamic_unknown_key_get_count=0
text_key_write_count=0
keys_values_json_use_count=0
plugin_extern_publication_count=0
map_return_escape_count=0

selected_front=kilo_leaf_map_get_dynamic_covered_i64
selected_storage_plan=LocalI64KeyMap
selected_scope=shadow_only
implementation_allowed=0
next_task=LOCAL-I64-MAP-STORAGE-SHADOW-001
summary=ok
```

## Evidence

The selected front has one local `MapBox` birth, three i64-key writes, a hot
loop i64 get through `k = i % 3`, and one post-loop i64 literal get:

```hako
local map = new MapBox()
map.set(0, 1)
map.set(1, 2)
map.set(2, 3)

loop (i < ops) {
  local k = i % 3
  local v = map.get(k)
  sum = sum + v
}

return sum + map.get(1)
```

The source does not publish the map through `keys`, `values`, `toJSON`, plugin
or extern calls, or a dynamic return boundary.

## Decision

Use this front as the first `LocalI64KeyMap` shadow candidate.

This row does not authorize lowering. The next row may produce a passive
storage shadow report, but implementation stays closed until the shadow proves
that publication sites and RoutePlan consumers are explicit.

## Stop Lines

- no product MapBox i64-only storage
- no product hasher swap
- no sidecar storage
- no MIRBuilder map storage ownership
- no backend lowering from front-selection evidence alone
- no benchmark-name / helper-name / variable-name special case
