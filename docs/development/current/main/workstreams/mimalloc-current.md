---
Status: Active
Date: 2026-05-31
Scope: active mimalloc migration and optimization workstream.
Related:
  - docs/development/current/main/CURRENT_STATE.toml
  - docs/development/current/main/design/current-docs-update-policy-ssot.md
  - docs/development/current/main/phases/phase-296x/296x-415-MIMALLOC-SOURCE-LEVEL-OWNER-SELECTION.md
  - lang/src/hako_alloc/memory/object_lifecycle_facade_box.hako
---

# Mimalloc Current Workstream

## Goal

Continue mimalloc migration and optimization without reopening Array/helper
fast-path work unless current perf evidence selects it.

Current owner surface:

```text
object_lifecycle_facade
```

## Stop Line

- no new numbered row for inventory-only work
- no row-specific `.sh` guard
- no new Array / RuntimeDataBox / helper fast path without current mimalloc
  perf evidence and positive-net implementation path
- no provider activation
- no allocator replacement
- no hook installation
- no `#[global_allocator]`
- no winner claim

## Checklist

Each task is intended to be small enough for one focused pass. Do not create a
new row for these tasks; update this checklist or use a Ghost Task commit
message.

### Observation

- [x] MIM-001: source-shape inventory for
  `lang/src/hako_alloc/memory/object_lifecycle_facade_box.hako`
  - output: short table of facade methods, field/capsule/page interactions,
    and likely owner candidates
  - no code changes
- [x] MIM-002: smallest owner selection inside `object_lifecycle_facade`
  - output: one selected source-level candidate or explicit no-owner result
  - no fast-path reopen
- [x] MIM-003: perf evidence refresh before source edits
  - output: owner-family evidence and reject/keep reason
  - no source edit before evidence

### Candidate

- [x] MIM-004: implementation boundary selection
  - output: one file/function boundary, or park with reason
  - allowed scope: `.hako` mimalloc source/state shape only
- [x] MIM-005: narrow implementation
  - output: code change only within the selected boundary
  - no Array / RuntimeDataBox / helper fast-path work
- [x] MIM-006: smoke and quick gate
  - output: existing lane guard / dev gate result
  - no new row-specific `.sh`

### Decision

- [x] MIM-007: keeper / nonkeeper decision only if it affects future work
  - output: Workstream Decision Log entry or durable row only when required by
    policy
- [x] MIM-008: SSOT direct edit only if design truth changes
  - output: owning `design/*.md` change with reason in commit message
- [x] MIM-009: cleanup / Ghost Task commit
  - output: commit message records small refactors, guard wording, or pointer
    fixes; no `CURRENT_STATE.toml` progress log
- [x] MIM-010: page selection delegation cleanup
  - output: `objectLifecycleSmallAlloc` delegates selection to
    `queue.selectPage()` and keeps the owner surface inside the existing
    page-queue seam
  - no fast-path reopen
- [x] MIM-011: selected page acquire route cleanup
  - output: `objectLifecycleSmallAlloc` calls `page.acquireFreshSmall(size)`
    after `queue.selectPage()` has selected an available page
  - no new page helper, Array lane, RuntimeDataBox lane, or direct-path reopen
- [x] MIM-012: alloc result reset-attempt capsule cleanup
  - output: `HakoAllocObjectLifecycleAllocResult.resetAttempt()` owns the
    reset-plus-attempt state transition used by `objectLifecycleSmallAlloc`
  - no new helper lane, Array lane, RuntimeDataBox lane, or direct-path reopen
- [x] MIM-013: defer alloc result block publication until acquire success
  - output: `objectLifecycleSmallAlloc` keeps the failure path on the
    `resetAttempt()` sentinel and publishes `last_block_id` only after a
    successful block acquisition
  - no new helper lane, Array lane, RuntimeDataBox lane, or direct-path reopen

## Decision Log

- 2026-05-31: Rows 388-413 are historical DirectArray / RuntimeDataBox /
  helper-cache closeout evidence. Row414 returned the lane to mimalloc
  source-level work. Row415 keeps `object_lifecycle_facade` as the active owner
  surface. Continue inside this workstream instead of opening inventory-only
  rows.
