# 296x-839 MIMALLOC-MAP-MISSING-EMPTY-ROUTE-DESIGN-001

Status: Landed
Date: 2026-06-16

## Purpose

Define the proof boundary for a get-only missing-key `MapBox` route before any
implementation.

The target front is still `kilo_leaf_map_get_missing`, but this row is
design-only. It does not add a route, alter MapBox, change backend lowering, or
claim a performance win.

## Decision

Add a future proof-bearing route for this narrow shape:

```text
route_family=MapMissingEmptyRoute
selected_route=map_get_missing_empty_const_zero
semantic_op=MapGet
result_value=zero/null runtime-data missing value
fallback_route=generic_map_runtime_data_load_any
```

The route is not a generic direct map representation and not a MapBox storage
change. It is a local-empty-map read proof:

```text
MapBox:
  remains generic dynamic container

MapMissingEmptyRoute:
  proves one get-only read can fold to the runtime-data missing/null result
  only before publication, escape, or mutation of the receiver

Backend:
  consumes selected RouteDecision / LoweringPlan only
```

## Required Proof Fields

A site may select `map_get_missing_empty_const_zero` only when all of these are
true:

```text
receiver_birth_is_new_mapbox=1
receiver_root_is_same_local_value=1
receiver_not_published_before_get=1
receiver_not_escaped_before_get=1
no_map_set_before_get=1
no_map_delete_before_get=1
no_map_clear_before_get=1
no_unknown_receiver_mutation_before_get=1
get_key_route=i64_const
generic_method_core_op=MapGet
generic_method_publication_policy=runtime_data_facade
generic_method_return_shape=mixed_runtime_i64_or_handle
result_missing_value=zero_null
fallback_policy=generic_runtime_data_facade
```

These fields intentionally use publication/escape/mutation proofs rather than
backend helper names. If any proof is unavailable, the route must stay generic.

## Ownership

```text
MIRBuilder:
  emits NewBox / call shape only
  does not decide missing-empty routes

GenericMethodRoute:
  supplies MapGet semantic site evidence

RouteDecision / LoweringPlan:
  owns backend-active route selection

Backend:
  emits const zero/null only from selected proof-bearing route

MapBox:
  remains the product runtime semantic owner for generic map behavior
```

## Result

```text
output_contract=hako-mimalloc-map-missing-empty-route-design-v0
source_evidence=296x-838
row_kind=design
implementation_started=0
perf_first_required=1

target_front=kilo_leaf_map_get_missing
route_family=MapMissingEmptyRoute
selected_route=map_get_missing_empty_const_zero
semantic_op=MapGet
result_missing_value=zero_null
fallback_route=generic_map_runtime_data_load_any

receiver_birth_is_new_mapbox_required=1
receiver_root_is_same_local_value_required=1
receiver_not_published_before_get_required=1
receiver_not_escaped_before_get_required=1
no_map_set_before_get_required=1
no_map_delete_before_get_required=1
no_map_clear_before_get_required=1
no_unknown_receiver_mutation_before_get_required=1
get_key_route_required=i64_const
generic_method_publication_policy_required=runtime_data_facade
generic_method_return_shape_required=mixed_runtime_i64_or_handle

backend_consumes_route_decision_only=1
map_lookup_fusion_route_reused=0
map_repr_plan_changed=0
mapbox_storage_changed=0
product_default_changed=0

selected_next=MIMALLOC-MAP-MISSING-EMPTY-ROUTE-GUARD-SURFACE-001
summary=ok
```

## Stop Line

```text
do not implement before guard surface
do not fold missing get from helper names
do not fold missing get from literal key 0 alone
do not reuse MapLookupFusionRoute for get-only missing-empty proof
do not call generic MapBox a DirectMap
do not read raw MapBox storage from backend
do not change MapBox missing-key public semantics
do not silently fall back from a required selected route
```

