---
Status: Landed
Date: 2026-05-28
Scope: refresh source/MIR observation after static-scalar call lowering measurement.
Blocker: POST-STATIC-SCALAR-SOURCE-MIR-REFRESH-296X-001
Related:
  - docs/development/current/main/phases/phase-296x/296x-138-STATIC-SCALAR-CALL-LOWERING-IMPLEMENTATION.md
  - docs/development/current/main/phases/phase-296x/296x-139-POST-STATIC-SCALAR-CALL-LOWERING-MEASUREMENT.md
---

# 296x-140 Post Static Scalar Source/MIR Refresh

## Purpose

Refresh source/MIR observation after static-scalar call lowering before choosing
another optimization keeper.

## Required Output

```text
output_contract=post-static-scalar-source-mir-refresh-v0
input_contract=post-static-scalar-call-lowering-measurement-v0
selected_method
remaining_call_surface
selected_next
summary=ok
```

## Evidence

Report:

```text
output_contract=post-static-scalar-source-mir-refresh-v0
input_contract=post-static-scalar-call-lowering-measurement-v0
selected_method=HakoAllocObjectLifecycleFacade.objectLifecycleSmallAlloc/1
remaining_call_surface=facade_result_helpers_and_page_hotpath
small_alloc_mir_instruction_count=185
small_alloc_call_count=16
small_alloc_copy_count=99
small_alloc_phi_count=15
small_alloc_record_failure_call_count=5
small_alloc_record_success_call_count=1
small_alloc_page_acquire_call_count=1
release_block_mir_instruction_count=127
release_block_call_count=12
release_block_copy_count=71
page_acquire_mir_instruction_count=235
page_acquire_call_count=4
page_acquire_copy_count=116
page_acquire_phi_count=15
page_acquire_array_get_call_count=2
page_acquire_array_set_call_count=2
source_small_alloc_method_call_count=21
source_small_alloc_result_capsule_churn=1
source_release_block_method_call_count=17
source_release_block_result_capsule_churn=1
gap_owner=compiler_lowering
gap_confidence=medium
next_diagnostic=small_alloc_call_copy_shape_deep_dive
selected_next=small_alloc_call_copy_shape_deep_dive
winner_claim=0
replacement_active=0
summary=ok
```

Interpretation:

```text
The static-scalar reason calls are gone, but objectLifecycleSmallAlloc still
has 16 calls, 99 copies, and 15 PHIs. The next row should classify the
call/copy materialization shape and page-local ArrayBox dynamic weight before
choosing another .hako keeper.
```

Worker source/MIR audit:

```text
MIR-side highest remaining surface:
  objectLifecycleSmallAlloc/1 call/copy materialization
  - 185 instructions
  - 16 calls
  - 99 copies
  - 15 PHIs

Source-side highest remaining surface:
  page-local ArrayBox pressure
  - HakoAllocPageModel.acquire/1 still uses free.get + block_used.set
  - releaseLocal/1 still uses block_used/local_free set/get
  - resetToFresh/0 runs during benchmark setup and performs dense ArrayBox sets

Park for now:
  cold fallback page scans and post-loop observers are not current primary
  surfaces for the exact proof workload.
```

Guard:

```bash
bash tools/checks/k2_wide_phase296x_post_static_scalar_source_mir_refresh_guard.sh
```
