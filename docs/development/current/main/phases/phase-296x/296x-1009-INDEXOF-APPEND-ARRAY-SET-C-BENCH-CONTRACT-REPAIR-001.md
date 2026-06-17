# 296x-1009 INDEXOF-APPEND-ARRAY-SET-C-BENCH-CONTRACT-REPAIR-001

Status: Landed
Date: 2026-06-17
Scope: benchmark contract repair

## Contract

```text
output_contract=hako-indexof-append-array-set-c-bench-contract-repair-v0
source_evidence=296x-1008
row_kind=benchmark_repair

target_front=kilo_meso_indexof_append_array_set
c_benchmark_overflow_before=1
c_lines_capacity_before=96
c_rows=128
c_ops=320000
c_append_bytes_per_hit=2
c_max_appends_per_row=2500
c_required_line_capacity=5017

c_benchmark_overflow_after=0
c_capacity_formula_enabled=1
hako_source_changed=0
compiler_lowering_changed=0
product_runtime_changed=0

post_repair_c_kernel_instr=36362861
post_repair_c_kernel_cycles=6329251
post_repair_ny_kernel_instr=461052577
post_repair_ny_kernel_cycles=250151042
post_repair_ratio_kernel_instr=0.08
post_repair_ratio_kernel_cycles=0.03

selected_next_front=kilo_meso_substring_concat_array_set
selected_next_owner_family=array_text_slot_insert_store_boundary
next_task=SUBSTRING-CONCAT-ARRAY-SET-ARRAY-TEXT-STORE-OWNER-INVENTORY-001
summary=ok
```

## Purpose

Fix the invalid C pair before continuing owner selection.

The previous C benchmark used:

```c
char lines[128][96];
```

but the loop performs 320000 appends across 128 rows. Each row receives about
2500 appends, and each append writes two bytes plus the terminating NUL. The
maximum row length is:

```text
seed_len + append_bytes * ceil(ops / rows) + nul
16 + 2 * 2500 + 1 = 5017
```

The old C pair wrote out of bounds and must not be used as owner evidence.

## Change

`benchmarks/c/bench_kilo_meso_indexof_append_array_set.c` now derives the
capacity from the benchmark constants:

```c
enum {
  rows = 128,
  ops = 320000,
  seed_len = 16,
  append_bytes = 2,
  max_appends_per_row = (ops + rows - 1) / rows,
  line_cap = seed_len + append_bytes * max_appends_per_row + 1
};

char lines[rows][line_cap];
```

This changes only the C benchmark contract. It does not change Hako source,
compiler lowering, or product runtime behavior.

## Measurement

Command:

```bash
bash tools/perf/bench_micro_c_vs_aot_lanes.sh \
  kilo_meso_indexof_append_array_set 1 3 100
```

Observed after repair:

```text
c_kernel_instr=36362861
c_kernel_cycles=6329251
ny_kernel_instr=461052577
ny_kernel_cycles=250151042
ratio_kernel_instr=0.08
ratio_kernel_cycles=0.03
```

The repaired benchmark still shows a large Hako gap, but it is a mixed growing
text front. The smaller next owner remains `kilo_meso_substring_concat_array_set`.

## Next

```text
SUBSTRING-CONCAT-ARRAY-SET-ARRAY-TEXT-STORE-OWNER-INVENTORY-001
```

Classify the smaller valid front before returning to the growing
`indexof_append_array_set` front.
