# 296x-963 MIMALLOC-ARRAY-TEXT-LOOP-SESSION-CLOSEOUT-001

Status: Landed
Date: 2026-06-16

## Purpose

Close the `kilo_leaf_array_string_len` array text loop-session lane after the
296x-959 backend lowering and 296x-962 helper-local length table keeper.

This row prevents further same-front shaving without fresh evidence.

## Closed Chain

```text
296x-929:
  selected_front=kilo_leaf_array_string_len
  selected_owner_family=array_text_slot_len_loop_local_session_boundary
  selected_ny_aot_instr=92925688
  selected_ny_aot_cycles=32626920
  selected_ratio_cycles=0.10

296x-959:
  backend lowering emits nyash.array.string_len_sum_region_hiii once
  loop body skipped through MIR-owned loop_body metadata

296x-960:
  backend lowering keeper measured
  ny_aot_instr=20925175
  ny_aot_cycles=4585694
  ratio_cycles=0.70

296x-961:
  remaining owner inventoried as repeated Boxed-slot text projection
  selected_seam=region_helper_local_length_table

296x-962:
  helper-local length table implemented
  ny_aot_instr=5928013
  ny_aot_cycles=1395792
  ratio_cycles=2.29
```

## Closeout Result

```text
output_contract=hako-mimalloc-array-text-loop-session-closeout-v0
target_front=kilo_leaf_array_string_len

lane_closed=1
selected_owner_family_closed=array_text_slot_len_loop_local_session_boundary
backend_loop_session_lowering_keeper=1
helper_local_length_table_keeper=1

baseline_ny_aot_instr=92925688
final_ny_aot_instr=5928013
total_ny_aot_instr_reduction_pct=93.62

baseline_ny_aot_cycles=32626920
final_ny_aot_cycles=1395792
total_ny_aot_cycles_reduction_pct=95.72

baseline_ratio_cycles=0.10
final_ratio_cycles=2.29
exact_front_counter_winner=1
wall_time_winner_claim=0
product_default_speedup_claim=0

product_array_storage_changed=0
array_push_route_changed=0
borrowed_handle_representation_changed=0
mirbuilder_object_management_enabled=0
benchmark_name_branch_added=0
helper_name_inference_added=0

selected_next=MIMALLOC-FRESH-FRONT-SELECTION-AFTER-ARRAY-TEXT-CLOSEOUT-001
summary=ok
```

## Decision

Stop optimizing this exact front for now.

Reason:

```text
the selected owner is closed
the front is now a counter winner against its C pair
remaining direct microasm samples are too sparse/startup-sensitive for another
same-front implementation without fresh owner selection
```

The next row must select a fresh front / owner from current evidence instead of
continuing to optimize the already-won array text loop-session path.

## Stop Line

```text
do not add another array text loop-session implementation row
do not generalize the length-table helper to mutating regions without a new card
do not change ArrayBox text storage policy from this lane
do not change BorrowedHandleBox representation from this lane
do not claim product default speedup from an exact-front counter keeper
```

## Proof Bundle

```bash
cargo test -p nyash_kernel array_string_len_sum_region -- --nocapture
cargo fmt --check
cargo check --bin hakorune
bash tools/perf/build_perf_release.sh
bash tools/perf/bench_micro_c_vs_aot_stat.sh kilo_leaf_array_string_len 1 3
bash tools/checks/current_state_pointer_guard.sh
git diff --check
```
