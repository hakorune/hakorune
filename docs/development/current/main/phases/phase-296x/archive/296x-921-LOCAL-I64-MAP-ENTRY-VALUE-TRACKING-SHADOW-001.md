# 296x-921 LOCAL-I64-MAP-ENTRY-VALUE-TRACKING-SHADOW-001

Status: Landed
Date: 2026-06-16

## Contract

```text
output_contract=hako-local-i64-map-entry-value-tracking-shadow-v0
source_evidence=296x-920
row_kind=shadow_only
target_front=kilo_leaf_map_get_dynamic_covered_i64

shadow_candidate=LocalFastPathFact+LocalI64MapDirectStoragePlan+EntryValueTrackingRows
requires_local_fastpath_fact=1
requires_direct_storage_plan=1
requires_entry_value_tracking_rows=1
requires_receiver_match=1
requires_key_value=1
requires_value_value=1

product_mapbox_storage_changed=0
product_hasher_swap=0
sidecar_storage=0
mirbuilder_map_storage_ownership=0
new_runtime_helper_enabled=0
backend_lowering_enabled=0
helper_emission_changed=0
winner_claim=0

next_task=LOCAL-I64-MAP-ENTRY-VALUE-TRACKING-ENABLEMENT-DESIGN-001
summary=ok
```

## Decision

This row adds a shadow-only Python backend helper that recognizes when a map get
callsite has all three passive inputs available:

```text
LocalFastPathFact
LocalI64MapDirectStoragePlan
local_i64_map_entry_value_tracking_plans_by_receiver
```

The helper returns the fact, direct storage plan, and entry value tracking rows
for the receiver. It does not emit a new helper and does not change lowering.

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

`LOCAL-I64-MAP-ENTRY-VALUE-TRACKING-ENABLEMENT-DESIGN-001` may decide whether
the next row should keep this as a non-executable shadow, introduce an explicit
entry table materialization design, or return to fresh owner selection.
