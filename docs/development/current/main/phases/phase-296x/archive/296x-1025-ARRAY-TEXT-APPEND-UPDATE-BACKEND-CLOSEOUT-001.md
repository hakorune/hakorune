Status: Done
Date: 2026-06-17
Scope: closeout for the append/update observer len-sum backend fast path.
Related:
  - docs/development/current/main/CURRENT_STATE.toml
  - docs/development/current/main/phases/phase-296x/296x-1022-ARRAY-TEXT-APPEND-UPDATE-BACKEND-LOWERING-IMPLEMENTATION-001.md
  - docs/development/current/main/phases/phase-296x/296x-1023-ARRAY-TEXT-APPEND-UPDATE-BACKEND-LOWERING-VALIDATION-001.md
  - docs/development/current/main/phases/phase-296x/296x-1024-ARRAY-TEXT-APPEND-UPDATE-BACKEND-MEASUREMENT-001.md

# ARRAY-TEXT-APPEND-UPDATE-BACKEND-CLOSEOUT-001

## Purpose

Close the append/update observer len-sum backend row after validation and
measurement.

The selected compiler/backend owner is complete for the current target front:
`ny_main` reaches the MIR-owned len-sum helper route, and the old per-iteration
materialization route is absent from the `ny_main` objdump snippet.

## Decision

```text
backend_fast_path_keeper=1
target_front=kilo_meso_indexof_append_array_set
selected_helper=nyash.array.string_indexof_suffix_store_len_sum_region_hiisisi
old_materialization_route_removed_from_ny_main=1
store_count_helper_reused=0
raw_mir_window_rescan_allowed=0
benchmark_name_branch=0
helper_name_inference=0
product_default_changed=0
```

Measurement summary:

```text
ratio_kernel_instr=37.16
ratio_kernel_cycles=35.66
ratio_total_instr=1.47
ratio_total_cycles=1.39
ratio_total_ms=0.80
```

Interpretation:

```text
kernel_backend_route_win=1
product_winner_claim=0
total_wall_winner_claim=0
remaining_hot_owner=runtime_helper_body
backend_reachability_owner_closed=1
```

## Stop Lines

```text
do not continue backend route work on this front without fresh evidence
do not optimize helper internals in this closeout row
do not claim product-wide speedup from total wall time
do not reopen store-only helper reuse
```

## Next

```text
FRESH-COMPILER-OWNER-SELECTION-003
```

Return to owner-first selection. If the runtime helper body is selected later,
it must be a separate runtime/helper owner row, not a continuation of this
backend reachability row.
