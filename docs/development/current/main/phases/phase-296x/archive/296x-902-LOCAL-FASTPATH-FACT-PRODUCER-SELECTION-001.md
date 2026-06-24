# 296x-902 LOCAL-FASTPATH-FACT-PRODUCER-SELECTION-001

Status: Landed
Date: 2026-06-16

## Contract

```text
output_contract=hako-local-fastpath-fact-producer-selection-v0
source_evidence=296x-901
row_kind=producer_selection_and_minimal_implementation

selected_producer=map_repr_scalar_i64_no_publication_get
producer_owner=src/mir/map_repr_plan.rs
producer_input=MapReprPlan(route_id=map_repr.generic_hash_runtime,source_route_kind=map_load_scalar_i64,publication_policy=no_publication,return_shape=scalar_i64_or_missing_zero)
producer_output=FunctionMetadata.local_fastpath_facts
producer_positive_fact_only=1
producer_fallback_evidence_enabled=0
producer_observation_export_enabled=0

alias_class_source=v0_receiver_value_placeholder
route_plan_id_source=v0_map_repr_plan_index
storage_plan_id_source=v0_map_repr_plan_index
full_alias_analysis_enabled=0
full_object_storage_plan_enabled=0

hosthandle_bypass_enabled=0
direct_storage_enabled=0
product_mapbox_storage_changed=0
product_hasher_swap=0
winner_claim=0
next_task=LOCAL-KNOWN-RECEIVER-DIRECT-CALL-MEASUREMENT-001
summary=ok
```

## Decision

The first `LocalFastPathFact` producer is deliberately narrow: it consumes only
existing positive `MapReprPlan` rows where:

```text
route_id=map_repr.generic_hash_runtime
source_route_kind=map_load_scalar_i64
publication_policy=no_publication
return_shape=scalar_i64_or_missing_zero
```

It does not inspect helper symbols or source variable names, and it does not
create fallback facts. Unknown/missing proof still means no fact.

## v0 IDs

This row does not open a full alias analysis or full ObjectStoragePlan producer.
The v0 fact uses conservative placeholders:

```text
alias_class = receiver_value
route_plan_id = map_repr_plan index
storage_plan_id = map_repr_plan index
```

These IDs are transport/proof handles for the current narrow producer only. A
later row may replace them with real AliasClassifier / RoutePlan /
ObjectStoragePlan IDs.

## Stop Lines

- no fallback Fact producer
- no observation / fallback evidence export
- no helper-name or source-variable-name inference
- no HostHandle bypass
- no direct storage enablement
- no product MapBox storage or hasher change
- no Hako-vs-C winner claim

## Validation

```bash
cargo test --lib mir::map_repr_plan::tests::refresh_function_map_repr_plans_emits_local_fastpath_facts_for_scalar_no_publication_get
bash tools/checks/k2_wide_phase296x_local_fastpath_fact_producer_selection_guard.sh
```
