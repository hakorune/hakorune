Status: Done
Date: 2026-06-17
Scope: fresh exact-AOT compiler owner selection after the append/update
observer len-sum backend closeout.
Related:
  - docs/development/current/main/CURRENT_STATE.toml
  - docs/development/current/main/phases/phase-296x/296x-1025-ARRAY-TEXT-APPEND-UPDATE-BACKEND-CLOSEOUT-001.md
  - docs/development/current/main/design/perf-owner-first-optimization-ssot.md
Artifacts:
  - target/fresh-compiler-owner-selection-1026/lanes.log
  - target/len-substring-views-aot-failure-1027/aot_asm_direct_only.log

# FRESH-COMPILER-OWNER-SELECTION-003

## Purpose

Select the next compiler optimization owner after the
`kilo_meso_indexof_append_array_set` backend reachability row was closed.

This row is selection-only. It must not turn a build/emit failure into a perf
owner, and it must not reopen closed target fronts without fresh hot-path
evidence.

## Sweep

Command:

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
  kilo_micro_concat_birth
do
  bash tools/perf/bench_micro_c_vs_aot_lanes.sh "$key" 1 3 100 || true
done
```

Observed front summary:

```text
kilo_meso_substring_concat_array_set:
  ratio_kernel_instr=1.59
  ratio_kernel_cycles=0.90
  status=already-covered / no fresh owner

kilo_meso_substring_concat_array_set_loopcarry:
  ratio_kernel_instr=1.01
  ratio_kernel_cycles=1.08
  status=near-parity / no fresh owner

kilo_meso_substring_concat_len:
  ratio_kernel_instr=290.74
  ratio_kernel_cycles=41.89
  status=closed-form exact seed / no fresh owner

kilo_meso_indexof_append_array_set:
  ratio_kernel_instr=37.22
  ratio_kernel_cycles=36.28
  status=just-closed backend keeper / no fresh owner

kilo_micro_len_substring_views:
  aot_status=skip
  direct_probe_reason=pure_first_unsupported_shape
  status=coverage blocker candidate, not perf owner

kilo_micro_substring_concat:
  ratio_kernel_instr=484.29
  ratio_kernel_cycles=70.94
  status=closed-form exact seed / no fresh owner

kilo_micro_concat_hh_len:
  ratio_kernel_instr=596.32
  ratio_kernel_cycles=113.59
  status=tiny exact kernel / no fresh owner

kilo_micro_array_string_store:
  ratio_kernel_instr=12.65
  ratio_kernel_cycles=42.09
  status=Hako kernel already faster / no fresh owner

kilo_micro_indexof_line:
  ratio_kernel_instr=9.12
  ratio_kernel_cycles=4.78
  status=Hako kernel already faster / no fresh owner

kilo_micro_substring_only:
  ratio_kernel_instr=272.82
  ratio_kernel_cycles=45.44
  status=closed/tiny exact kernel / no fresh owner

kilo_micro_substring_views_only:
  ratio_kernel_instr=0.42
  ratio_kernel_cycles=0.46
  c_kernel_instr=1302
  ny_kernel_instr=3101
  status=tiny-floor residue / rejected as owner

kilo_micro_concat_birth:
  ratio_kernel_instr=17656.04
  ratio_kernel_cycles=2167.95
  status=closed/tiny exact kernel / no fresh owner
```

## Decision

```text
output_contract=fresh-compiler-owner-selection-v3
successful_front_count=11
aot_failed_front_count=1
selected_perf_owner=none
selected_perf_owner_confidence=none
fresh_compiler_optimization_owner_selected=0

blocked_front=kilo_micro_len_substring_views
blocked_front_direct_probe_reason=pure_first_unsupported_shape
blocked_front_is_perf_owner=0
blocked_front_is_compiler_coverage_blocker=1

tiny_floor_rejected_front=kilo_micro_substring_views_only
tiny_floor_rejected_reason=kernel_window_too_small

closed_fronts_reopened=0
implementation_started=0
summary=ok
```

The next row is a coverage inventory, not a performance implementation row:

```text
next_task=LEN-SUBSTRING-VIEWS-AOT-FAILURE-INVENTORY-001
```

## Stop Lines

```text
do not treat pure_first_unsupported_shape as a perf owner
do not optimize closed-form exact seed fronts from ratio alone
do not reopen kilo_meso_indexof_append_array_set without fresh hot evidence
do not claim a winner from tiny kernel windows
do not add benchmark/source/helper-name branches
```

