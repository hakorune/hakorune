---
Status: Landed
Date: 2026-05-28
Scope: refresh source/MIR after the small-alloc acquire_usize fast path measurement.
Blocker: POST-PAGE-ACQUIRE-USIZE-SOURCE-MIR-REFRESH-296X-001
Related:
  - docs/development/current/main/phases/phase-296x/296x-153-POST-PAGE-ACQUIRE-USIZE-FAST-PATH-MEASUREMENT.md
---

# 296x-154 Post Page Acquire Usize Source/MIR Refresh

## Purpose

Refresh source/MIR after measuring the acquire_usize fast path keeper, then
select the next owner without mixing page-model keepers and compiler
helper-copy work in the same row.

## Required Output

```text
output_contract=post-page-acquire-usize-source-mir-refresh-v0
input_contract=post-page-acquire-usize-fast-path-measurement-v0
active_owner
selected_next
summary=ok
```

## Evidence

```text
output_contract=post-page-acquire-usize-source-mir-refresh-v0
input_contract=post-page-acquire-usize-fast-path-measurement-v0
measurement_scope=object_lifecycle_facade_exact_exe_after_acquire_usize_fast_path
workload_id=representative-object-lifecycle-small-block-v0
operation_repeat=8192
sample_count=1
release_known_page_fast_path_count=524288
release_known_page_fallback_count=0
sample_0_hako_external_elapsed_ms=560
elapsed_median_ms=560
elapsed_min_ms=560
elapsed_max_ms=560
external_rss_median_bytes=3534848
previous_checkpoint_median_ms=600
previous_checkpoint_source=296x-149-post-known-live-release-measurement
source_target_file=lang/src/hako_alloc/memory/object_lifecycle_facade_box.hako
source_target_box=HakoAllocObjectLifecycleFacade
source_target_method=objectLifecycleSmallAlloc
source_method_call_count=21
source_field_get_count=8
source_field_set_count=2
source_result_capsule_churn=1
source_hot_path_reason=source_selectPage_hot_path
small_alloc_mir_instruction_count=185
small_alloc_call_count=9
small_alloc_copy_count=34
small_alloc_receiver_copy_count=23
small_alloc_arg_copy_count=8
small_alloc_result_copy_count=3
small_alloc_local_ssa_copy_count=68
dominant_copy_family=helper_result_local_ssa
dominant_callee_family=facade_result_helpers
active_owner=compiler_helper_copy
selected_next=same_module_helper_call_lowering_seam
keeper_effect=accepted
winner_claim=0
replacement_active=0
hook_installed=0
global_allocator=0
summary=ok
```

Interpretation:

```text
Caching `alloc_result` locally in `objectLifecycleSmallAlloc` reduced the
facade result helper surface without reopening the page-model keeper lane. The
exact-EXE scout remained accepted at 560ms against the 600ms checkpoint, but
the remaining dominant family is still helper_result_local_ssa, so the next row
should attack same-module helper call lowering for the remaining simple setter
helpers.
```

Guard:

```bash
bash tools/checks/k2_wide_phase296x_post_page_acquire_usize_source_mir_refresh_guard.sh
```
