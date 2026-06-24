Status: Done
Date: 2026-06-17
Scope: repair the exact-AOT backend consumer contract for
`kilo_leaf_array_rmw_add1`.
Related:
  - docs/development/current/main/CURRENT_STATE.toml
  - docs/development/current/main/phases/phase-296x/296x-1030-FRESH-COMPILER-OWNER-SELECTION-004.md
Artifacts:
  - target/array-rmw-add1-aot-failure-1031/direct.mir.json
  - target/array-rmw-add1-aot-failure-1031/direct_build.fixed.log
  - target/array-rmw-add1-aot-failure-1031/direct_run.fixed.log
  - target/array-rmw-add1-aot-failure-1031/lanes.fixed.log
  - target/array-rmw-add1-aot-failure-1031/aot_asm.fixed.log

# ARRAY-RMW-ADD1-EXACT-SEED-METADATA-CONTRACT-001

## Purpose

Repair the C ABI exact seed consumer for `kilo_leaf_array_rmw_add1`.

The previous consumer rejected the current source-direct MIR because it pinned
raw instruction positions from an older synthetic fixture. The direct MIR still
publishes the correct exact seed proof and selected RMW block, but current
metadata reports the RMW get/set positions as `7/12` instead of the stale
consumer expectation `8/13`.

This row is a coverage-contract repair only. It does not optimize array runtime
helpers and does not add a fallback route.

## Root Cause

`FRESH-COMPILER-OWNER-SELECTION-004` found that direct MIR emission succeeds and
publishes:

```text
array_rmw_add1_leaf_seed_route.proof=kilo_leaf_array_rmw_add1_7block
array_rmw_add1_leaf_seed_route.rmw_proof=array_get_add1_set_same_slot
array_rmw_add1_leaf_seed_route.selected_rmw_block=23
array_rmw_add1_leaf_seed_route.selected_rmw_instruction_index=7
array_rmw_add1_leaf_seed_route.selected_rmw_set_instruction_index=12
```

The backend consumer rejected that route as a metadata contract mismatch because
it required the raw instruction indices to be exactly `8/13`.

Raw instruction positions are not the semantic truth for this consumer. The
route proof, RMW proof, size/operation counts, selected RMW block, and set-after-
get ordering are the durable contract.

## Change

The C ABI consumer now keeps the existing proof/shape checks and replaces the
stale raw-index equality with:

```text
selected_rmw_instruction_index > 0
selected_rmw_set_instruction_index > selected_rmw_instruction_index
```

The direct emit smoke updates the source-direct MIR expectation to `7/12`.
Rust-side synthetic fixtures remain unchanged because they still exercise their
own synthetic `8/13` shape.

## Evidence

The fixed direct MIR build now consumes the exact seed route:

```text
[llvm-route/trace] stage=exact_seed_backend_route result=hit reason=mir_route_metadata extra=array_rmw_add1_leaf
[llvm-route/trace] stage=array_rmw_add1_leaf result=emit reason=exact_match extra=kilo_leaf_array_rmw_add1 mir_route_plan
```

The fixed executable returns the expected byte result:

```text
Result: 145
exit=145
```

The source direct-emission contract smoke passes:

```text
[PASS] phase137x_direct_emit_array_rmw_add1_leaf_contract
```

The perf lane is no longer blocked:

```text
aot_status=ok
ny_kernel_instr=12003143
ny_kernel_cycles=2011711
ratio_kernel_instr=1.00
ratio_kernel_cycles=1.00
ny_kernel_ipc=5.97
```

Micro-ASM confirms the route reaches `ny_main` as a direct loop with no runtime
array helper in the hot loop.

## Decision

```text
output_contract=array-rmw-add1-exact-seed-metadata-contract-v0
target_front=kilo_leaf_array_rmw_add1
direct_emit_succeeds=1
exact_seed_route_consumed=1
stale_instruction_index_pin_removed=1
c_consumer_expected_indices_8_13=0
c_consumer_requires_block_23=1
c_consumer_requires_set_after_get=1
aot_status_after=ok
kernel_ratio_after=1.00
fallback_route_added=0
runtime_array_helper_changed=0
benchmark_name_branch_added=0
winner_claim=0
next_task=FRESH-COMPILER-OWNER-SELECTION-005
summary=ok
```

## Stop Lines

```text
do not treat raw instruction index equality as the durable seed contract
do not add a fallback route for this exact seed
do not optimize array runtime helpers from this evidence
do not change the Rust synthetic fixture to match source-direct MIR positions
do not claim a new performance winner from a coverage repair
```
