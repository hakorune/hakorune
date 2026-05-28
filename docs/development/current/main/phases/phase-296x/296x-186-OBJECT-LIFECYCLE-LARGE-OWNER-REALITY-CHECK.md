---
Status: Landed
Date: 2026-05-28
Scope: stop copy-only optimization and reclassify the large Hako/C object-lifecycle owner.
Blocker: OBJECT-LIFECYCLE-LARGE-OWNER-REALITY-CHECK-296X-001
Related:
  - docs/development/current/main/phases/phase-296x/296x-185-RECEIVER-PIN-CHAIN-NARROWING-KEEPER.md
  - docs/development/current/main/design/perf-owner-first-optimization-ssot.md
---

# 296x-186 Object Lifecycle Large Owner Reality Check

## Purpose

The row185 receiver/copy keeper reduced MIR copy pressure, but the Hako exact-EXE
body still measures around hundreds of milliseconds while the explicit C
mimalloc pair measures a few milliseconds. The next row must stop copy-only
keeper search and classify the large owner before another optimization.

## Current Evidence

One fresh scout pair after row185:

```text
hako_body_elapsed_ns=566000000
c_body_elapsed_ns=4187000
body_elapsed_ratio=135.180
hako_external_elapsed_ms=570
c_external_elapsed_ms=10
allocation_count=524288
free_count=524288
select_page_single_fast_path_count=524288
release_known_page_fast_path_count=524288
```

Current `objectLifecycleSmallAlloc/1` MIR shape:

```text
instruction_count=153
call_count=12
copy_count=61
phi_count=18
helper_call_count=6
receiver_copy_count=18
local_ssa_copy_count=20
dominant_callee_family=page_hotpath_helpers
dominant_copy_owner=local_ssa_copy_materialization
```

Dynamic owner estimate from the hot workload:

```text
objectLifecycleSmallAlloc/1 dynamic calls=524288
objectLifecycleReleaseBlock/2 dynamic calls=524288
estimated field_get+field_set dynamic ops ~= 32.2M
estimated MIR call dynamic ops ~= 16.8M
row185 copy reduction ~= 14.2M dynamic copies, but timing did not materially move
```

Direct perf top on the exact EXE confirms the large owner:

```text
23.86% nyash.object.field_set_hii
18.64% nyash.object.field_get_hii
17.19% nyash.object.field_get_u64_hii
16.45% nyash_kernel::plugin::array_runtime_facade::array_runtime_set_idx_i64
13.27% nyash.object.field_set_u64_hiu
 6.65% nyash_kernel::plugin::array_slot_store::array_slot_store_i64
 1.55% nyash_kernel::plugin::array_handle_cache::array_get_index_encoded_i64
 1.54% nyash_kernel::plugin::array_runtime_facade::array_runtime_get_idx
```

## Selection

```text
selected_owner_family=mir_field_access_runtime_helper_cost
selected_owner_subfamily=typed_object_field_get_set_hot_loop
secondary_owner=array_runtime_set_get_hot_loop
rejected_owner=local_ssa_copy_materialization
rejected_reason=copy count improved substantially without closing the body timing gap
confidence=high
```

## Next Diagnostic

The large owner is now selected. The next diagnostic should inspect the exact
field/array lowering boundary behind the hot symbols, then choose one narrow
keeper. Assembly is now useful only around the selected symbols, not as a broad
search surface.

```bash
tmp=/tmp/row186-body-pair
mkdir -p "$tmp"

bash tools/allocator/hako_exe_memory_runner.sh \
  --app apps/hako-alloc-mimalloc-comparison-in-process-object-lifecycle-small-block-proof/main.hako \
  --workload representative-object-lifecycle-small-block-v0 \
  --runtime-config empty \
  --operation-repeat 1 \
  --out "$tmp/hako.out"

bash tools/allocator/c_mimalloc_explicit_runner.sh \
  --out "$tmp/c.out" \
  --allow-ldconfig-discovery \
  --workload representative-object-lifecycle-small-block-v0 \
  --in-process-repeat 8192 \
  --operation-repeat 1

python3 tools/allocator/hako_mimalloc_object_lifecycle_body_timing_pair_adapter.py \
  --hako-report "$tmp/hako.out" \
  --c-report "$tmp/c.out" \
  --out "$tmp/pair.out"
```

Perf/asm gate used:

```text
perf top points at typed-object field helpers and ArrayBox runtime helpers.
Proceed to field-access/runtime lowering inspection.
```

## Non-Goals

```text
- Do not continue narrow copy cleanup until the large owner is classified.
- Do not implement generic CSE or generic copy coalescing.
- Do not inspect assembly without a selected hot symbol.
- Do not open provider activation, allocator replacement, hooks, global allocator,
  or winner claims.
```

## Acceptance

```text
large_owner_selected=1
selected_owner_family=mir_field_access_runtime_helper_cost
secondary_owner=array_runtime_set_get_hot_loop
next_diagnostic=field_array_runtime_lowering_boundary_probe
optimization_open=0
winner_claim=0
replacement_active=0
hook_installed=0
global_allocator=0
summary=ok
```
