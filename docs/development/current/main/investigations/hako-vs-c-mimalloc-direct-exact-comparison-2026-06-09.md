Status: Active
Date: 2026-06-09
Scope: one-shot C vs .hako mimalloc direct-exact comparison sweep.
Related:
  - tools/perf/bench_compare_c_vs_hako.sh
  - tools/allocator/hako_mimalloc_direct_exact_pair.sh
  - docs/development/current/main/investigations/docs-pointer-inventory-2026-06-09.md

# Hako vs C Mimalloc Direct-Exact Comparison

This note records the one-shot sweep that compared the current `.hako`
direct-exact front against C mimalloc across the benchmark pairs that exist in
`benchmarks/bench_*.hako` and `benchmarks/c/bench_*.c`.

## Command

```bash
PERF_SKIP_VM_PREFLIGHT=1 bash tools/perf/bench_compare_c_vs_hako.sh <bench_key> 1 3
```

The sweep ran for 32 matching benchmark pairs.

## Summary

```text
bench_count=32
hako_slower_than_c=32
median_slowdown=99.0x
mean_slowdown=394.2x
```

The direct-exact representative small-block object-lifecycle pair remains
slower than C, but much closer than the worst micro-benchmarks:

```text
representative-object-lifecycle-small-block-v0
hako_body_elapsed_ns=7000000
c_body_elapsed_ns=3386976
ratio_c_over_hako=2.067
summary=ok
```

## Worst Slowdowns

```text
3628.7x  kilo_micro_userbox_counter_step_chain
2315.0x  kilo_micro_userbox_point_sum
1939.0x  kilo_micro_userbox_counter_step
1314.7x  kilo_micro_userbox_point_add
 433.0x  kilo_leaf_map_getset_has
 369.7x  kilo_leaf_array_rmw_add1
 363.2x  chip8_kernel_small
 343.7x  kilo_leaf_map_get_missing
 302.2x  kilo_micro_array_getset
 221.3x  method_call_only
```

## Least-Bad Workloads

```text
0.40  kilo_micro_userbox_flag_toggle
0.27  kilo_micro_substring_concat
0.27  kilo_meso_substring_concat_array_set_loopcarry
0.25  method_call_only_small
0.21  box_create_destroy_small
```

## Current Observation

The current userbox counter-heavy leader is no longer the same shape as the
original sweep. `kilo_micro_userbox_counter_step_chain` now lowers through the
selected exact-slot bridge and the C-ABI exact-seed emitter returns a closed
form result, so it is no longer the clearest residual owner for codegen work.

The first exact front with a still-measurable kernel-only body is now
`kilo_micro_userbox_point_add`:

```text
point_add kernel-only:
  ratio_kernel_cycles=1.00
  ratio_kernel_ms=1.00
```

The remaining gap on `counter_step_chain` is dominated by startup / loader
noise, so the next design choice is whether to invest in startup/loader
reduction or move the userbox optimization lane to `point_add`.

## Next Optimization Focus

The sweep suggests the next exact front should come from the userbox / counter
heavy family first, then the map / array leaf family:

```text
1. kilo_micro_userbox_counter_step_chain
2. kilo_micro_userbox_point_sum
3. kilo_micro_userbox_counter_step
4. kilo_micro_userbox_point_add
5. kilo_leaf_map_getset_has
```

These are the largest current gaps and are the most likely to produce a
meaningful exact-front win before widening to broader allocator surfaces.
