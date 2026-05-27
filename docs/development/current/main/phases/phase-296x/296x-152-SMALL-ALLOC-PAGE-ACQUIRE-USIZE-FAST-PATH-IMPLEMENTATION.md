---
Status: Landed
Date: 2026-05-28
Scope: apply the selected small-alloc page acquire_usize fast path keeper.
Blocker: SMALL-ALLOC-PAGE-ACQUIRE-USIZE-FAST-PATH-IMPLEMENTATION-296X-001
Related:
  - docs/development/current/main/phases/phase-296x/296x-151-PAGE-ACQUIRE-FAST-PATH-KEEPER-SELECTION.md
---

# 296x-152 Small Alloc Page Acquire Usize Fast Path Implementation

## Purpose

Apply the selected keeper by routing the object-lifecycle small-alloc page
acquire call through `HakoAllocPageModel.acquire_usize/1`.

## Required Output

```text
output_contract=small-alloc-page-acquire-usize-fast-path-implementation-v0
input_contract=page-acquire-fast-path-keeper-selection-v0
selected_keeper=small_alloc_page_acquire_usize_fast_path
generic_page_acquire_preserved=1
semantic_summary=ok
summary=ok
```

## Guard Requirements

```text
The implementation guard must use a lightweight exact-EXE smoke and must not
run the full object-lifecycle proof app directly through the VM.
```

## Evidence

```text
output_contract=small-alloc-page-acquire-usize-fast-path-implementation-v0
input_contract=page-acquire-fast-path-keeper-selection-v0
selected_keeper=small_alloc_page_acquire_usize_fast_path
keeper_applied=1
generic_page_acquire_preserved=1
lightweight_exact_exe_proof_ok=1
allocation_count=64
free_count=64
release_known_page_fast_path_count=64
release_known_page_fallback_count=0
output_summary_ok=1
external_elapsed_ms=1
full_repeat_measurement_executed=0
winner_claim=0
replacement_active=0
semantic_summary=ok
selected_next=post_page_acquire_usize_fast_path_measurement
summary=ok
```

Guard:

```bash
bash tools/checks/k2_wide_phase296x_small_alloc_page_acquire_usize_fast_path_implementation_guard.sh
```
