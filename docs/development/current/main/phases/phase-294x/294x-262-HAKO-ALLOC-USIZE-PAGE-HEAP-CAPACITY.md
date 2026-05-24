---
Status: Landed
Date: 2026-05-24
Scope: migrate the legacy page-heap capacity payload to exact `usize`.
Blocker: HAKO-ALLOC-USIZE-FIELD-GROUP-261-PAGE-HEAP-CAPACITY-001
Related:
  - lang/src/hako_alloc/memory/page_heap_box.hako
  - lang/src/hako_alloc/memory/NUMERIC_FIELDS.md
  - docs/development/current/main/phases/phase-294x/294x-261-HAKO-ALLOC-USIZE-NEXT-FIELD-GROUP-SELECTION.md
---

# 294x-262 Hako Alloc Usize Page Heap Capacity

## Decision

Close:

```text
HAKO-ALLOC-USIZE-FIELD-GROUP-261-PAGE-HEAP-CAPACITY-001
```

Migrate only the stored legacy page-heap capacity payload:

```hako
box HakoAllocPage {
    capacity: usize
}
```

This makes the immutable page capacity exact while leaving `free_top` in the
current signed lane until the stack-top mutation / underflow row is selected.

## Stop Line

This row does not change:

- `HakoAllocPage.page_id` or `free_top`;
- `HakoAllocHandle.page_id`, `block_id`, or `requested_size`;
- `HakoAllocPage.requested_sizes` array payload semantics;
- method parameter types for `allocate`, `resizeInPlace`, `realloc`, or
  `reallocResult`;
- production `HakoAllocPageModel` fields;
- page-map entries, pointer-like payloads, provider/DLL seams, hooks,
  worker/TLS, atomics, or `#[global_allocator]`.

## Verification

```bash
bash tools/checks/k2_wide_hako_alloc_usize_page_heap_capacity_guard.sh
bash tools/checks/k2_wide_hako_alloc_usize_page_heap_block_size_guard.sh
git diff --check
bash tools/checks/current_state_pointer_guard.sh
```

## Next Row

Resume explicit field-group selection:

```text
HAKO-ALLOC-USIZE-FIELD-GROUP-NEXT-SELECTION-008
```
