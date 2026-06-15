# 296x-843 MIMALLOC-MAP-MISSING-EMPTY-ROUTE-CLOSEOUT-001

Status: Landed
Date: 2026-06-16

## Purpose

Close the `kilo_leaf_map_get_missing` fresh front after the proof-bearing
missing-empty-map route landed and measured as a keeper.

This closeout keeps the win scoped to the proven route:

```text
MapMissingEmptyRoute:
  local new MapBox
  no publish / escape / mutation before get
  i64 const key
  runtime-data missing result folds to 0
```

It does not authorize a generic DirectMap, MapBox storage replacement, or
helper-name inferred map folding.

## Closeout

```text
output_contract=hako-mimalloc-map-missing-empty-route-closeout-v0
source_evidence=296x-841,296x-842
row_kind=closeout
target_front=kilo_leaf_map_get_missing

route_plan_owner=MapMissingEmptyRoute
route_decision_owner=RouteDecision
backend_consumer_owner=generic_method_get_policy
python_backend_unit_consumer=1
boundary_backend_consumer=1

selected_route=map_get_missing_empty_const_zero
source_plan_kind=MapMissingEmptyRoute
selected_i64_const=0
ny_main_runtime_data_get_hh_call_count=0
route_winner_claim=1

front_closed=1
kernel_path_closed_for_this_front=1
generic_direct_map_enabled=0
mapbox_storage_changed=0
mapbox_public_semantics_changed=0
product_default_changed=0
helper_name_inference_enabled=0
literal_key_only_fold_enabled=0

selected_next=MIMALLOC-FRESH-FRONT-SELECTION-AFTER-MAP-MISSING-EMPTY-CLOSEOUT-001
summary=ok
```

## Proof Bundle

```bash
cargo fmt --check
cargo test --lib map_missing_empty -- --nocapture
PYTHONPATH=src/llvm_py python3 -m unittest src.llvm_py.tests.test_collection_method_call
cargo build --release --bin hakorune
bash tools/perf/build_perf_release.sh
bash tools/checks/k2_wide_phase296x_map_missing_empty_route_implementation_guard.sh
bash tools/checks/k2_wide_phase296x_map_missing_empty_route_measurement_guard.sh
bash tools/checks/k2_wide_phase296x_map_missing_empty_route_closeout_guard.sh
bash tools/checks/current_state_pointer_guard.sh
```

## Stop Line

```text
do not keep optimizing this front without a fresh owner-selection row
do not broaden the route beyond the existing proof fields
do not remove MapBox birth in this closeout
do not retire MapBox / Arc / HostHandle here
do not infer from nyash.runtime_data.get_hh or helper names
```
