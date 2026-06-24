# 296x-918 LOCAL-I64-MAP-DIRECT-STORAGE-ENABLEMENT-DESIGN-001

Status: Landed
Date: 2026-06-16

## Contract

```text
output_contract=hako-local-i64-map-direct-storage-enablement-design-v0
source_evidence=296x-917
row_kind=design
target_front=kilo_leaf_map_get_dynamic_covered_i64

selected_decision=entry_value_tracking_required_before_executable_lowering
direct_storage_helper_emission_allowed=0
direct_storage_backend_lowering_allowed=0
entry_value_tracking_required=1
entry_value_tracking_owner=MapStoragePlan
entry_value_tracking_surface_next=1
entry_value_tracking_backend_lowering_enabled=0

product_mapbox_storage_changed=0
product_hasher_swap=0
sidecar_storage=0
mirbuilder_map_storage_ownership=0
new_runtime_helper_enabled=0
backend_lowering_enabled=0
helper_emission_changed=0
winner_claim=0

next_task=LOCAL-I64-MAP-ENTRY-VALUE-TRACKING-SURFACE-001
summary=ok
```

## Decision

Direct storage cannot be enabled from the current shadow candidate alone. The
current surfaces prove:

```text
LocalFastPathFact + LocalI64MapDirectStoragePlan(receiver)
```

They do not yet prove which value should be returned from the local table for a
given i64 key. Enabling a direct storage helper without entry-value tracking
would either re-enter product MapBox lookup or invent a benchmark-specific value
path. Both are forbidden.

Therefore the next executable work is gated on a passive entry-value tracking
surface owned by `MapStoragePlan`.

## Required Next Surface

The next row may add metadata that records the set-site values needed to build a
future closed-world i64 key/value table:

```text
receiver_value
set_site
key_value
value_value
key_const_if_known
value_const_if_known
```

The row must remain passive:

```text
backend_lowering_enabled=0
runtime_helper_enabled=0
helper_emission_changed=0
```

## Stop Lines

- no direct storage helper emission
- no backend lowering
- no product `MapBox` storage change
- no product hasher swap
- no sidecar storage
- no MIRBuilder map storage ownership
- no runtime helper
- no entry table materialization yet
- no helper-name / benchmark-name inference
- no winner claim

## Next

`LOCAL-I64-MAP-ENTRY-VALUE-TRACKING-SURFACE-001` may add passive set-site value
tracking metadata. It must not lower differently.
