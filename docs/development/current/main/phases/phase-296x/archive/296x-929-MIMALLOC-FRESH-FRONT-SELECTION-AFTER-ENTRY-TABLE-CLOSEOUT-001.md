# 296x-929 MIMALLOC-FRESH-FRONT-SELECTION-AFTER-ENTRY-TABLE-CLOSEOUT-001

Status: Landed
Date: 2026-06-16

## Purpose

Select the next optimization front after LocalI64Map entry-table materialization
closed `kilo_leaf_map_get_dynamic_covered_i64` as a hot-loop success.

This row is selection-only. It does not patch compiler lowering, runtime
helpers, ObjectPlan, RoutePlan, product runtime behavior, or benchmark source.

## Measurement

Command shape:

```bash
for key in \
  kilo_leaf_array_string_len \
  kilo_leaf_array_string_indexof_const \
  kilo_leaf_map_get_dynamic_covered_i64 \
  kilo_leaf_map_getset_has
do
  bash tools/perf/bench_micro_c_vs_aot_stat.sh "$key" 1 1 || true
done
```

Observed summary:

```text
kilo_leaf_array_string_len:
  c_instr=14526802
  c_cycles=3237216
  c_ms=5
  ny_aot_instr=92925688
  ny_aot_cycles=32626920
  ny_aot_ms=10
  ratio_instr=0.16
  ratio_cycles=0.10
  ratio_ms=0.50
  aot_status=ok

kilo_leaf_array_string_indexof_const:
  c_instr=37326771
  c_cycles=5719449
  c_ms=4
  ny_aot_instr=109926387
  ny_aot_cycles=33037474
  ny_aot_ms=10
  ratio_instr=0.34
  ratio_cycles=0.17
  ratio_ms=0.40
  aot_status=ok

kilo_leaf_map_get_dynamic_covered_i64:
  aot_status=skip
  reason=c_benchmark_missing

kilo_leaf_map_getset_has:
  c_instr=10125034
  c_cycles=2215275
  c_ms=4
  ny_aot_instr=476735
  ny_aot_cycles=1055777
  ny_aot_ms=4
  ratio_instr=21.24
  ratio_cycles=2.10
  ratio_ms=1.00
  aot_status=ok
```

`ratio_*` is `C / Hako`. Values below `1.0` mean Hako remains slower.

## Selection

`kilo_leaf_array_string_len` is selected again as the next fresh front. It is
simple, still Hako-slower, and already has a parked design surface:

```text
296x-851 MIMALLOC-ARRAY-TEXT-LOOP-SESSION-PLAN-SURFACE-001
```

The next row should continue from that parked surface with an inventory row:

```text
MIMALLOC-ARRAY-TEXT-LOOP-SESSION-PLAN-INVENTORY-001
```

`kilo_leaf_array_string_indexof_const` remains secondary because it is broader
than length-only. `kilo_leaf_map_get_dynamic_covered_i64` has no matched C pair
and is closed for the current entry-table lane. `kilo_leaf_map_getset_has` is
not selected because the current AOT route is already smaller than the C pair
and this row is looking for the next Hako-slower exact front.

## Result

```text
output_contract=hako-mimalloc-fresh-front-selection-after-entry-table-closeout-v0
source_evidence=296x-928,fresh-repeat1-2026-06-16
row_kind=selection
implementation_started=0
perf_first_required=1

previous_front=kilo_leaf_map_get_dynamic_covered_i64
previous_front_closed=1
previous_front_hot_loop_map_helper_call_count=0

candidate_front_count=4
selected_front=kilo_leaf_array_string_len
selected_owner_family=array_text_slot_len_loop_local_session_boundary
selected_reason=simple_leaf_hako_slower_and_parked_plan_surface_exists

selected_c_instr=14526802
selected_c_cycles=3237216
selected_c_ms=5
selected_ny_aot_instr=92925688
selected_ny_aot_cycles=32626920
selected_ny_aot_ms=10
selected_ratio_instr=0.16
selected_ratio_cycles=0.10
selected_ratio_ms=0.50
selected_aot_status=ok

secondary_front=kilo_leaf_array_string_indexof_const
secondary_ratio_instr=0.34
secondary_ratio_cycles=0.17
secondary_aot_status=ok

closed_front=kilo_leaf_map_get_dynamic_covered_i64
closed_front_status=skip
closed_front_reason=c_benchmark_missing

not_selected_front=kilo_leaf_map_getset_has
not_selected_reason=not_hako_slower_exact_front
not_selected_ratio_cycles=2.10

backend_lowering_changed=0
runtime_helper_changed=0
product_default_changed=0
benchmark_source_changed=0
helper_name_inference_enabled=0

selected_next=MIMALLOC-ARRAY-TEXT-LOOP-SESSION-PLAN-INVENTORY-001
summary=ok
```

## Stop Line

```text
do not reopen LocalI64Map entry-table work from this row
do not optimize indexOf before the length-only front is re-inventoried
do not infer owner from helper symbol spelling
do not change ArrayBox/StringBox storage
do not add raw array text session FFI
do not touch MIRBuilder object management
do not claim Hako-vs-C winner from this selection row
```
