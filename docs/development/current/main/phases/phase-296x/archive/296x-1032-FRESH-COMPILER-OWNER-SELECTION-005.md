Status: Done
Date: 2026-06-17
Scope: fresh exact-AOT compiler owner selection after
`ARRAY-RMW-ADD1-EXACT-SEED-METADATA-CONTRACT-001`.
Related:
  - docs/development/current/main/CURRENT_STATE.toml
  - docs/development/current/main/phases/phase-296x/296x-1031-ARRAY-RMW-ADD1-EXACT-SEED-METADATA-CONTRACT-001.md
Artifacts:
  - target/fresh-compiler-owner-selection-1032/lanes.log
  - target/fresh-compiler-owner-selection-1032/map_dynamic_covered_i64.cpair.log

# FRESH-COMPILER-OWNER-SELECTION-005

## Purpose

Select the next compiler fastpath owner after the array RMW add1 exact seed
coverage repair restored `kilo_leaf_array_rmw_add1` to `aot_status=ok`.

This row is selection and measurement hygiene only. It does not add a backend
fastpath.

## Sweep

The sweep re-ran the current exact-AOT candidate set:

```text
kilo_meso_substring_concat_array_set
kilo_meso_substring_concat_array_set_loopcarry
kilo_meso_substring_concat_len
kilo_meso_indexof_append_array_set
kilo_micro_len_substring_views
kilo_micro_substring_concat
kilo_micro_concat_hh_len
kilo_micro_array_string_store
kilo_micro_indexof_line
kilo_micro_substring_only
kilo_micro_substring_views_only
kilo_micro_concat_birth
kilo_leaf_array_string_indexof_const
kilo_leaf_array_rmw_add1
kilo_leaf_map_getset_has
kilo_leaf_map_get_dynamic_covered_i64
```

## Measurement Hygiene Fix

`kilo_leaf_map_get_dynamic_covered_i64` was still Hako-only in the lane runner,
so the sweep initially reported:

```text
c benchmark missing: benchmarks/c/bench_kilo_leaf_map_get_dynamic_covered_i64.c
```

This row adds a real C pair using a small open-address i64 map. The C pair
performs the same shape as the Hako front:

```text
preseed keys 0, 1, 2
loop key = i % 3
map lookup every iteration
sum values
final get(1)
```

It intentionally avoids the old volatile-compare placeholder pattern.

## Results

The repaired array RMW front is no longer blocked:

```text
kilo_leaf_array_rmw_add1:
  aot_status=ok
  ratio_kernel_instr=1.00
  ratio_kernel_cycles=1.00
```

The new real C pair for `kilo_leaf_map_get_dynamic_covered_i64` is comparison
ready and does not expose a Hako-slower compiler owner:

```text
kilo_leaf_map_get_dynamic_covered_i64:
  aot_status=ok
  c_kernel_instr=58001400
  c_kernel_cycles=10230335
  ny_kernel_instr=19018186
  ny_kernel_cycles=5937045
  ratio_kernel_instr=3.05
  ratio_kernel_cycles=1.72
```

The other successful fronts are already folded, recently closed, or Hako-smaller
on the kernel lane. No fresh Hako-slower exact-AOT compiler owner is selected
from this sweep.

## Decision

```text
output_contract=fresh-compiler-owner-selection-v5
fresh_compiler_optimization_owner_selected=0
selected_perf_owner=none
selected_perf_owner_confidence=none

array_rmw_add1_aot_status=ok
array_rmw_add1_kernel_ratio=1.00

map_dynamic_covered_i64_c_pair_added=1
map_dynamic_covered_i64_c_pair_real_hashmap=1
map_dynamic_covered_i64_volatile_compare_pair=0
map_dynamic_covered_i64_aot_status=ok
map_dynamic_covered_i64_ratio_kernel_instr=3.05
map_dynamic_covered_i64_ratio_kernel_cycles=1.72

implementation_started=0
next_task=FASTPATH-OPTIMIZATION-CHECKPOINT-001
summary=ok
```

## Stop Lines

```text
do not open a new compiler fastpath without a Hako-slower owner
do not use volatile-compare C placeholders for map lookup winner claims
do not treat missing C pairs as performance evidence
do not change Hako source to chase a C ratio
```
