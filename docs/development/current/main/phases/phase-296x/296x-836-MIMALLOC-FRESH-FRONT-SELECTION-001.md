# 296x-836 MIMALLOC-FRESH-FRONT-SELECTION-001

Status: Landed
Date: 2026-06-16

## Purpose

Select a fresh optimization front after the previous
`objectLifecycleSmallAlloc/1` front stopped being Hako-slower.

This row is measurement-only. It must not patch compiler lowering, runtime
helpers, MapBox, ObjectPlan, RoutePlan, or product runtime behavior.

## Measurement

Command shape:

```bash
bash tools/perf/build_perf_release.sh
bash tools/perf/bench_micro_c_vs_aot_lanes.sh <bench> 1 3 100
PERF_MICROASM_RUNNER_MODE=direct KEEP_PERF_MICROASM_ARTIFACTS=1 \
  bash tools/perf/bench_micro_aot_asm.sh kilo_leaf_map_get_missing ny_main 3
```

Logs:

```text
target/perf_front_select_835/lanes_20260616_031240.log
target/perf_front_select_835/lanes_extra_20260616_031322.log
target/perf_front_select_835/asm_map_get_missing_20260616_031755.log
```

## Candidate Summary

Resident kernel lane:

```text
kilo_micro_array_getset:
  ratio_kernel_cycles=0.99
  classification=equivalence_guard

kilo_micro_userbox_point_add:
  ratio_kernel_cycles=1.02
  classification=equivalence_guard

kilo_micro_concat_const_suffix:
  ratio_kernel_cycles=0.99
  classification=equivalence_guard

kilo_micro_userbox_flag_toggle:
  ratio_kernel_cycles=1.76
  classification=hako_faster_or_previous_keeper_green

kilo_micro_array_string_store:
  ratio_kernel_cycles=44.22
  classification=hako_faster_or_c_pair_not_next_owner

kilo_micro_indexof_line:
  ratio_kernel_cycles=4.74
  classification=process_total_vs_resident_kernel_route_split

kilo_leaf_array_rmw_add1:
  aot_status=skip
  classification=emit_helper_retry_failed

kilo_leaf_array_string_len:
  ratio_kernel_cycles=2.37
  classification=hako_faster

kilo_leaf_array_string_indexof_const:
  ratio_kernel_cycles=4.54
  classification=hako_faster

kilo_leaf_map_get_missing:
  ratio_kernel_cycles=0.01
  classification=selected_hako_slower_leaf_front

kilo_leaf_map_getset_has:
  ratio_kernel_cycles=214.71
  classification=hako_faster_or_folded_route

kilo_meso_substring_concat_len:
  ratio_kernel_cycles=41.47
  classification=hako_faster_or_folded_route

kilo_meso_substring_concat_array_set:
  ratio_kernel_cycles=0.09
  classification=hako_slower_but_composite_front

kilo_meso_indexof_append_array_set:
  ratio_kernel_cycles=0.10
  classification=hako_slower_but_composite_front
```

The selected fresh front is the simpler leaf map-missing surface:

```text
selected_front=kilo_leaf_map_get_missing
selected_reason=leaf_hako_slower_resident_kernel
primary_lane=resident_kernel
kernel_inner_runs=100
```

The resident kernel lane is meaningfully slower:

```text
c_kernel_cycles=2004686
ny_kernel_cycles=212112419
ratio_kernel_cycles=0.01
c_kernel_instr=10001307
ny_kernel_instr=896005922
ratio_kernel_instr=0.01
```

The selected asm owner is narrow enough to inventory next:

```text
asm_top_symbol_0=nyash_rust::boxes::map_box::MapBox::get_opt_key_str
asm_top_symbol_0_percent=58.32
asm_top_symbol_1=<i64 as alloc::string::SpecToString>::spec_to_string
asm_top_symbol_1_percent=41.67
```

## Result

```text
output_contract=hako-mimalloc-fresh-front-selection-v0
source_evidence=296x-824,296x-835
row_kind=selection
implementation_started=0
perf_first_required=1

fresh_front_selection_allowed=1
previous_front=object_lifecycle_body
previous_front_paused=1

candidate_front_count=14
selected_front=kilo_leaf_map_get_missing
selected_owner_family=map_missing_key_string_lookup_runtime_boundary
selected_reason=leaf_hako_slower_resident_kernel

primary_lane=resident_kernel
kernel_inner_runs=100
selected_ratio_kernel_cycles=0.01
selected_ratio_kernel_instr=0.01
selected_aot_status=ok

asm_top_symbol_0=nyash_rust::boxes::map_box::MapBox::get_opt_key_str
asm_top_symbol_0_percent=58.32
asm_top_symbol_1=<i64 as alloc::string::SpecToString>::spec_to_string
asm_top_symbol_1_percent=41.67

boot_startup_lane_reopened=0
product_nyrt_entry_changed=0
provider_activation_changed=0
backend_lowering_changed=0
product_default_changed=0

selected_next=MIMALLOC-MAP-MISSING-KEY-OWNER-INVENTORY-001
summary=ok
```

## Stop Line

```text
do not patch MapBox before owner inventory
do not optimize from process-total alone
do not select composite meso fronts before leaf owner inventory
do not infer a keeper from method/helper names
do not change product runtime or provider activation
do not resume objectLifecycleSmallAlloc without new Hako-slower evidence
```
