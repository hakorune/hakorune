# 296x-1014 FRESH-COMPILER-OWNER-SELECTION-002

Status: Landed
Date: 2026-06-17
Scope: fresh front / owner selection after array-text store local mutation keeper

## Contract

```text
output_contract=hako-fresh-compiler-owner-selection-v2
source_evidence=296x-1012,296x-1013,target/fresh-compiler-owner-selection-1014
row_kind=selection
implementation_started=0

array_text_store_local_mutation_keeper_closed=1
copy_noise_origin_classified=1
broad_localssa_copy_coalescing_reopened=0
receiver_materialization_reopened=0

candidate_front_count=4
selected_next_front=kilo_meso_indexof_append_array_set
selected_next_owner_family=text_materialization_allocator_boundary
selected_next_owner_confidence=medium
selected_next_task=INDEXOF-APPEND-MATERIALIZATION-BOUNDARY-INVENTORY-001

fresh_narrow_compiler_fastpath_selected=0
fresh_runtime_materialization_inventory_selected=1
product_runtime_changed=0
benchmark_name_branch_allowed=0
source_name_branch_allowed=0
helper_name_inference_allowed=0

next_task=INDEXOF-APPEND-MATERIALIZATION-BOUNDARY-INVENTORY-001
summary=ok
```

## Purpose

Return to perf-first owner selection after `kilo_meso_substring_concat_array_set`
was reduced to a len-only array-text route and the remaining Copy noise was
classified as normal SSA carrier noise.

This row does not open another implementation. It selects the next owner family
from current measured evidence.

## Measurement

Command:

```bash
mkdir -p target/fresh-compiler-owner-selection-1014
for key in \
  kilo_meso_substring_concat_array_set \
  kilo_meso_substring_concat_array_set_loopcarry \
  kilo_meso_substring_concat_len \
  kilo_meso_indexof_append_array_set
do
  echo "==== ${key} ===="
  bash tools/perf/bench_micro_c_vs_aot_lanes.sh "$key" 1 3 100
done | tee target/fresh-compiler-owner-selection-1014/lanes.log

KEEP_PERF_MICROASM_ARTIFACTS=1 PERF_MICROASM_RUNNER_MODE=direct \
  bash tools/perf/bench_micro_aot_asm.sh \
  kilo_meso_indexof_append_array_set ny_main 1 \
  | tee target/fresh-compiler-owner-selection-1014/indexof_append_array_set_microasm.log
```

## Candidate Summary

```text
kilo_meso_substring_concat_array_set:
  c_kernel_instr=901308
  c_kernel_cycles=183576
  ny_kernel_instr=565498
  ny_kernel_cycles=197205
  ratio_kernel_instr=1.59
  ratio_kernel_cycles=0.93
  classification=closed_by_296x_1012

kilo_meso_substring_concat_array_set_loopcarry:
  c_kernel_instr=901308
  c_kernel_cycles=182091
  ny_kernel_instr=893585
  ny_kernel_cycles=171656
  ratio_kernel_instr=1.01
  ratio_kernel_cycles=1.06
  classification=equivalence_guard

kilo_meso_substring_concat_len:
  c_kernel_instr=901308
  c_kernel_cycles=183540
  ny_kernel_instr=3102
  ny_kernel_cycles=4354
  ratio_kernel_instr=290.56
  ratio_kernel_cycles=42.15
  classification=closed_form_exact_seed_closed

kilo_meso_indexof_append_array_set:
  c_kernel_instr=36362861
  c_kernel_cycles=6039862
  ny_kernel_instr=461048790
  ny_kernel_cycles=249686593
  ratio_kernel_instr=0.08
  ratio_kernel_cycles=0.02
  classification=selected_broad_runtime_materialization_front
```

## Owner Reading

`kilo_meso_indexof_append_array_set` remains the only measured candidate with a
large Hako-slower kernel gap after the array-text store keeper.

The executable loop still contains a mixed route:

```asm
call hako.array_text.session_indexof_const_utf8
test %rax,%rax
js ...
call nyash.array.get_hi
call nyash.string.concat_hh
call nyash.array.set_his
call nyash.string.len_fast_h
```

However, the top sampled owners are not a single missing compiler metadata
consumer:

```text
malloc_consolidate=34.50%
ArrayBox::boxed_from_text=19.55%
_int_malloc=16.32%
__memmove_avx512_unaligned_erms=13.62%
unlink_chunk=4.92%
malloc=1.81%
```

This is a text materialization / allocator boundary front. It should not be
treated as a narrow compiler fastpath row without a separate inventory that
proves a specific route seam.

## Decision

Select:

```text
selected_next_front=kilo_meso_indexof_append_array_set
selected_next_owner_family=text_materialization_allocator_boundary
selected_next_task=INDEXOF-APPEND-MATERIALIZATION-BOUNDARY-INVENTORY-001
```

The next row must inventory the materialization boundary before implementation:

```text
boxed_from_text traffic
concat_hh allocation traffic
array.get / array.set publication traffic
text-session reuse opportunities
allocator / memmove share
compiler-route candidate count
```

## Not Selected

```text
kilo_meso_substring_concat_array_set:
  not selected because 296x-1012 closed the active store helpers and the
  remaining delta is C-like enough for this lane.

kilo_meso_substring_concat_array_set_loopcarry:
  not selected because it remains an equivalence guard.

kilo_meso_substring_concat_len:
  not selected because it is already closed-form exact-seed output.

LocalSSA / receiver materialization:
  not selected because 296x-1013 classified the observed Copy as normal SSA
  carrier noise for the already-consumed array-text route.
```

## Stop Line

```text
do not reopen broad LocalSSA copy coalescing
do not change receiver materialization from this row
do not infer a fastpath from helper symbols alone
do not branch by benchmark/source/helper name
do not implement text materialization changes before inventory
do not claim a winner from selection-only evidence
```

## Next

```text
INDEXOF-APPEND-MATERIALIZATION-BOUNDARY-INVENTORY-001
```

Inventory whether the selected broad front has a narrow compiler-route seam or
must be treated as a runtime/text-storage/allocator boundary.
