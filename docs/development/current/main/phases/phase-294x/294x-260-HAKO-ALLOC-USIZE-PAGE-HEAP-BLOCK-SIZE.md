---
Status: Landed
Date: 2026-05-24
Scope: migrate the legacy page-heap block-size payload to exact `usize`.
Blocker: HAKO-ALLOC-USIZE-FIELD-GROUP-259-PAGE-HEAP-BLOCK-SIZE-001
Related:
  - lang/src/hako_alloc/memory/page_heap_box.hako
  - lang/src/hako_alloc/memory/NUMERIC_FIELDS.md
  - docs/development/current/main/phases/phase-294x/294x-259-HAKO-ALLOC-USIZE-NEXT-FIELD-GROUP-SELECTION.md
---

# 294x-260 Hako Alloc Usize Page Heap Block Size

## Decision

Close:

```text
HAKO-ALLOC-USIZE-FIELD-GROUP-259-PAGE-HEAP-BLOCK-SIZE-001
```

Migrate only the stored legacy page-heap block-size payload:

```hako
box HakoAllocPage {
    block_size: usize
}
```

This makes the legacy comparison-slice page size-class payload exact while
leaving stack-top and capacity mutation in the current signed lane.

The row also makes the existing live-handle observer return contract explicit:

```hako
isLiveHandle(handle): i64
```

That observer was already documented and implemented as a scalar 1/0 status.
The annotation keeps pure-first route metadata stable after exact numeric field
storage is introduced; it does not change parameter surfaces or behavior.

## Stop Line

This row does not change:

- `HakoAllocPage.page_id`, `capacity`, or `free_top`;
- `HakoAllocHandle.page_id`, `block_id`, or `requested_size`;
- `HakoAllocPage.requested_sizes` array payload semantics;
- method parameter types for `allocate`, `resizeInPlace`, `realloc`, or
  `reallocResult`;
- production `HakoAllocPageModel` fields;
- page-map entries, pointer-like payloads, provider/DLL seams, hooks,
  worker/TLS, atomics, or `#[global_allocator]`.

## Verification

```bash
bash tools/checks/k2_wide_hako_alloc_usize_page_heap_block_size_guard.sh
bash tools/checks/k2_wide_hako_alloc_usize_page_heap_requested_bytes_guard.sh
bash tools/checks/k2_wide_hako_alloc_usize_page_heap_handle_requested_size_guard.sh
git diff --check
bash tools/checks/current_state_pointer_guard.sh
```

## Next Row

Resume explicit field-group selection:

```text
HAKO-ALLOC-USIZE-FIELD-GROUP-NEXT-SELECTION-007
```
