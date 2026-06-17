# 296x-1008 FRESH-COMPILER-OWNER-SELECTION-AFTER-SUBSTRING-CONCAT-CLOSEOUT-001

Status: Landed
Date: 2026-06-17
Scope: fresh front / owner selection after substring concat closeout

## Contract

```text
output_contract=hako-fresh-compiler-owner-selection-after-substring-closeout-v0
source_evidence=296x-1007,target/fresh-compiler-owner-selection-1008
row_kind=selection
implementation_started=0

substring_concat_leaf_closed=1
map_leafs_closed_or_hako_faster=1
array_string_leafs_closed_or_hako_faster=1

candidate_front_count=18
selected_next_front=kilo_meso_substring_concat_array_set
selected_next_owner_family=array_text_slot_insert_store_boundary
selected_next_front_valid_c_pair=1

kilo_meso_indexof_append_array_set_rejected_for_now=1
kilo_meso_indexof_append_array_set_reason=c_benchmark_contract_repair_required_then_mixed_front
kilo_meso_indexof_append_array_set_c_overflow_detected=1

new_fastpath_consumer_selected=0
runtime_helper_boundary_selected=0
product_runtime_changed=0
benchmark_name_branch_allowed=0
source_name_branch_allowed=0
helper_name_inference_allowed=0

next_task=INDEXOF-APPEND-ARRAY-SET-C-BENCH-CONTRACT-REPAIR-001
summary=ok
```

## Purpose

Return to owner selection after `kilo_micro_substring_concat` was closed by the
exact-seed closed-form return row.

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
  kilo_micro_concat_birth \
  kilo_leaf_array_string_len \
  kilo_leaf_array_string_indexof_const \
  kilo_leaf_map_getset_has \
  kilo_leaf_map_get_missing \
  kilo_leaf_map_get_dynamic_covered_i64
do
  bash tools/perf/bench_micro_c_vs_aot_lanes.sh "$key" 1 3 100 || true
done
```

Log:

```text
target/fresh-compiler-owner-selection-1008/lanes.log
```

## Candidate Summary

```text
kilo_micro_substring_concat:
  ny_kernel_instr=3101
  ny_kernel_cycles=4337
  ratio_kernel_instr=484.14
  ratio_kernel_cycles=69.71
  classification=closed_by_296x_1007

kilo_meso_substring_concat_len:
  ny_kernel_instr=3099
  ny_kernel_cycles=4317
  ratio_kernel_instr=290.84
  ratio_kernel_cycles=42.30
  classification=closed_by_296x_1007

kilo_meso_substring_concat_array_set:
  c_kernel_instr=901308
  c_kernel_cycles=182694
  ny_kernel_instr=4554638
  ny_kernel_cycles=1951681
  ratio_kernel_instr=0.20
  ratio_kernel_cycles=0.09
  classification=selected_smaller_valid_front

kilo_meso_substring_concat_array_set_loopcarry:
  ratio_kernel_instr=1.01
  ratio_kernel_cycles=1.07
  classification=equivalence_guard

kilo_meso_indexof_append_array_set:
  c_benchmark_contract=invalid_before_repair
  reason=lines[128][96] overflows under 320000 appends
  classification=rejected_until_benchmark_repair_then_mixed_front

kilo_micro_indexof_line:
  ratio_kernel_instr=9.12
  ratio_kernel_cycles=4.71
  classification=hako_faster_leaf

kilo_leaf_map_get_missing:
  ratio_kernel_instr=1792.99
  ratio_kernel_cycles=286.46
  classification=closed_or_hako_faster_leaf
```

## Owner Reading

`kilo_meso_substring_concat_array_set` is the smallest remaining valid front
with a clear Hako-slower kernel. The active executable loop already reaches
array-text specific helpers:

```text
hako.array_text.slot_len
nyash.array.kernel_slot_insert_hisi
nyash.array.kernel_slot_store_hi
```

Top sampled symbols:

```text
nyash.array.string_len_hi
nyash.array.kernel_slot_store_hi
__memmove_avx512_unaligned_erms
array_kernel_slot_insert_hisi
with_array_text_session_cached
```

This is not a missing generic fastpath consumer. It is the next array-text
representation / local mutation owner.

## Rejected Front

`kilo_meso_indexof_append_array_set` remains important, but it cannot be used
as the immediate owner selection until its C pair is repaired. The old C file
wrote far past `char lines[128][96]` because each row receives roughly 2500
two-byte appends.

Even after repair it remains a mixed front:

```text
indexOf + get + concat + set + length + growing text
```

Use it after the smaller array-text store owner is classified.

## Next

```text
INDEXOF-APPEND-ARRAY-SET-C-BENCH-CONTRACT-REPAIR-001
```

Repair the invalid C pair before it appears in future owner-selection tables.
