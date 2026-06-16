# 296x-910 LOCAL-I64-MAP-LEGACY-SHADOW-CONSUMER-RETIRE-DESIGN-001

Status: Landed
Date: 2026-06-16

## Contract

```text
output_contract=hako-local-i64-map-legacy-shadow-consumer-retire-design-v0
source_evidence=296x-909
row_kind=retire_design
target_front=kilo_leaf_map_get_dynamic_covered_i64

legacy_consumer=map_repr.local_i64_key_map_shadow
legacy_consumer_backend_proof=0
selected_action=retire_backend_consumer
remaining_fastpath_owner=LocalFastPathFact_plus_LocalMapStorageRealizationPlan
required_fact=LocalFastPathFact
required_plan=LocalMapStorageRealizationPlan
required_plan_lookup_key=receiver_value

product_mapbox_storage_changed=0
product_hasher_swap=0
sidecar_storage=0
mirbuilder_map_storage_ownership=0
new_runtime_helper_enabled=0
winner_claim=0

next_task=LOCAL-I64-MAP-LEGACY-SHADOW-CONSUMER-RETIRE-001
summary=ok
```

## Decision

The older `map_repr.local_i64_key_map_shadow` consumer proved a metadata seam,
but it is no longer the backend proof owner. The backend fast path must be
authorized by:

```text
LocalFastPathFact at the callsite
LocalMapStorageRealizationPlan for receiver_value
```

The implementation row may remove the old direct consumer and keep the metadata
producer as observation / historical evidence until a separate producer cleanup
row is selected.

## Stop Lines

- do not remove product `MapBox` storage
- do not swap product hasher
- do not add sidecar storage
- do not move map storage ownership into MIRBuilder
- do not add a new runtime helper
- do not use fallback evidence as backend proof
- do not infer from helper name, benchmark name, or source variable name
- do not claim a performance win

## Next

`LOCAL-I64-MAP-LEGACY-SHADOW-CONSUMER-RETIRE-001` retires the Python backend
consumer and updates tests/guards to treat the old pilot as superseded by the
Fact+Plan path.
