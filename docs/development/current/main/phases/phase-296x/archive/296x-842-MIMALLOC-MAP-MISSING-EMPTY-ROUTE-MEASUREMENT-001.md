# 296x-842 MIMALLOC-MAP-MISSING-EMPTY-ROUTE-MEASUREMENT-001

Status: Landed
Date: 2026-06-16

## Purpose

Measure the narrow `MapMissingEmptyRoute` implementation from 296x-841 on the
selected fresh front before making any closeout or next-owner claim.

This row verifies the product AOT path, not only the Python unit backend:

```text
MIR:
  route_decisions contains MapMissingEmptyRoute at b19.i3

pure-first boundary C emitter:
  consumes selected_route=map_get_missing_empty_const_zero

generated ny_main:
  no nyash.runtime_data.get_hh call remains for the target site
```

## Result

```text
output_contract=hako-mimalloc-map-missing-empty-route-measurement-v0
source_evidence=296x-841
row_kind=measurement
target_front=kilo_leaf_map_get_missing
selected_route=map_get_missing_empty_const_zero
source_plan_kind=MapMissingEmptyRoute

before_ny_aot_instr=896474449
before_ny_aot_cycles=220979722
before_ny_aot_ms=45

after_c_instr=10125074
after_c_cycles=2191922
after_c_ms=4
after_ny_aot_instr=472935
after_ny_aot_cycles=749368
after_ny_aot_ms=3

after_ratio_instr_c_over_hako=21.41
after_ratio_cycles_c_over_hako=2.93
after_ratio_ms_c_over_hako=1.33

ny_main_runtime_data_get_hh_call_count=0
ny_main_map_birth_h_call_count=1
ny_main_return_const=2000000
route_winner_claim=1
kernel_path_closed_for_this_front=1
product_default_changed=0
mapbox_storage_changed=0
mapbox_public_semantics_changed=0

selected_next=MIMALLOC-MAP-MISSING-EMPTY-ROUTE-CLOSEOUT-001
summary=ok
```

The target `ny_main` after rebuild:

```asm
000000000040e700 <ny_main>:
  40e700: 50                    push   %rax
  40e701: e8 3a 1f 00 00        call   410640 <nyash.map.birth_h>
  40e706: b8 80 84 1e 00        mov    $0x1e8480,%eax
  40e70b: 59                    pop    %rcx
  40e70c: c3                    ret
```

## Proof

```bash
bash tools/perf/build_perf_release.sh
bash tools/perf/bench_micro_c_vs_aot_stat.sh kilo_leaf_map_get_missing 1 3
bash tools/perf/bench_micro_aot_asm.sh kilo_leaf_map_get_missing ny_main 3
bash tools/checks/k2_wide_phase296x_map_missing_empty_route_measurement_guard.sh
```

Observed stat:

```text
[microstat] name=kilo_leaf_map_get_missing c_instr=10125074 c_cycles=2191922 c_cache_miss=3265 c_ms=4 ny_aot_instr=472935 ny_aot_cycles=749368 ny_aot_cache_miss=10765 ny_aot_ms=3 ratio_instr=21.41 ratio_cycles=2.93 ratio_ms=1.33 c_ipc=4.62 ny_aot_ipc=0.63 aot_status=ok
```

## Stop Line

```text
do not generalize to all MapGet calls
do not remove map birth in this row
do not claim MapBox storage changed
do not claim product default changed
do not infer future routes from helper names
do not open another front before closeout
```