- 2026-05-31: MIM-001 source-shape inventory completed. The next owner should
  be selected from `objectLifecycleSmallAlloc`, cached release, or realloc-grow
  source shape. Observer/result readback methods stay out of the first source
  optimization candidate.
- 2026-05-31: MIM-003..MIM-007 completed as a narrow source-shape cleanup, not
  a perf keeper. `objectLifecycleSmallAlloc` now binds `alloc_result` before
  reset and calls `alloc_result.reset()` directly. This removes the remaining
  facade result helper call from the small-alloc helper-family probe, but does
  not materially improve the exact-EXE timing or MIR copy owner. Do not open a
  durable row from this cleanup.
- 2026-05-31: MIM-009 closed as a Ghost Task commit. Keep the source-shape
  cleanup recorded here and avoid spawning a new row or dedicated guard for it.
- 2026-05-31: MIM-010 closed as a page-queue delegation cleanup. The small-alloc
  entry now hands page selection back to `queue.selectPage()` instead of
  branching on page count in the facade. Helper-call count dropped, and the
  remaining helper family is still page-hotpath owned; do not treat this as a
  perf keeper.
- 2026-05-31: MIM-011 closed as a narrow source-shape keeper. After the page
  queue has selected an available page, `objectLifecycleSmallAlloc` now uses
  `page.acquireFreshSmall(size)` instead of the more generic
  `page.acquire_usize(size)`. This keeps the page-model owner boundary intact
  while avoiding the extra generic acquire fallback shape in the selected hot
  region.
- 2026-05-31: MIM-012 closed as a source-shape keeper inside the result
  capsule boundary. `resetAttempt()` combines the hot reset-plus-attempt
  transition without inlining result fields into the facade. This keeps capsule
  ownership intact while removing one public method call from the selected
  small-alloc region.
- 2026-05-31: MIM-013 closed as a source-shape keeper. The acquire failure path
  now relies on the existing `resetAttempt()` `last_block_id=-1` sentinel, and
  `recordBlock(block_id)` runs only after `block_id >= 0`. This removes the
  redundant failed-acquire block publication without changing public failure
  observation.

## MIM-001 Source-Shape Inventory

| Surface | Method(s) | Shape | Candidate read |
| --- | --- | --- | --- |
| small allocation | `objectLifecycleSmallAlloc` | resets and updates `alloc_result`, selects queue page, calls `page.acquireFreshSmall`, records last page cache | primary owner candidate; most direct allocation path |
| last-page cache write | `recordLastAllocPage` | writes `last_alloc_page_index`, `last_alloc_page_id`, `last_alloc_page` after successful small alloc | candidate only as part of small allocation, not standalone |
| cached release | `objectLifecycleReleaseDirectCachedPage`, `objectLifecycleReleaseBlock` | checks last allocated page fields, calls `page.releaseLocalKnownLive`, falls back to known-page lookup | secondary owner candidate; already source-shaped for fast path |
| known-page lookup | `objectLifecycleKnownPageIndexById`, `objectLifecycleReleaseKnownPageIndex` | scans `object_lifecycle_queue.pages` when cache misses | fallback surface; optimize only with current perf evidence |
| aligned alloc | `objectLifecycleSmallAllocAligned` | normalizes alignment, then delegates to small alloc | not first owner; mostly wrapper around small allocation |
| realloc shrink | `objectLifecycleReallocShrink`, `validateReallocShrinkPage` | validates page/block state and records success/failure | not first owner unless realloc workload is active |
| realloc grow | `objectLifecycleReallocGrow`, `objectLifecycleReallocGrowFromPage` | validates old block, calls small alloc, then release, records move | candidate only if grow workload is the active perf owner |
| observers/stats | `objectLifecycle*Count`, result getters, `objectLifecycleStatsSnapshot` | readback over queue/result fields | not implementation owner; keep as public observer surface |

Likely owner candidates for MIM-002:

