# 296x-840 MIMALLOC-MAP-MISSING-EMPTY-ROUTE-GUARD-SURFACE-001

Status: Landed
Date: 2026-06-16

## Purpose

Fix the post-implementation guard surface for the first get-only
missing-empty-map route.

This row is still docs/guard only. It authorizes a narrow implementation row,
but it does not add route metadata, backend lowering, or MapBox behavior.

## Post Target

The implementation row may only add one proof-bearing route for the current
front:

```text
post_missing_empty_map_route_count=1
post_selected_route=map_get_missing_empty_const_zero
post_route_source=MapMissingEmptyRoute
post_target_site=MapGet bb19.i3
```

The selected route must carry all proof fields from 296x-839:

```text
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
```

Backend behavior must be selected from the proof route, not from helper or
method names:

```text
post_backend_consumes_route_decision=1
post_backend_literal_key_special_case_count=0
post_backend_helper_name_branch_count=0
post_backend_benchmark_name_branch_count=0
```

The implementation must not change generic MapBox:

```text
post_mapbox_storage_changed=0
post_mapbox_public_semantics_changed=0
post_product_default_changed=0
```

## Result

```text
output_contract=hako-mimalloc-map-missing-empty-route-guard-surface-v0
source_evidence=296x-839
row_kind=guard_surface
implementation_started=0
perf_first_required=1

target_front=kilo_leaf_map_get_missing
target_site=MapGet bb19.i3
allowed_implementation=MIMALLOC-MAP-MISSING-EMPTY-ROUTE-IMPLEMENTATION-001

post_missing_empty_map_route_count=1
post_selected_route=map_get_missing_empty_const_zero
post_route_source=MapMissingEmptyRoute
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

post_backend_consumes_route_decision=1
post_backend_literal_key_special_case_count=0
post_backend_helper_name_branch_count=0
post_backend_benchmark_name_branch_count=0
post_mapbox_storage_changed=0
post_mapbox_public_semantics_changed=0
post_product_default_changed=0

selected_next=MIMALLOC-MAP-MISSING-EMPTY-ROUTE-IMPLEMENTATION-001
summary=ok
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

