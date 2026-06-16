# 296x-844 MIMALLOC-FRESH-FRONT-SELECTION-AFTER-MAP-MISSING-EMPTY-CLOSEOUT-001

Status: Landed
Date: 2026-06-16

## Purpose

Select the next fresh optimization front after
`kilo_leaf_map_get_missing` was closed by `MapMissingEmptyRoute`.

This row is measurement-only. It must not patch compiler lowering, runtime
helpers, ObjectPlan, RoutePlan, product runtime behavior, or benchmark source.

## Measurement

Command shape:

```bash
for key in \
  kilo_leaf_array_string_len \
  kilo_leaf_array_string_indexof_const \
  kilo_leaf_array_rmw_add1 \
  kilo_leaf_map_get_missing
do
  bash tools/perf/bench_micro_c_vs_aot_stat.sh "$key" 1 3 || true
done
```

Observed summary:

```text
kilo_leaf_array_string_len:
  c_instr=14526802
  c_cycles=3223732
  c_ms=4
  ny_aot_instr=92925832
  ny_aot_cycles=32183346
  ny_aot_ms=10
  ratio_instr=0.16
  ratio_cycles=0.10
  ratio_ms=0.40
  aot_status=ok

kilo_leaf_array_string_indexof_const:
  c_instr=37326771
  c_cycles=5730605
  c_ms=4
  ny_aot_instr=109926717
  ny_aot_cycles=33112732
  ny_aot_ms=10
  ratio_instr=0.34
  ratio_cycles=0.17
  ratio_ms=0.40
  aot_status=ok

kilo_leaf_array_rmw_add1:
  aot_status=skip
  reason=emit_helper_retry_failed
  stage=emit_retry

kilo_leaf_map_get_missing:
  c_instr=10125076
  c_cycles=2190004
  c_ms=4
  ny_aot_instr=473153
  ny_aot_cycles=751842
  ny_aot_ms=3
  ratio_instr=21.40
  ratio_cycles=2.91
  ratio_ms=1.33
  aot_status=ok
```

`ratio_*` is `C / Hako`. Values below `1.0` mean Hako remains slower.

## Selection

`kilo_leaf_array_string_len` is selected as the next fresh leaf front because it
is simpler than the `indexOf` front and still Hako-slower after map-missing
closeout.

`kilo_leaf_array_rmw_add1` is not selected because it fails before comparable
perf measurement with `emit_helper_retry_failed`.

`kilo_leaf_map_get_missing` remains closed.

## Result

```text
output_contract=hako-mimalloc-fresh-front-selection-after-map-missing-empty-closeout-v0
source_evidence=296x-843,leaf-repeat3-2026-06-16
row_kind=selection
implementation_started=0
perf_first_required=1

previous_front=kilo_leaf_map_get_missing
previous_front_closed=1
previous_front_route_winner_claim=1

candidate_front_count=4
selected_front=kilo_leaf_array_string_len
selected_owner_family=array_string_len_runtime_boundary_inventory
selected_reason=simpler_leaf_hako_slower_before_indexof

selected_c_instr=14526802
selected_c_cycles=3223732
selected_c_ms=4
selected_ny_aot_instr=92925832
selected_ny_aot_cycles=32183346
selected_ny_aot_ms=10
selected_ratio_instr=0.16
selected_ratio_cycles=0.10
selected_ratio_ms=0.40
selected_aot_status=ok

secondary_front=kilo_leaf_array_string_indexof_const
secondary_ratio_instr=0.34
secondary_ratio_cycles=0.17
secondary_aot_status=ok

blocked_front=kilo_leaf_array_rmw_add1
blocked_front_status=skip
blocked_front_reason=emit_helper_retry_failed

closed_front=kilo_leaf_map_get_missing
closed_front_ratio_instr=21.40
closed_front_ratio_cycles=2.91
closed_front_status=ok

backend_lowering_changed=0
runtime_helper_changed=0
product_default_changed=0
benchmark_source_changed=0
helper_name_inference_enabled=0

selected_next=MIMALLOC-LEAF-ARRAY-STRING-LEN-OWNER-INVENTORY-001
summary=ok
```

## Stop Line

```text
do not patch Array/String helpers before owner inventory
do not infer a keeper from helper or method names
do not select indexOf before the simpler length front is inventoried
do not reopen map missing unless fresh regression evidence appears
do not treat emit_helper_retry_failed as a perf owner
do not change product runtime or benchmark source
```
