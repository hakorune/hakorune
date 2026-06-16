# 296x-964 MIMALLOC-FRESH-FRONT-SELECTION-AFTER-ARRAY-TEXT-CLOSEOUT-001

Status: Landed
Date: 2026-06-16

## Purpose

Select a fresh optimization front after closing
`kilo_leaf_array_string_len` in 296x-963.

This row is measurement-only. It does not patch compiler lowering, runtime
helpers, ObjectPlan, RoutePlan, product runtime behavior, or benchmark source.

## Measurement

Command shape:

```bash
for key in \
  kilo_leaf_array_string_indexof_const \
  kilo_leaf_array_rmw_add1 \
  kilo_leaf_map_getset_has \
  kilo_micro_indexof_line \
  kilo_micro_substring_only \
  kilo_micro_substring_concat \
  kilo_micro_substring_views_only \
  kilo_micro_len_substring_views \
  kilo_micro_concat_const_suffix \
  kilo_micro_concat_hh_len \
  kilo_meso_substring_concat_len \
  kilo_meso_indexof_append_array_set
do
  bash tools/perf/bench_micro_c_vs_aot_stat.sh "$key" 1 3 || true
done
```

Observed summary:

```text
kilo_leaf_array_string_indexof_const:
  c_instr=37326776
  c_cycles=5773338
  c_ms=4
  ny_aot_instr=109926040
  ny_aot_cycles=33004811
  ny_aot_ms=10
  ratio_instr=0.34
  ratio_cycles=0.17
  ratio_ms=0.40
  aot_status=ok

kilo_leaf_array_rmw_add1:
  aot_status=skip
  reason=emit_helper_retry_failed
  stage=emit_retry

kilo_leaf_map_getset_has:
  c_instr=10125032
  c_cycles=2193062
  ny_aot_instr=476488
  ny_aot_cycles=743056
  ratio_instr=21.25
  ratio_cycles=2.95
  aot_status=ok

kilo_micro_indexof_line:
  c_instr=39039317
  c_cycles=7213680
  ny_aot_instr=116287835
  ny_aot_cycles=38602857
  ratio_instr=0.34
  ratio_cycles=0.19
  aot_status=ok

kilo_micro_substring_only:
  c_instr=1625031
  c_cycles=492570
  ny_aot_instr=473574
  ny_aot_cycles=729242
  ratio_instr=3.43
  ratio_cycles=0.68
  aot_status=ok

kilo_micro_substring_concat:
  c_instr=1625031
  c_cycles=486841
  ny_aot_instr=5272137
  ny_aot_cycles=5543051
  ratio_instr=0.31
  ratio_cycles=0.09
  aot_status=ok

kilo_micro_substring_views_only:
  c_instr=125032
  c_cycles=191189
  ny_aot_instr=471937
  ny_aot_cycles=729090
  ratio_instr=0.26
  ratio_cycles=0.26
  aot_status=ok

kilo_micro_len_substring_views:
  aot_status=skip
  reason=emit_helper_retry_failed
  stage=emit_retry

kilo_micro_concat_const_suffix:
  c_instr=3125037
  c_cycles=790362
  ny_aot_instr=2271993
  ny_aot_cycles=1298773
  ratio_instr=1.38
  ratio_cycles=0.61
  aot_status=ok

kilo_micro_concat_hh_len:
  c_instr=4125032
  c_cycles=1000046
  ny_aot_instr=476847
  ny_aot_cycles=719964
  ratio_instr=8.65
  ratio_cycles=1.39
  aot_status=ok

kilo_meso_substring_concat_len:
  c_instr=1025036
  c_cycles=377017
  ny_aot_instr=471764
  ny_aot_cycles=730854
  ratio_instr=2.17
  ratio_cycles=0.52
  aot_status=ok

kilo_meso_indexof_append_array_set:
  c_instr=102311899
  c_cycles=24641575
  ny_aot_instr=46127814323
  ny_aot_cycles=25115691397
  ratio_instr=0.00
  ratio_cycles=0.00
  aot_status=ok
```

`ratio_*` is `C / Hako`. Values below `1.0` mean Hako remains slower.

## Selection

Selected front:

```text
kilo_leaf_array_string_indexof_const
```

Reason:

```text
leaf front
valid C pair
still strongly Hako-slower
simple read-only array text indexOf shape
already-visible exact-AOT session helper boundary
```

Micro-ASM confirmation:

```bash
KEEP_PERF_MICROASM_ARTIFACTS=1 PERF_MICROASM_RUNNER_MODE=direct \
  bash tools/perf/bench_micro_aot_asm.sh kilo_leaf_array_string_indexof_const ny_main 3
```

Observed:

```text
event_count_approx=36050108
top_symbol=memchr::arch::x86_64::memchr::memchr_raw::find_avx2
top_symbol_percent=65.53
secondary_symbol=std::thread::local::LocalKey<T>::with
secondary_symbol_percent=32.52
```

`ny_main` shape:

```text
build 64 text slots
loop 400000:
  row = i & 63
  call hako.array_text.session_indexof_const_utf8(handle, row, "line", 4)
  hits += (pos >= 0)
return hits + array.len
```

## Not Selected

```text
kilo_meso_indexof_append_array_set:
  rejected_for_now=broad_meso_work_explosion
  reason=mutating append + array.set + indexOf + length; too broad for the
         first post-closeout owner

kilo_micro_substring_concat:
  rejected_for_now=string_alloc_concat_front
  reason=strong Hako-slower but not as close to the just-closed array-text lane

kilo_leaf_array_rmw_add1 / kilo_micro_len_substring_views:
  rejected_for_now=emit_helper_retry_failed

kilo_leaf_map_getset_has / kilo_micro_concat_hh_len:
  rejected_for_now=not_hako_slower_on_cycles
```

## Result

```text
output_contract=hako-mimalloc-fresh-front-selection-after-array-text-closeout-v0
source_evidence=296x-963,fresh-repeat3-2026-06-16
row_kind=selection
implementation_started=0
perf_first_required=1

previous_front=kilo_leaf_array_string_len
previous_front_closed=1
previous_front_exact_counter_winner=1

candidate_front_count=12
selected_front=kilo_leaf_array_string_indexof_const
selected_owner_family=array_text_indexof_const_session_boundary
selected_reason=leaf_hako_slower_read_only_array_text_indexof

selected_c_instr=37326776
selected_c_cycles=5773338
selected_c_ms=4
selected_ny_aot_instr=109926040
selected_ny_aot_cycles=33004811
selected_ny_aot_ms=10
selected_ratio_instr=0.34
selected_ratio_cycles=0.17
selected_ratio_ms=0.40
selected_aot_status=ok

selected_microasm_event_count_approx=36050108
selected_microasm_top_symbol=memchr_find_avx2
selected_microasm_top_symbol_percent=65.53
selected_microasm_secondary_symbol=thread_local_key_with
selected_microasm_secondary_symbol_percent=32.52

backend_lowering_changed=0
runtime_helper_changed=0
product_default_changed=0
benchmark_source_changed=0
helper_name_inference_enabled=0

selected_next=MIMALLOC-ARRAY-TEXT-INDEXOF-CONST-OWNER-INVENTORY-001
summary=ok
```

## Stop Line

```text
do not patch indexOf helpers before owner inventory
do not select the broad meso indexOf+append+set front first
do not infer keeper from memchr symbol alone
do not reopen array length loop-session lane
do not change ArrayBox/StringBox storage in this selection row
do not change benchmark sources
```
