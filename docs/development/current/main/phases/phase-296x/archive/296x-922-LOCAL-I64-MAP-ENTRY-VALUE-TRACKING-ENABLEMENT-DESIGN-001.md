# 296x-922 LOCAL-I64-MAP-ENTRY-VALUE-TRACKING-ENABLEMENT-DESIGN-001

Status: Landed
Date: 2026-06-16

## Contract

```text
output_contract=hako-local-i64-map-entry-value-tracking-enablement-design-v0
source_evidence=296x-921
row_kind=design_decision
target_front=kilo_leaf_map_get_dynamic_covered_i64

shadow_candidate_available=1
entry_value_tracking_available=1
executable_lowering_enabled=0
selected_decision=defer_until_entry_table_materialization_design
entry_table_materialization_owner_required=1
publication_materialization_policy_required=1
runtime_helper_abi_required=1

product_mapbox_storage_changed=0
product_hasher_swap=0
sidecar_storage=0
mirbuilder_map_storage_ownership=0
new_runtime_helper_enabled=0
backend_lowering_enabled=0
helper_emission_changed=0
winner_claim=0

next_task=LOCAL-I64-MAP-ENTRY-TABLE-MATERIALIZATION-DESIGN-001
summary=ok
```

## Decision

Do not open executable lowering from the entry value tracking shadow row yet.

The shadow row proves that the backend can observe this conjunction:

```text
LocalFastPathFact
LocalI64MapDirectStoragePlan
EntryValueTrackingRows
```

That is not enough to emit a new fast path. The next executable row needs an
explicit owner for:

```text
entry table materialization
publication materialization policy
runtime helper ABI, if any
```

Until those are designed, the backend must not emit a direct table lookup or a
new helper from the shadow candidate.

## Stop Lines

- no product `MapBox` storage change
- no product hasher swap
- no sidecar storage
- no MIRBuilder map storage ownership
- no new runtime helper
- no backend lowering in this row
- no helper emission change in this row
- no entry table materialization
- no helper-name / benchmark-name inference
- no winner claim

## Next

`LOCAL-I64-MAP-ENTRY-TABLE-MATERIALIZATION-DESIGN-001` should decide the shape
of the local entry table and where publication back to product `MapBox` happens.
