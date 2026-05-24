---
Status: Landed
Date: 2026-05-24
Scope: close out the legacy page-heap exact non-id `usize` slice.
Blocker: HAKO-ALLOC-USIZE-PAGE-HEAP-NON-ID-CLOSEOUT-001
Related:
  - lang/src/hako_alloc/memory/page_heap_box.hako
  - lang/src/hako_alloc/memory/NUMERIC_FIELDS.md
  - docs/development/current/main/phases/phase-294x/294x-265-HAKO-ALLOC-USIZE-NEXT-FIELD-GROUP-SELECTION.md
---

# 294x-266 Hako Alloc Usize Page Heap Non-Id Closeout

## Decision

Close:

```text
HAKO-ALLOC-USIZE-PAGE-HEAP-NON-ID-CLOSEOUT-001
```

The legacy page-heap comparison-slice non-id state is exact:

- `HakoAllocHandle.requested_size`;
- `HakoAllocPage.block_size`;
- `HakoAllocPage.capacity`;
- `HakoAllocPage.free_top`;
- `HakoAllocPage.alloc_count`;
- `HakoAllocPage.free_count`;
- `HakoAllocPage.reuse_count`;
- `HakoAllocPage.current_used`;
- `HakoAllocPage.peak_used`;
- `HakoAllocPage.requested_bytes`.

Identity fields remain signed:

- `HakoAllocHandle.page_id`;
- `HakoAllocHandle.block_id`;
- `HakoAllocPage.page_id`.

## Stop Line

This closeout does not change:

- page/handle id storage;
- method parameter surfaces;
- `requested_sizes` array payload semantics;
- production `HakoAllocPageModel` fields;
- page-map entries, pointer-like payloads, provider/DLL seams, hooks,
  worker/TLS, atomics, or `#[global_allocator]`.

## Verification

```bash
bash tools/checks/k2_wide_hako_alloc_usize_page_heap_non_id_closeout_guard.sh
git diff --check
bash tools/checks/current_state_pointer_guard.sh
```

## Next Row

Resume row selection from:

```text
HAKO-ALLOC-USIZE-FIELD-GROUP-NEXT-SELECTION-010
```

The next selection should either pick a genuinely needed comparison-slice field
outside legacy page-heap non-id state or start a comparison report closeout. Do
not migrate page/handle ids by momentum alone.
