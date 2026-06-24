# 296x-841 MIMALLOC-MAP-MISSING-EMPTY-ROUTE-IMPLEMENTATION-001

Status: Landed
Date: 2026-06-16

## Purpose

Implement the first proof-bearing get-only missing-empty-map route selected by
296x-839 and bounded by 296x-840.

This row adds a MIR-owned `MapMissingEmptyRoute`, projects it into
`RouteDecision`, and lets both exact-AOT backend consumers read the selected
route:

```text
Python/llvmlite unit backend:
  src/llvm_py/instructions/mir_call/collection_method_call.py

pure-first boundary C emitter used by ny-llvmc perf runs:
  lang/c-abi/shims/hako_llvmc_ffi_generic_method_get_policy.inc
```

It does not change generic MapBox storage, public MapBox semantics, or product
defaults.

## Implementation

The implementation keeps the proof owner in MIR:

```text
src/mir/map_missing_empty_route_plan.rs
  -> collect_function_map_missing_empty_routes()
  -> MapMissingEmptyRoute
```

The route is intentionally narrow:

```text
receiver birth:
  NewBox MapBox

receiver state before get:
  unpublished
  unescaped
  no set/delete/clear
  no unknown receiver mutation

key:
  i64 const

current generic method shape:
  RuntimeDataBox.get
  publication_policy=runtime_data_facade
  return_shape=mixed_runtime_i64_or_handle
```

`RouteDecision` is the backend-facing surface:

```text
source_plan_kind=MapMissingEmptyRoute
selected_route=map_get_missing_empty_const_zero
selected_i64_const=0
```

The backends consume only this selected route. They reject the same route name
when it comes from a different `source_plan_kind`; the boundary emitter also
requires the site-local `route.decision`, `semantic_op=MapGet`, fallback route,
receiver box, and `selected_i64_const=0`.

## Result

```text
output_contract=hako-mimalloc-map-missing-empty-route-implementation-v0
source_evidence=296x-840
row_kind=implementation
implementation_started=1

target_front=kilo_leaf_map_get_missing
target_site=MapGet bb19.i3
missing_empty_map_route_plan_added=1
route_decision_source_plan_kind=MapMissingEmptyRoute
selected_route=map_get_missing_empty_const_zero
selected_i64_const=0
selected_route_count_on_target_front=1

post_missing_empty_map_route_count=1
post_selected_route=map_get_missing_empty_const_zero
post_route_source=MapMissingEmptyRoute
post_backend_consumes_route_decision=1
post_boundary_backend_consumes_route_decision=1
post_backend_literal_key_special_case_count=0
post_backend_helper_name_branch_count=0
post_backend_benchmark_name_branch_count=0

post_receiver_birth_is_new_mapbox=1
post_receiver_root_is_same_local_value=1
post_receiver_not_published_before_get=1
post_receiver_not_escaped_before_get=1
post_no_map_set_before_get=1
post_no_map_delete_before_get=1
post_no_map_clear_before_get=1
post_no_unknown_receiver_mutation_before_get=1
post_get_key_route=i64_const
post_generic_method_publication_policy=runtime_data_facade
post_generic_method_return_shape=mixed_runtime_i64_or_handle

post_mapbox_storage_changed=0
post_mapbox_public_semantics_changed=0
post_product_default_changed=0
perf_win_claim=0

selected_next=MIMALLOC-MAP-MISSING-EMPTY-ROUTE-MEASUREMENT-001
summary=ok
```

Target MIR JSON proof:

```text
matching_route_decisions=1
function=main
site_id=b19.i3
semantic_op=MapGet
access_kind=map_missing_empty_get
selected_route=map_get_missing_empty_const_zero
source_plan_kind=MapMissingEmptyRoute
selected_i64_const=0
fallback_route=generic_map_runtime_data_load_any
receiver_box_name=MapBox
```

## Proof

```bash
cargo test --lib map_missing_empty -- --nocapture
PYTHONPATH=src/llvm_py python3 -m unittest src.llvm_py.tests.test_collection_method_call
cargo build --release --bin hakorune
target/release/hakorune --backend mir --emit-mir-json target/map_missing_probe_841/map_missing.direct.mir.json benchmarks/bench_kilo_leaf_map_get_missing.hako
bash tools/perf/build_perf_release.sh
bash tools/checks/k2_wide_phase296x_map_missing_empty_route_implementation_guard.sh
```

## Stop Line

```text
do not broaden beyond get-only missing-empty-map
do not implement a generic DirectMap
do not patch MapBox storage or visible get semantics
do not fold from literal key 0 without receiver birth and no-mutation proofs
do not let backend infer from helper names
do not use benchmark-name branches
do not claim a perf win before measurement row
```
