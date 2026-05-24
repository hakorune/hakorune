---
Status: Landed
Date: 2026-05-24
Scope: migrate legacy page-heap requested-byte accumulator to exact `usize`.
Blocker: HAKO-ALLOC-USIZE-FIELD-GROUP-255-PAGE-HEAP-REQUESTED-BYTES-001
Related:
  - docs/development/current/main/phases/phase-294x/294x-255-HAKO-ALLOC-USIZE-NEXT-FIELD-GROUP-SELECTION.md
  - lang/src/hako_alloc/memory/page_heap_box.hako
  - lang/src/hako_alloc/memory/NUMERIC_FIELDS.md
  - tools/checks/k2_wide_hako_alloc_usize_page_heap_requested_bytes_guard.sh
---

# 294x-256 Hako Alloc Usize Page Heap Requested Bytes

## Decision

Close:

```text
HAKO-ALLOC-USIZE-FIELD-GROUP-255-PAGE-HEAP-REQUESTED-BYTES-001
```

`HakoAllocPage.requested_bytes` is now an exact `usize` field.

## Implementation

- Changed `HakoAllocPage.requested_bytes` from `i64` to `usize`.
- Kept `HakoAllocHandle.requested_size` signed.
- Kept `HakoAllocPage.page_id`, `block_size`, `capacity`, and `free_top`
  signed.
- Updated `NUMERIC_FIELDS.md`.
- Updated older page-heap guards so they no longer expect requested bytes to
  stay signed.
- Added a narrow requested-bytes guard that runs `mimalloc-lite`,
  `allocator-stress`, and `boxtorrent-mini`, then checks MIR typed-object
  storage.

## Stop Line

This row does not migrate:

- `HakoAllocHandle.page_id`, `block_id`, or `requested_size`;
- `HakoAllocPage.page_id`, `block_size`, `capacity`, or `free_top`;
- `HakoAllocPage.requested_sizes` array payload semantics;
- production `HakoAllocPageModel` fields;
- page-map entries, pointer-like payloads, provider/DLL seams, hooks,
  worker/TLS, atomics, or `#[global_allocator]`.

## Next Row

Return to explicit field-group selection:

```text
HAKO-ALLOC-USIZE-FIELD-GROUP-NEXT-SELECTION-005
```

## Verification

```bash
bash tools/checks/k2_wide_hako_alloc_usize_page_heap_requested_bytes_guard.sh
bash tools/checks/k2_wide_hako_alloc_usize_page_heap_occupancy_guard.sh
bash tools/checks/k2_wide_hako_alloc_usize_page_heap_stats_counters_guard.sh
git diff --check
bash tools/checks/current_state_pointer_guard.sh
```