```text
candidate_0=objectLifecycleSmallAlloc
candidate_1=objectLifecycleReleaseDirectCachedPage
candidate_2=objectLifecycleReallocGrowFromPage
fallback_surface=objectLifecycleKnownPageIndexById
observer_surface=objectLifecycle* getters / stats snapshot
```

## MIM-002 Owner Selection

Selected owner:

```text
selected_owner=objectLifecycleSmallAlloc
selected_reason=representative_small_block_workload_enters_facade_through_small_alloc_and_uses_release_realloc_as_secondary_paths
implementation_open=0
fast_path_reopen=0
```

Rejected for first source edit:

| Candidate | Reason |
| --- | --- |
| `objectLifecycleReleaseDirectCachedPage` | secondary release path; current smoke already proves cached release correctness, but it is not the first allocation entry |
| `objectLifecycleReallocGrowFromPage` | composite path built from small alloc plus release; optimize only when realloc workload is active |
| `objectLifecycleKnownPageIndexById` | fallback scan; no source edit without perf evidence that cache misses dominate |
| observers / stats | public readback surface; not a hot source-level owner candidate |

MIM-003 must gather current perf evidence before source edits. The selected
source owner does not reopen Array, RuntimeDataBox, helper, provider,
replacement, hook, or global allocator work.

## MIM-003 Perf Evidence Refresh

Command shape:

```text
hako_exe_memory_runner:
  app=apps/hako-alloc-mimalloc-comparison-in-process-object-lifecycle-small-block-proof/main.hako
  workload=representative-object-lifecycle-small-block-v0
  runtime_config=empty
  operation_repeat=1

mir tools:
  tools/allocator/mir_callsite_copy_attribution.py
  tools/allocator/hako_mimalloc_small_alloc_helper_copy_family_probe.py
```

Before source edit:

```text
body_elapsed_ns=550000000
allocation_count=524288
free_count=524288
select_page_single_fast_path_count=524288
select_page_single_fallback_count=0
release_known_page_fast_path_count=524288
release_known_page_fallback_count=0
instruction_count=153
call_count=12
copy_count=61
phi_count=18
helper_call_count=6
helper_copy_count=22
dominant_callee_family=page_hotpath_helpers
dominant_copy_owner=local_ssa_copy_materialization
callsite_0_callee=acquire_usize
callsite_0_attributed_copy_count=8
helper_family_call_count=5
facade_result_helpers_call_count=1
page_hotpath_helpers_call_count=4
summary=ok
```

Interpretation:

```text
selected_source_boundary=objectLifecycleSmallAlloc.alloc_result_reset_binding
selected_reason=source_has_one_remaining_facade_result_wrapper_call_before_alloc_result_local_binding
fast_path_reopen=0
implementation_open=1
```

## MIM-004 / MIM-005 Narrow Source Cleanup

Changed only:

```text
file=lang/src/hako_alloc/memory/object_lifecycle_facade_box.hako
method=HakoAllocObjectLifecycleFacade.objectLifecycleSmallAlloc/1
change=bind alloc_result local before reset and call alloc_result.reset directly
```

After source edit:

```text
body_elapsed_ns=562000000
allocation_count=524288
free_count=524288
select_page_single_fast_path_count=524288
select_page_single_fallback_count=0
release_known_page_fast_path_count=524288
release_known_page_fallback_count=0
instruction_count=154
call_count=12
copy_count=62
phi_count=18
helper_call_count=5
helper_copy_count=21
dominant_callee_family=page_hotpath_helpers
dominant_copy_owner=local_ssa_copy_materialization
callsite_0_callee=acquire_usize
callsite_0_attributed_copy_count=8
helper_family_call_count=4
facade_result_helpers_call_count=0
page_hotpath_helpers_call_count=4
summary=ok
```

3-sample timing smoke after source edit:

```text
sample_count=3
sample_0_hako_external_elapsed_ms=560
sample_1_hako_external_elapsed_ms=570
sample_2_hako_external_elapsed_ms=550
after_hako_elapsed_median_ms=560
after_hako_elapsed_min_ms=550
after_hako_elapsed_max_ms=570
winner_claim=0
replacement_active=0
hook_installed=0
global_allocator=0
summary=ok
```

