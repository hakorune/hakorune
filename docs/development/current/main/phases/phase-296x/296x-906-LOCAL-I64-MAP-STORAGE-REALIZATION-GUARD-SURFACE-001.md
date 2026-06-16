# 296x-906 LOCAL-I64-MAP-STORAGE-REALIZATION-GUARD-SURFACE-001

Status: Landed
Date: 2026-06-16

## Contract

```text
output_contract=hako-local-i64-map-storage-realization-guard-surface-v0
source_evidence=296x-905
row_kind=passive_plan_surface
target_front=kilo_leaf_map_get_dynamic_covered_i64

plan_surface=FunctionMetadata.local_map_storage_realization_plans
plan_struct=LocalMapStorageRealizationPlan
plan_owner=src/mir/map_repr_plan.rs
json_export_enabled=1
json_field=local_map_storage_realization_plans

representation=local_i64_key_map
publication_materialization_required=1
backend_lowering_enabled=0
runtime_helper_enabled=0
product_mapbox_storage_changed=0
product_hasher_swap=0
sidecar_storage=0
mirbuilder_map_storage_ownership=0
winner_claim=0

next_task=LOCAL-I64-MAP-STORAGE-REALIZATION-LOWERING-DESIGN-001
summary=ok
```

## Implementation

This row adds only passive plan metadata:

```text
LocalMapStorageRealizationPlan
  receiver_value
  representation=local_i64_key_map
  candidate_set_count
  candidate_scalar_get_count
  publication_materialization_required=1
  backend_lowering_enabled=0
  runtime_helper_enabled=0
```

The plan is emitted from existing `MapReprPlan` / generic-method route evidence
for receivers that already have the local i64 shadow shape. It does not change
lowering.

## Stop Lines

- no backend lowering
- no runtime helper enablement
- no product `MapBox` storage change
- no product hasher swap
- no sidecar storage
- no MIRBuilder map storage ownership
- no performance winner claim

## Validation

```bash
cargo fmt --check
cargo test --lib mir::map_repr_plan::tests::refresh_function_map_repr_plans_emits_local_i64_key_map_shadow_rows
cargo test --lib runner::mir_json_emit::tests::map_repr_plans::build_mir_json_root_emits_local_map_storage_realization_plans
bash tools/checks/k2_wide_phase296x_local_i64_map_storage_realization_guard_surface_guard.sh
bash tools/checks/current_state_pointer_guard.sh
git diff --check
```
