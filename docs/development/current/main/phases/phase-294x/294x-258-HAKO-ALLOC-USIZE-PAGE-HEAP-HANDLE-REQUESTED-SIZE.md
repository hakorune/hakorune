---
Status: Landed
Date: 2026-05-24
Scope: migrate the legacy page-heap handle requested-size payload to exact `usize`.
Blocker: HAKO-ALLOC-USIZE-FIELD-GROUP-257-PAGE-HEAP-HANDLE-REQUESTED-SIZE-001
Related:
  - lang/src/hako_alloc/memory/page_heap_box.hako
  - lang/src/hako_alloc/memory/NUMERIC_FIELDS.md
  - docs/development/current/main/phases/phase-294x/294x-257-HAKO-ALLOC-USIZE-NEXT-FIELD-GROUP-SELECTION.md
---

# 294x-258 Hako Alloc Usize Page Heap Handle Requested Size

## Decision

Close:

```text
HAKO-ALLOC-USIZE-FIELD-GROUP-257-PAGE-HEAP-HANDLE-REQUESTED-SIZE-001
```

Migrate only the stored legacy handle requested-size payload:

```hako
box HakoAllocHandle {
    requested_size: usize
}
```

This records the accepted allocation/realloc request size as an exact
non-negative payload after the existing request guards have rejected invalid
sizes.

## Stop Line

This row does not change:

- `HakoAllocHandle.page_id` or `block_id`;
- `HakoAllocPage.page_id`, `block_size`, `capacity`, or `free_top`;
- `HakoAllocPage.requested_sizes` array payload semantics;
- method parameter types for `allocate`, `resizeInPlace`, `realloc`, or
  `reallocResult`;
- production `HakoAllocPageModel` fields;
- page-map entries, pointer-like payloads, provider/DLL seams, hooks,
  worker/TLS, atomics, or `#[global_allocator]`.

## Verification

```bash
bash tools/checks/k2_wide_hako_alloc_usize_page_heap_handle_requested_size_guard.sh
bash tools/checks/k2_wide_hako_alloc_usize_page_heap_requested_bytes_guard.sh
git diff --check
bash tools/checks/current_state_pointer_guard.sh
```

## Next Row

Resume explicit field-group selection:

```text
HAKO-ALLOC-USIZE-FIELD-GROUP-NEXT-SELECTION-006
```
