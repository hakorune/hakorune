Status: Done
Date: 2026-06-17
Scope: checkpoint the current exact-AOT compiler fastpath lane after fresh owner
selection found no new Hako-slower implementation owner.
Related:
  - docs/development/current/main/CURRENT_STATE.toml
  - docs/development/current/main/phases/phase-296x/296x-1032-FRESH-COMPILER-OWNER-SELECTION-005.md

# FASTPATH-OPTIMIZATION-CHECKPOINT-001

## Purpose

Close the current compiler fastpath burst at a stable boundary.

Recent rows repaired the exact seed coverage hole for `kilo_leaf_array_rmw_add1`
and fixed the measurement hygiene hole for `kilo_leaf_map_get_dynamic_covered_i64`.
After those repairs, the fresh exact-AOT sweep no longer exposes a high-
confidence Hako-slower compiler fastpath owner.

## Current State

Closed or no-action fronts:

```text
kilo_leaf_array_rmw_add1:
  status=coverage repaired
  aot_status=ok
  kernel_ratio=1.00
  next_compiler_owner=none

kilo_leaf_map_get_dynamic_covered_i64:
  status=measurement hygiene repaired
  c_pair=real_i64_hashmap
  ratio_kernel_instr=3.05
  ratio_kernel_cycles=1.72
  next_compiler_owner=none

kilo_micro_len_substring_views:
  status=substring result length fact closed
  len_fast_h_hot_owner=0

local_i64_map_entry_table:
  status=route reaches active C ABI backend
  winner_claim=target_front_reachability_only
```

The remaining exact fronts are either folded/tiny, Hako-smaller on the kernel
lane, or already recently closed by route reachability rows.

## Decision

```text
output_contract=fastpath-optimization-checkpoint-v0
compiler_fastpath_lane_checkpointed=1
fresh_compiler_owner_selected=0
new_backend_fastpath_allowed=0
measurement_hygiene_blocker_open=0
coverage_blocker_open=0

array_rmw_add1_aot_status=ok
map_dynamic_covered_i64_c_pair_available=1
map_dynamic_covered_i64_c_pair_real_hashmap=1

next_task=PERF-FRONT-SELECTION-006
summary=ok
```

## Stop Lines

```text
do not add another fastpath without fresh Hako-slower owner evidence
do not chase helper symbols from folded/tiny kernels
do not claim Hako-vs-C wins from Hako-only fronts
do not reopen startup optimization from this lane
do not move representation decisions into MIRBuilder
```

## Next

The next optimization action should be a fresh perf front selection, not a
drive-by backend consumer.

Candidate directions:

```text
1. choose a non-folded exact-AOT front with Hako-slower kernel evidence
2. choose a product-body front only if measurement boundaries are matched
3. otherwise pause optimization and return to compiler foundation tasks
```
