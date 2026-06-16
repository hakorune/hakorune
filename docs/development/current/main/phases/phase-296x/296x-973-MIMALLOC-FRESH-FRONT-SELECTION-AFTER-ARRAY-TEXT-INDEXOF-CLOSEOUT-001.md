# 296x-973 MIMALLOC-FRESH-FRONT-SELECTION-AFTER-ARRAY-TEXT-INDEXOF-CLOSEOUT-001

Status: Landed
Date: 2026-06-17

## Purpose

Select the next optimization front after closing
`kilo_leaf_array_string_indexof_const` in 296x-972.

This row is measurement-only. It does not patch compiler lowering, runtime
helpers, RoutePlan, ObjectPlan, product runtime behavior, or benchmark source.

## Measurement

Command shape:

```bash
for key in \
  kilo_leaf_array_rmw_add1 \
  kilo_meso_substring_concat_array_set \
  kilo_meso_substring_concat_array_set_loopcarry \
  kilo_meso_indexof_append_array_set \
  kilo_meso_substring_concat_len \
  kilo_micro_len_substring_views \
  kilo_micro_substring_concat \
  kilo_micro_concat_hh_len \
  kilo_micro_array_string_store \
  kilo_micro_indexof_line \
  kilo_micro_substring_only \
  kilo_micro_substring_views_only \
  kilo_micro_concat_birth
do
  bash tools/perf/bench_micro_c_vs_aot_lanes.sh "$key" 1 3 100 || true
done
```

Logs:

```text
target/perf_front_select_after_indexof/lanes.log
target/perf_front_select_after_indexof/lanes_substring.log
```

## Candidate Summary

```text
kilo_leaf_array_rmw_add1:
  aot_status=skip
  reason=emit_helper_retry_failed

kilo_meso_substring_concat_array_set:
  c_kernel_instr=901308
  c_kernel_cycles=182088
  ny_kernel_instr=4554637
  ny_kernel_cycles=1943177
  ratio_kernel_instr=0.20
  ratio_kernel_cycles=0.09
  classification=hako_slower_composite_front

kilo_meso_substring_concat_array_set_loopcarry:
  ratio_kernel_instr=1.01
  ratio_kernel_cycles=1.06
  classification=equivalence_guard

kilo_meso_indexof_append_array_set:
  c_kernel_instr=102650115
  c_kernel_cycles=24438173
  ny_kernel_instr=461052753
  ny_kernel_cycles=251242472
  ratio_kernel_instr=0.22
  ratio_kernel_cycles=0.10
  classification=hako_slower_broad_meso_front

kilo_meso_substring_concat_len:
  ratio_kernel_instr=290.65
  ratio_kernel_cycles=41.29
  classification=hako_faster_or_folded_route

kilo_micro_len_substring_views:
  aot_status=skip
  reason=emit_helper_retry_failed

kilo_micro_substring_concat:
  c_kernel_instr=1501307
  c_kernel_cycles=303776
  ny_kernel_instr=4803111
  ny_kernel_cycles=4806781
  ratio_kernel_instr=0.31
  ratio_kernel_cycles=0.06
  classification=selected_leaf_hako_slower_front

kilo_micro_concat_hh_len:
  ratio_kernel_instr=596.32
  ratio_kernel_cycles=106.33
  classification=hako_faster_or_folded_route

kilo_micro_array_string_store:
  ratio_kernel_instr=12.65
  ratio_kernel_cycles=42.77
  classification=hako_faster

kilo_micro_indexof_line:
  ratio_kernel_instr=9.12
  ratio_kernel_cycles=4.74
  classification=hako_faster_after_indexof_rows

kilo_micro_substring_only:
  ratio_kernel_instr=272.72
  ratio_kernel_cycles=38.94
  classification=hako_faster_or_folded_route

kilo_micro_substring_views_only:
  ratio_kernel_instr=0.42
  ratio_kernel_cycles=0.45
  classification=hako_slower_but_too_small

kilo_micro_concat_birth:
  ratio_kernel_instr=17656.04
  ratio_kernel_cycles=2074.12
  classification=hako_faster_or_folded_route
```

## Selection

Selected front:

```text
kilo_micro_substring_concat
```

Reason:

```text
leaf front
valid C pair
strong resident-kernel Hako-slower evidence
smaller owner surface than meso append/set fronts
not already closed by array-text len/indexOf region rows
```

Micro-ASM confirmation:

```bash
KEEP_PERF_MICROASM_ARTIFACTS=1 PERF_MICROASM_RUNNER_MODE=direct \
  bash tools/perf/bench_micro_aot_asm.sh kilo_micro_substring_concat ny_main 3
```

Observed `ny_main` is still a tight loop doing stack byte moves for the
substring/concat/rotation shape. The next row must classify whether the owner is
string-region representation, repeated materialization, or C-pair foldability.

## Not Selected

```text
kilo_meso_indexof_append_array_set:
  rejected_for_now=broad_composite_front
  reason=indexOf + append + array.set + large cache footprint; inventory later
         only after a smaller string/substring owner is closed or rejected

kilo_meso_substring_concat_array_set:
  rejected_for_now=composite_front
  reason=Hako-slower but array.set publication/mutation is mixed with the
         substring/concat owner

kilo_micro_substring_views_only:
  rejected_for_now=too_small_for_next_owner
  reason=Hako-slower but kernel is only a few thousand cycles/instructions
```

## Result

```text
output_contract=hako-mimalloc-fresh-front-selection-after-array-text-indexof-closeout-v0
row_kind=selection
implementation_started=0
perf_first_required=1

previous_front=kilo_leaf_array_string_indexof_const
previous_front_closed=1

candidate_front_count=13
selected_front=kilo_micro_substring_concat
selected_owner_family=substring_concat_region_owner_inventory
selected_reason=leaf_hako_slower_substring_concat

selected_c_kernel_instr=1501307
selected_c_kernel_cycles=303776
selected_ny_kernel_instr=4803111
selected_ny_kernel_cycles=4806781
selected_ratio_kernel_instr=0.31
selected_ratio_kernel_cycles=0.06
selected_aot_status=ok

product_default_changed=0
provider_activation_changed=0
backend_lowering_changed=0
winner_claim=0

selected_next=MIMALLOC-SUBSTRING-CONCAT-OWNER-INVENTORY-001
summary=ok
```

## Stop Line

```text
do not implement from substring/helper names
do not touch StringBox storage before owner inventory
do not select meso append/set fronts before the leaf owner inventory
do not infer a keeper from C folding alone
do not change product runtime or provider activation
```
