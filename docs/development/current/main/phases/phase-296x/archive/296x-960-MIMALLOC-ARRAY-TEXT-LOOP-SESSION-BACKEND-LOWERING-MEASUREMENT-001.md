# 296x-960 MIMALLOC-ARRAY-TEXT-LOOP-SESSION-BACKEND-LOWERING-MEASUREMENT-001

Status: Landed
Date: 2026-06-16

## Purpose

Measure the backend lowering from 296x-959 before making any further
implementation claim.

The measured front is:

```text
kilo_leaf_array_string_len
```

The accepted implementation shape is:

```text
ny_main:
  build 64 text slots
  call nyash.array.string_len_sum_region_hiii(handle, 600000, 64, 0)
  call nyash.array.len_h(handle)
  add
  ret
```

## Evidence

Paired microstat:

```bash
bash tools/perf/bench_micro_c_vs_aot_stat.sh kilo_leaf_array_string_len 1 3
```

Result:

```text
c_instr=14526802
c_cycles=3195352
c_ms=4
ny_aot_instr=20925175
ny_aot_cycles=4585694
ny_aot_ms=4
ratio_instr=0.69
ratio_cycles=0.70
ratio_ms=1.00
c_ipc=4.55
ny_aot_ipc=4.56
aot_status=ok
```

Previous selected-front baseline from 296x-929:

```text
selected_ny_aot_instr=92925688
selected_ny_aot_cycles=32626920
selected_ratio_instr=0.16
selected_ratio_cycles=0.10
selected_ratio_ms=0.50
```

The new paired measurement removes the previous loop-session boundary as the
dominant owner:

```text
ny_aot_instr_reduction_pct=77.48
ny_aot_cycles_reduction_pct=85.94
ratio_cycles_before=0.10
ratio_cycles_after=0.70
```

Micro-ASM confirmation:

```bash
KEEP_PERF_MICROASM_ARTIFACTS=1 PERF_MICROASM_RUNNER_MODE=direct \
  bash tools/perf/bench_micro_aot_asm.sh kilo_leaf_array_string_len ny_main 3
```

Result:

```text
event_count_approx=1211190
top_symbol=BorrowedHandleBox::as_str_fast
top_symbol_percent=95.24
runner=direct
```

The generated `ny_main` contains:

```text
call nyash.array.string_len_sum_region_hiii
call nyash.array.len_h
add helper_result, array_length
ret
```

The direct executable returns:

```text
Result: 5400064
exit=0
```

## Result

```text
output_contract=hako-mimalloc-array-text-loop-session-backend-lowering-measurement-v0
target_front=kilo_leaf_array_string_len

backend_lowering_keeper=1
winner_claim=0
paired_microstat_repeat=3

previous_ny_aot_instr=92925688
current_ny_aot_instr=20925175
ny_aot_instr_reduction_pct=77.48

previous_ny_aot_cycles=32626920
current_ny_aot_cycles=4585694
ny_aot_cycles_reduction_pct=85.94

previous_ratio_cycles=0.10
current_ratio_cycles=0.70
current_ratio_ms=1.00

loop_session_runtime_boundary_removed=1
ny_main_contains_region_helper_call=1
post_loop_array_length_preserved=1
result_correct=1

remaining_hot_owner=borrowed_handle_text_access
remaining_top_symbol=BorrowedHandleBox::as_str_fast
remaining_top_symbol_percent=95.24
selected_next=MIMALLOC-ARRAY-TEXT-LOOP-SESSION-BORROWED-TEXT-ACCESS-OWNER-INVENTORY-001
summary=ok
```

## Interpretation

The loop-session lowering is a keeper: it removes the repeated generic
`array.get -> string.length` loop from `ny_main` and cuts the selected front's
AOT cycles by roughly 86% versus the 296x-929 selected-front baseline.

The remaining gap is no longer the backend loop-session seam. The direct
micro-ASM profile is dominated by `BorrowedHandleBox::as_str_fast` inside the
new helper path. The next row must inventory that text-access boundary before
any implementation.

## Stop Line

```text
do not add another backend loop-session lowering row for this front
do not claim product default speedup
do not special-case benchmark names
do not infer the next optimization from helper symbol alone
do not change StringBox / BorrowedHandleBox representation without inventory
```

## Proof Bundle

```bash
bash tools/perf/bench_micro_c_vs_aot_stat.sh kilo_leaf_array_string_len 1 3
KEEP_PERF_MICROASM_ARTIFACTS=1 PERF_MICROASM_RUNNER_MODE=direct \
  bash tools/perf/bench_micro_aot_asm.sh kilo_leaf_array_string_len ny_main 3
/home/tomoaki/git/hakorune-selfhost/target/perf_ny_kilo_leaf_array_string_len.microasm.1483023.exe
bash tools/checks/current_state_pointer_guard.sh
git diff --check
```
