---
Status: Landed
Date: 2026-05-28
Scope: select one page-acquire keeper after known-live release source/MIR refresh.
Blocker: PAGE-ACQUIRE-FAST-PATH-KEEPER-SELECTION-296X-001
Related:
  - docs/development/current/main/phases/phase-296x/296x-150-POST-KNOWN-LIVE-RELEASE-SOURCE-MIR-REFRESH.md
---

# 296x-151 Page Acquire Fast Path Keeper Selection

## Purpose

Select exactly one page-acquire keeper from the remaining page-local ArrayBox
surface. Keep compiler helper-copy lowering parked as secondary.

## Required Output

```text
output_contract=page-acquire-fast-path-keeper-selection-v0
input_contract=post-known-live-release-source-mir-refresh-v0
selected_keeper
keeper_owner
fallback_preservation
summary=ok
```

## Evidence

```text
output_contract=page-acquire-fast-path-keeper-selection-v0
input_contract=post-known-live-release-source-mir-refresh-v0
active_owner=allocator_page_array_surface
baseline_page_acquire_mir_instruction_count=235
baseline_page_acquire_call_count=4
baseline_page_acquire_copy_count=116
baseline_page_acquire_array_get_call_count=2
baseline_page_acquire_array_set_call_count=2
candidate_0=small_alloc_page_acquire_usize_fast_path
candidate_0_method=HakoAllocPageModel.acquire_usize/1
candidate_0_mir_instruction_count=147
candidate_0_call_count=3
candidate_0_copy_count=80
candidate_0_array_get_call_count=1
candidate_0_semantics=preserves_retired_decommitted_size_checks_and_generic_acquire_fallback
candidate_1=small_alloc_page_acquire_fresh_small_fast_path
candidate_1_method=HakoAllocPageModel.acquireFreshSmall/1
candidate_1_mir_instruction_count=115
candidate_1_call_count=2
candidate_1_copy_count=64
candidate_1_array_get_call_count=1
candidate_1_semantics=drops_retired_decommitted_checks_and_local_free_collect_fallback
selected_keeper=small_alloc_page_acquire_usize_fast_path
keeper_owner=object_lifecycle_facade_small_alloc_page_acquire_callsite
keeper_kind=box_count
fallback_preservation=generic_page_acquire_preserved_when_free_top_is_zero
rejected_keeper=small_alloc_page_acquire_fresh_small_fast_path
rejected_reason=too_narrow_for_first_keeper_semantics
selected_next=small_alloc_page_acquire_usize_fast_path_implementation
winner_claim=0
replacement_active=0
summary=ok
```

Interpretation:

```text
The remaining heavy hot surface is page acquire. The first keeper should use
the existing acquire_usize fast path at the object-lifecycle small-alloc call
site because it reduces the hot method shape while preserving generic acquire
as the fallback when free_top is zero.
```

Guard:

```bash
bash tools/checks/k2_wide_phase296x_page_acquire_fast_path_keeper_selection_guard.sh
```
