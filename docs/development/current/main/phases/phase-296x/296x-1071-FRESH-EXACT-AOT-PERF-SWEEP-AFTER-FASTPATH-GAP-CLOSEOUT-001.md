Status: Done
Date: 2026-06-17
Scope: fresh exact-AOT perf sweep after user-box fastpath gap closeout
Previous:
  - docs/development/current/main/phases/phase-296x/296x-1070-FRESH-COMPILER-OWNER-SELECTION-007.md
Artifact:
  - target/fresh-exact-aot-perf-sweep-1071/lanes.log

# FRESH-EXACT-AOT-PERF-SWEEP-AFTER-FASTPATH-GAP-CLOSEOUT-001

## Purpose

Re-run the exact-AOT candidate sweep after the user-box method fastpath gap was
closed as thin-entry covered rather than a missing `LocalFastPathFact`
producer.

This row is measurement and owner selection only. It does not implement a new
backend fastpath.

## Preflight

The first sweep attempt failed the perf harness contract:

```text
aot_status=skip
reason=default_release_out_of_sync
stage=contract
hint=rerun bash tools/perf/build_perf_release.sh
```

The release artifacts were rebuilt:

```bash
bash tools/perf/build_perf_release.sh
```

Then the candidate sweep was re-run:

```bash
for key in \
  kilo_meso_substring_concat_array_set \
  kilo_meso_substring_concat_array_set_loopcarry \
  kilo_meso_substring_concat_len \
  kilo_meso_indexof_append_array_set \
  kilo_micro_len_substring_views \
  kilo_micro_substring_concat \
  kilo_micro_concat_hh_len \
  kilo_micro_array_string_store \
  kilo_micro_indexof_line \
  kilo_micro_substring_only \
  kilo_micro_substring_views_only \
  kilo_micro_concat_birth \
  kilo_leaf_array_string_indexof_const \
  kilo_leaf_array_rmw_add1 \
  kilo_leaf_map_getset_has \
  kilo_leaf_map_get_dynamic_covered_i64
do
  bash tools/perf/bench_micro_c_vs_aot_lanes.sh "$key" 1 3 100 || true
done | tee target/fresh-exact-aot-perf-sweep-1071/lanes.log
```

## Sweep Summary

```text
kilo_meso_substring_concat_array_set:
  ratio_kernel_instr=1.59
  ratio_kernel_cycles=0.93
  status=near_parity_cycles_not_fresh_owner

kilo_meso_substring_concat_array_set_loopcarry:
  ratio_kernel_instr=1.01
  ratio_kernel_cycles=1.06
  status=equivalence_guard

kilo_meso_substring_concat_len:
  ratio_kernel_instr=290.84
  ratio_kernel_cycles=41.04
  status=closed_form_exact_seed

kilo_meso_indexof_append_array_set:
  ratio_kernel_instr=37.16
  ratio_kernel_cycles=35.29
  status=hako_faster_after_recent_materialization_work

kilo_micro_len_substring_views:
  ratio_kernel_instr=239.71
  ratio_kernel_cycles=41.15
  status=closed_keeper

kilo_micro_substring_concat:
  ratio_kernel_instr=484.14
  ratio_kernel_cycles=67.43
  status=closed_form_exact_seed

kilo_micro_concat_hh_len:
  ratio_kernel_instr=596.41
  ratio_kernel_cycles=104.45
  status=closed_tiny_exact_kernel

kilo_micro_array_string_store:
  ratio_kernel_instr=12.67
  ratio_kernel_cycles=43.66
  status=hako_faster

kilo_micro_indexof_line:
  ratio_kernel_instr=9.12
  ratio_kernel_cycles=4.72
  status=hako_faster

kilo_micro_substring_only:
  ratio_kernel_instr=272.77
  ratio_kernel_cycles=43.81
  status=closed_tiny_exact_kernel

kilo_micro_substring_views_only:
  ratio_kernel_instr=0.42
  ratio_kernel_cycles=0.44
  c_kernel_instr=1301
  ny_kernel_instr=3100
  status=tiny_floor_rejected

kilo_micro_concat_birth:
  ratio_kernel_instr=17656.04
  ratio_kernel_cycles=2097.29
  status=closed_tiny_exact_kernel

kilo_leaf_array_string_indexof_const:
  ratio_kernel_instr=264.09
  ratio_kernel_cycles=202.03
  status=hako_faster_closed_leaf

kilo_leaf_array_rmw_add1:
  ratio_kernel_instr=1.00
  ratio_kernel_cycles=1.01
  status=near_parity_equivalence_guard

kilo_leaf_map_getset_has:
  ratio_kernel_instr=1064.65
  ratio_kernel_cycles=231.45
  status=hako_faster_closed_map_front

kilo_leaf_map_get_dynamic_covered_i64:
  ratio_kernel_instr=3.05
  ratio_kernel_cycles=1.69
  status=hako_faster_with_real_c_pair
```

## Decision

No fresh exact-AOT compiler optimization owner is selected from this sweep.

```text
fresh_compiler_optimization_owner_selected=0
selected_perf_owner=none
selected_perf_owner_confidence=none
```

The only Hako-slower successful front is tiny-floor residue:

```text
tiny_floor_rejected_front=kilo_micro_substring_views_only
tiny_floor_rejected_reason=c_kernel_instr_1301_too_small_for_owner_selection
```

The nearest non-tiny mixed result is not a strong compiler owner:

```text
near_parity_front=kilo_meso_substring_concat_array_set
ratio_kernel_instr=1.59
ratio_kernel_cycles=0.93
reason=Hako has fewer instructions but slightly worse cycles; no single owner
```

## Next Design Point

The fastpath lane has reached a selection boundary. Continuing with more
fastpath work requires a new owner source:

```text
next_task=NEXT-LANE-SELECTION-AFTER-FASTPATH-SWEEP-001
```

Candidate choices:

```text
A. Pause exact-AOT fastpath optimization and return to compiler construction
   work.

B. Open a new perf-owner discovery row with a wider benchmark set.

C. Investigate the near-parity substring_concat_array_set cycle gap only if a
   concrete hot owner appears in perf annotate/asm, not from ratio alone.
```

## Contract

```text
output_contract=fresh-exact-aot-perf-sweep-after-fastpath-gap-closeout-v0

successful_front_count=16
aot_failed_front_count=0
fresh_compiler_optimization_owner_selected=0
selected_perf_owner=none
selected_perf_owner_confidence=none

user_box_fastpath_gap_closed=1
local_fastpath_fact_producer_gap_reopened=0

tiny_floor_rejected_front=kilo_micro_substring_views_only
near_parity_front=kilo_meso_substring_concat_array_set

implementation_started=0
next_task=NEXT-LANE-SELECTION-AFTER-FASTPATH-SWEEP-001
summary=ok
```

## Stop Lines

```text
do not select a new compiler owner from tiny kernel windows
do not reopen user-box LocalFastPathFact producer without uncovered_count>0
do not claim performance win from report cleanup
do not optimize substring_concat_array_set from ratio alone
do not start implementation without a fresh owner
```
