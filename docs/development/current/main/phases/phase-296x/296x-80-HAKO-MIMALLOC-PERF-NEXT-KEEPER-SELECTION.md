---
Status: Landed
Date: 2026-05-27
Scope: select the next single keeper after post-release measurement.
Blocker: HAKO-MIMALLOC-PERF-NEXT-KEEPER-SELECTION-296X-001
Related:
  - docs/development/current/main/phases/phase-296x/296x-77-HAKO-CHECK-PERF-SURFACE-INVENTORY.md
  - docs/development/current/main/phases/phase-296x/296x-79-HAKO-MIMALLOC-PERF-POST-RELEASE-KEEPER-MEASUREMENT.md
---

# 296x-80 Hako Mimalloc Perf Next Keeper Selection

## Purpose

Select exactly one next keeper candidate from hako_check perf-surface evidence
and post-release measurement. Do not implement the keeper in this row.

## Required Output

```text
output_contract=hako-mimalloc-perf-next-keeper-selection-v0
input_contract=hako-mimalloc-perf-post-release-keeper-measurement-v0
previous_keeper=release_known_page_fast_path
next_keeper
selection_reason
winner_claim=0
replacement_active=0
summary=ok
```

## Stop Line

This row only selects the next keeper. Implementation belongs to the next row.

## Landed Evidence

```text
output_contract=hako-mimalloc-perf-next-keeper-selection-v0
input_contract=hako-mimalloc-perf-post-release-keeper-measurement-v0
previous_keeper=release_known_page_fast_path
next_keeper=select_page_single_page_fast_path
selection_reason=hako_check_perf_surface_found_objectLifecycleSmallAlloc_selectPage_hot_path
selected_target_file=lang/src/hako_alloc/memory/object_lifecycle_facade_box.hako
selected_target_method=objectLifecycleSmallAlloc
implementation_row=HAKO-MIMALLOC-PERF-SELECT-PAGE-SINGLE-PAGE-FAST-PATH-296X-001
winner_claim=0
replacement_active=0
summary=ok
```

Guard:

```bash
bash tools/checks/k2_wide_phase296x_hako_mimalloc_perf_next_keeper_selection_guard.sh
```