Decision:

```text
cleanup_effect=accepted_as_source_shape_cleanup
perf_keeper_claim=0
remaining_owner=local_ssa_copy_materialization_and_page_hotpath_helpers
next_task=MIM-008_or_MIM-009_cleanup_then_resume_owner_selection
```

## Evidence

- Active handoff guard:
  `bash tools/checks/k2_wide_phase296x_mimalloc_source_level_owner_refresh_guard.sh`
- Direct-path closeout guard:
  `bash tools/checks/k2_wide_phase296x_post_directarray_remaining_direct_path_surface_check_guard.sh`
- Current pointer guard:
  `bash tools/checks/current_state_pointer_guard.sh`

### MIM-011 Evidence

Current baseline before the source edit:

```text
sample_count=3
body_elapsed_ns=555000000,555000000,557000000
external_elapsed_ms=560,550,560
summary=ok
```

After switching the selected page acquire route to `acquireFreshSmall`:

```text
sample_count=3
body_elapsed_ns=544000000,544000000,546000000
external_elapsed_ms=550,540,550
allocation_count=524288
free_count=524288
select_page_single_fast_path_count=524288
release_known_page_fast_path_count=524288
summary=ok
```

MIR shape after the source edit:

```text
instruction_count=139
call_count=10
copy_count=63
helper_call_count=3
page_hotpath_helpers_call_count=3
dominant_callee_family=page_hotpath_helpers
dominant_copy_owner=local_ssa_copy_materialization
top_callsite_callee=acquireFreshSmall
top_callsite_attributed_copy_count=8
winner_claim=0
replacement_active=0
hook_installed=0
global_allocator=0
```

### MIM-012 Evidence

Baseline before the source edit is the MIM-011 selected-page acquire route:

```text
sample_count=3
body_elapsed_ns=544000000,544000000,546000000
external_elapsed_ms=550,540,550
summary=ok
```

After adding `HakoAllocObjectLifecycleAllocResult.resetAttempt()` and using it
from `objectLifecycleSmallAlloc`:

```text
sample_count=3
body_elapsed_ns=540000000,538000000,538000000
allocation_count=524288
free_count=524288
select_page_single_fast_path_count=524288
release_known_page_fast_path_count=524288
summary=ok
```

MIR shape after the source edit:

```text
instruction_count=133
call_count=10
copy_count=61
phi_count=14
helper_call_count=3
helper_copy_count=13
dominant_callee_family=page_hotpath_helpers
dominant_copy_owner=local_ssa_copy_materialization
top_callsite_callee=acquireFreshSmall
top_callsite_attributed_copy_count=8
facade_result_helpers_call_count=0
winner_claim=0
replacement_active=0
hook_installed=0
global_allocator=0
```

### MIM-013 Evidence

Baseline before the source edit is the MIM-012 reset-attempt capsule cleanup:

```text
sample_count=3
body_elapsed_ns=540000000,538000000,538000000
summary=ok
```

After deferring `recordBlock(block_id)` until after `block_id >= 0`:

```text
sample_count=3
body_elapsed_ns=543000000,536000000,536000000
allocation_count=524288
free_count=524288
select_page_single_fast_path_count=524288
release_known_page_fast_path_count=524288
summary=ok
```

MIR shape after the source edit:

```text
instruction_count=132
call_count=10
copy_count=60
phi_count=14
helper_call_count=3
helper_copy_count=11
dominant_callee_family=page_hotpath_helpers
dominant_copy_owner=local_ssa_copy_materialization
top_callsite_callee=acquireFreshSmall
top_callsite_attributed_copy_count=6
facade_result_helpers_call_count=0
winner_claim=0
replacement_active=0
hook_installed=0
global_allocator=0
```

## Parking Lot

- Array lane extension backlog remains in
  `docs/development/current/main/design/array-lane-extension-roadmap-ssot.md`.
- RuntimeDataBox route policy archaeology stays historical unless a current
  mimalloc perf pass selects it again.
- DirectArray optional member work stays closed until selected by current
  mimalloc evidence.
