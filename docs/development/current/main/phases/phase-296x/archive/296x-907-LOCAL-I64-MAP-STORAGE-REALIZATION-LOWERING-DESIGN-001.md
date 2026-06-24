# 296x-907 LOCAL-I64-MAP-STORAGE-REALIZATION-LOWERING-DESIGN-001

Status: Landed
Date: 2026-06-16

## Contract

```text
output_contract=hako-local-i64-map-storage-realization-lowering-design-v0
source_evidence=296x-906
row_kind=lowering_design
target_front=kilo_leaf_map_get_dynamic_covered_i64

selected_backend_consumer=local_map_storage_realization_plan_loader
selected_lookup_key=receiver_value
selected_first_lowering_shape=metadata_gated_local_i64_map_lookup

requires_local_fastpath_fact=1
requires_local_storage_realization_plan=1
backend_reads_fallback_evidence=0
backend_reads_helper_symbol=0
backend_reads_source_variable_name=0

backend_loader_next=1
backend_lowering_enabled=0
runtime_helper_enabled=0
product_mapbox_storage_changed=0
product_hasher_swap=0
sidecar_storage=0
mirbuilder_map_storage_ownership=0
winner_claim=0

next_task=LOCAL-I64-MAP-STORAGE-REALIZATION-BACKEND-LOADER-001
summary=ok
```

## Decision

The backend may consider local i64 map storage only when both positive proofs
are present:

```text
LocalFastPathFact at the callsite
LocalMapStorageRealizationPlan for the receiver_value
```

This row does not enable lowering. It only fixes the future backend input
contract and the next implementation seam.

## Boundary

Allowed future reader:

```text
local_map_storage_realization_plans_by_receiver[receiver_value]
```

Forbidden future readers:

```text
fallback_reason
helper symbol
source variable name
PublicArrayBoxFallback / fallback-only evidence
```

The backend must treat missing plan, missing fact, unknown receiver, dynamic
route, generic storage, or maybe-published state as product-compatible
fallback.

## Stop Lines

- no backend lowering
- no runtime helper enablement
- no product `MapBox` storage change
- no product hasher swap
- no sidecar storage
- no MIRBuilder map storage ownership
- no fallback evidence as backend proof
- no helper-name or source-variable-name inference
- no performance winner claim

## Next

`LOCAL-I64-MAP-STORAGE-REALIZATION-BACKEND-LOADER-001` may add a Python backend
metadata loader for `local_map_storage_realization_plans`, keyed by
`receiver_value`. It must remain behavior-neutral until a later explicit
lowering row.
