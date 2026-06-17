Status: Done
Date: 2026-06-17
Scope: fresh exact-AOT compiler owner selection after
`STRING-SUBSTRING-RESULT-LEN-FACT-001`.
Related:
  - docs/development/current/main/CURRENT_STATE.toml
  - docs/development/current/main/phases/phase-296x/296x-1029-STRING-SUBSTRING-RESULT-LEN-FACT-001.md
Artifacts:
  - target/fresh-compiler-owner-selection-1030/lanes.log
  - target/array-rmw-add1-aot-failure-1031/direct.mir.json
  - target/array-rmw-add1-aot-failure-1031/direct_build.log
  - target/array-rmw-add1-aot-failure-1031/helper_emit.log

# FRESH-COMPILER-OWNER-SELECTION-004

## Purpose

Select the next compiler optimization owner after
`kilo_micro_len_substring_views` became a keeper and no longer exposes
`len_fast_h` as a hot body owner.

This row is selection-only. AOT coverage failures are not performance owners.

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
```

## Findings

`kilo_micro_len_substring_views` is now closed for the selected owner:

```text
aot_status=ok
ny_kernel_instr=6266
ny_kernel_cycles=7915
ratio_kernel_instr=239.60
ratio_kernel_cycles=38.23
```

Most successful fronts are already Hako-smaller, tiny exact kernels, or recently
closed route reachability wins. They do not expose a fresh Hako-slower compiler
owner in this sweep.

The only blocked front is still `kilo_leaf_array_rmw_add1`:

```text
aot_status=skip
reason=emit_helper_retry_failed
stage=emit_retry
```

Direct MIR emission succeeds and publishes the expected route metadata:

```text
array_rmw_add1_leaf_seed_route.proof=kilo_leaf_array_rmw_add1_7block
array_rmw_add1_leaf_seed_route.rmw_proof=array_get_add1_set_same_slot
array_rmw_add1_leaf_seed_route.selected_rmw_block=23
array_rmw_add1_leaf_seed_route.selected_rmw_instruction_index=7
array_rmw_add1_leaf_seed_route.selected_rmw_set_instruction_index=12
```

The C ABI exact seed consumer rejects it as a metadata contract mismatch because
the consumer still pins the stale raw instruction positions `8/13`.

## Decision

```text
output_contract=fresh-compiler-owner-selection-v4
successful_front_count=14
aot_failed_front_count=1
selected_perf_owner=none
selected_perf_owner_confidence=none
fresh_compiler_optimization_owner_selected=0

blocked_front=kilo_leaf_array_rmw_add1
blocked_front_reason=emit_helper_retry_failed
blocked_front_is_perf_owner=0
blocked_front_is_compiler_coverage_blocker=1
blocked_front_direct_emit_succeeds=1
blocked_front_consumer_failure=array_rmw_add1_leaf_metadata_contract_mismatch
stale_consumer_instruction_index_contract=1

next_task=ARRAY-RMW-ADD1-EXACT-SEED-METADATA-CONTRACT-001
summary=ok
```

## Stop Lines

```text
do not treat emit_helper_retry_failed as a perf owner
do not optimize array runtime helpers from this evidence
do not add a fallback route for the exact seed
do not pin backend consumer validity to stale raw instruction indices
```
