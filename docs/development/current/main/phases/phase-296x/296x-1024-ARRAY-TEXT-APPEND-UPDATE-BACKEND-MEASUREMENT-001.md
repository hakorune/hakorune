Status: Done
Date: 2026-06-17
Scope: target-front measurement after append/update observer len-sum backend lowering.
Related:
  - docs/development/current/main/CURRENT_STATE.toml
  - docs/development/current/main/phases/phase-296x/296x-1023-ARRAY-TEXT-APPEND-UPDATE-BACKEND-LOWERING-VALIDATION-001.md
  - target/array-text-append-update-backend-measurement-1024/lanes.log
  - target/array-text-append-update-backend-measurement-1024/microasm.log

# ARRAY-TEXT-APPEND-UPDATE-BACKEND-MEASUREMENT-001

## Purpose

Measure the active target front after reachability is proven.

This row judges the target-front kernel route. It does not claim a product-wide
or total-wall Hako-vs-C win because the total wall lane still includes startup
and runner effects.

## Commands

```bash
mkdir -p target/array-text-append-update-backend-measurement-1024
bash tools/perf/bench_micro_c_vs_aot_lanes.sh \
  kilo_meso_indexof_append_array_set 1 3 100 \
  | tee target/array-text-append-update-backend-measurement-1024/lanes.log

KEEP_PERF_MICROASM_ARTIFACTS=1 PERF_MICROASM_RUNNER_MODE=direct \
  bash tools/perf/bench_micro_aot_asm.sh \
  kilo_meso_indexof_append_array_set ny_main 3 \
  | tee target/array-text-append-update-backend-measurement-1024/microasm.log
```

## Result

Lane measurement:

```text
c_total_instr=36488443
c_total_cycles=6492663
c_total_ms=4
c_kernel_instr=36362860
c_kernel_cycles=6056513
c_kernel_ms=1.180

ny_total_instr=24799849
ny_total_cycles=4674681
ny_total_ms=5
ny_startup_instr=470076
ny_startup_cycles=729175
ny_startup_ms=4
ny_kernel_instr=978450
ny_kernel_cycles=169851
ny_kernel_ms=2.500

ratio_total_instr=1.47
ratio_total_cycles=1.39
ratio_total_ms=0.80
ratio_kernel_instr=37.16
ratio_kernel_cycles=35.66
ratio_kernel_ms=0.47
aot_status=ok
```

Direct micro-ASM top owner:

```text
95.58% nyash_array_string_indexof_suffix_store_len_sum_region_hiisisi_alias
```

`ny_main` still has the intended compact route:

```text
call nyash.array.string_indexof_suffix_store_len_sum_region_hiisisi
call nyash.array.len_h
```

## Interpretation

```text
target_kernel_instruction_win=1
target_kernel_cycle_win=1
total_cycle_win=1
total_wall_win=0
product_winner_claim=0
next_hot_owner=runtime_helper_body
```

The selected compiler/backend fast path is a keeper for the target-front kernel:
the per-iteration get/indexOf/concat/set/length materialization route was
removed from `ny_main`, and kernel work collapsed to the new helper. Further
optimization is no longer a backend reachability problem for this front; the
remaining hot owner is the runtime helper body itself.

## Stop Lines

```text
do not claim product-wide speedup from total wall time
do not continue backend route work for this front without a fresh owner
do not split helper internals without a new owner card
```

## Next

```text
ARRAY-TEXT-APPEND-UPDATE-BACKEND-CLOSEOUT-001
```

Close the backend fast-path row and return to fresh owner selection or a
separate runtime-helper owner only if new evidence selects it.
