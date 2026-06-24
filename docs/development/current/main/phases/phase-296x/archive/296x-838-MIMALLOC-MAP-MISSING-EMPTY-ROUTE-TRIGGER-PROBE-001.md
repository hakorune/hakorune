# 296x-838 MIMALLOC-MAP-MISSING-EMPTY-ROUTE-TRIGGER-PROBE-001

Status: Landed
Date: 2026-06-16

## Purpose

Probe whether the selected `kilo_leaf_map_get_missing` front already emits a
backend-active route for the get-only missing-key case.

This row is probe/docs only. It must not patch MapBox, the backend, the route
planner, or product runtime behavior.

## Probe

Command shape:

```bash
target/release/hakorune \
  --backend mir \
  --emit-mir-json target/map_missing_probe_837/map_missing.direct.mir.json \
  benchmarks/bench_kilo_leaf_map_get_missing.hako
```

The generic `tools/perf/dump_mir.sh` provider route fell back to `jsonfrag` for
this source, and that fallback erased the `map.get(0)` call. The direct
`--backend mir --emit-mir-json` route is therefore the evidence used for this
row.

## Observed MIR Metadata

The direct MIR JSON has one `MapBox` allocation and one Map get call in `main`:

```text
newbox_mapbox_count=1
map_get_call_site=bb19.i3
map_get_receiver_value=3
map_get_key_value=30
map_get_key_route=i64_const
```

The semantic route exists, but it remains a runtime-data facade route:

```text
generic_method_route_count=1
generic_method_route_core_op=MapGet
generic_method_route_route_kind=runtime_data_load_any
generic_method_route_publication_policy=runtime_data_facade
generic_method_route_value_demand=runtime_i64_or_handle
generic_method_route_return_shape=mixed_runtime_i64_or_handle
generic_method_route_helper_symbol=nyash.runtime_data.get_hh
```

The existing same-key get/has fusion does not trigger:

```text
map_lookup_fusion_route_count=0
route_decision_count=0
current_front_map_get_count=1
current_front_map_has_count=0
existing_same_key_get_has_fusion_triggered=0
```

The only map representation metadata is generic runtime hash-map metadata:

```text
map_repr_plan_count=1
map_repr_plan_route_id=map_repr.generic_hash_runtime
map_repr_plan_repr_kind=generic_hash_runtime
map_repr_plan_source_helper_symbol=nyash.runtime_data.get_hh
```

## Interpretation

The selected front does not currently have a backend-active missing-key route.
The existing `MapLookupFusionRoute` cannot cover it because its scope is a
same-receiver same-i64-key `MapGet`/`MapHas` pair.

A get-only missing-key route would need its own proof. At minimum, it must
prove:

```text
receiver_birth_is_new_mapbox=1
no_map_set_or_delete_before_get=1
receiver_not_published_before_get=1
receiver_not_escaped_before_get=1
result_shape_is_null_or_zero_missing=1
fallback_policy_is_explicit=1
```

Until that proof exists, `MapBox` stays on the generic runtime-data facade and
the backend must not fold `map.get(0)` from helper names, literal keys, or C-pair
shape.

## Result

```text
output_contract=hako-mimalloc-map-missing-empty-route-trigger-probe-v0
source_evidence=296x-837
row_kind=probe
implementation_started=0
perf_first_required=1

target_front=kilo_leaf_map_get_missing
mir_json_emit_route=direct_backend_mir_emit
jsonfrag_fallback_metadata_usable=0

newbox_mapbox_count=1
map_get_call_site=bb19.i3
map_get_receiver_value=3
map_get_key_value=30
map_get_key_route=i64_const

generic_method_route_count=1
generic_method_route_core_op=MapGet
generic_method_route_route_kind=runtime_data_load_any
generic_method_route_publication_policy=runtime_data_facade
generic_method_route_value_demand=runtime_i64_or_handle
generic_method_route_return_shape=mixed_runtime_i64_or_handle
generic_method_route_helper_symbol=nyash.runtime_data.get_hh

map_lookup_fusion_route_count=0
route_decision_count=0
current_front_map_get_count=1
current_front_map_has_count=0
existing_same_key_get_has_fusion_triggered=0

map_repr_plan_count=1
map_repr_plan_route_id=map_repr.generic_hash_runtime
map_repr_plan_repr_kind=generic_hash_runtime
map_repr_plan_source_helper_symbol=nyash.runtime_data.get_hh

missing_empty_map_route_exists=0
missing_empty_map_route_proof_required=1
selected_owner=missing_empty_map_route_design
selected_owner_confidence=medium
selected_next=MIMALLOC-MAP-MISSING-EMPTY-ROUTE-DESIGN-001
summary=ok
```

## Stop Line

```text
do not fold get-only missing-key from generic_method_routes alone
do not consume map_lookup_fusion_routes without RouteDecision
do not add backend branches for literal key 0
do not add benchmark-name or helper-name branches
do not change MapBox public String-key semantics
do not replace MapBox storage in this row
do not open missing-empty-map lowering before proof fields are defined
```

