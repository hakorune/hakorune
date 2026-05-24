---
Status: Landed
Date: 2026-05-24
Scope: migrate legacy page-heap occupancy fields to exact `usize`.
Blocker: HAKO-ALLOC-USIZE-FIELD-GROUP-253-PAGE-HEAP-OCCUPANCY-001
Related:
  - docs/development/current/main/phases/phase-294x/294x-253-HAKO-ALLOC-USIZE-NEXT-FIELD-GROUP-SELECTION.md
  - lang/src/hako_alloc/memory/page_heap_box.hako
  - lang/src/hako_alloc/memory/NUMERIC_FIELDS.md
  - tools/checks/k2_wide_hako_alloc_usize_page_heap_occupancy_guard.sh
---

# 294x-254 Hako Alloc Usize Page Heap Occupancy

## Decision

Close:

```text
HAKO-ALLOC-USIZE-FIELD-GROUP-253-PAGE-HEAP-OCCUPANCY-001
```

`HakoAllocPage.current_used` and `HakoAllocPage.peak_used` are now exact
`usize` fields.

## Implementation

- Changed `HakoAllocPage.current_used` from `i64` to `usize`.
- Changed `HakoAllocPage.peak_used` from `i64` to `usize`.
- Kept `page_id`, `block_size`, `capacity`, `free_top`, and
  `requested_bytes` signed.
- Updated `NUMERIC_FIELDS.md`.
- Updated the older page-heap stats guard so its stop line no longer expects
  occupancy fields to stay signed.
- Added a narrow occupancy guard that runs the existing `mimalloc-lite` and
  `allocator-stress` proof apps and checks MIR typed-object storage.

## Stop Line

This row does not migrate:

- `HakoAllocHandle.page_id`, `block_id`, or `requested_size`;
- `HakoAllocPage.page_id`, `block_size`, `capacity`, `free_top`, or
  `requested_bytes`;
- production `HakoAllocPageModel` fields;
- page-map entries, pointer-like payloads, provider/DLL seams, hooks,
  worker/TLS, atomics, or `#[global_allocator]`.

## Next Row

Return to explicit field-group selection:

```text
HAKO-ALLOC-USIZE-FIELD-GROUP-NEXT-SELECTION-004
```

## Verification

```bash
bash tools/checks/k2_wide_hako_alloc_usize_page_heap_occupancy_guard.sh
bash tools/checks/k2_wide_hako_alloc_usize_page_heap_stats_counters_guard.sh
git diff --check
bash tools/checks/current_state_pointer_guard.sh
```
