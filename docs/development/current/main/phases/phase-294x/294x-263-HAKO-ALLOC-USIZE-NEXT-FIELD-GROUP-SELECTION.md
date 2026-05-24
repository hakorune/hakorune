---
Status: Landed
Date: 2026-05-24
Scope: select the next explicit non-negative production field group after page-heap capacity migration.
Blocker: HAKO-ALLOC-USIZE-FIELD-GROUP-NEXT-SELECTION-008
Related:
  - lang/src/hako_alloc/memory/page_heap_box.hako
  - lang/src/hako_alloc/memory/NUMERIC_FIELDS.md
  - docs/development/current/main/phases/phase-294x/294x-262-HAKO-ALLOC-USIZE-PAGE-HEAP-CAPACITY.md
---

# 294x-263 Hako Alloc Usize Next Field Group Selection

## Decision

Close:

```text
HAKO-ALLOC-USIZE-FIELD-GROUP-NEXT-SELECTION-008
```

Select the legacy page-heap free-stack top payload as the next field group:

```text
HAKO-ALLOC-USIZE-FIELD-GROUP-263-PAGE-HEAP-FREE-TOP-001
```

Field:

- `HakoAllocPage.free_top`

## Why This Group

`HakoAllocPage.free_top` is a non-negative stack-top count. Allocation already
checks `free_top == 0` before decrementing, and release only pushes back live
handles owned by the page. Now that `capacity` is exact, this stack-top field is
the next narrow comparison-slice state to migrate.

## Stop Line

The migration row must not change:

- `HakoAllocPage.page_id`;
- `HakoAllocHandle.page_id`, `block_id`, or `requested_size`;
- `HakoAllocPage.requested_sizes` array payload semantics;
- method parameter types for `allocate`, `resizeInPlace`, `realloc`, or
  `reallocResult`;
- production `HakoAllocPageModel` fields;
- page-map entries, pointer-like payloads, provider/DLL seams, hooks,
  worker/TLS, atomics, or `#[global_allocator]`.

## Next Row

Implement:

```text
HAKO-ALLOC-USIZE-FIELD-GROUP-263-PAGE-HEAP-FREE-TOP-001
```

Expected code change:

- `HakoAllocPage.free_top: usize`

Expected verification should include small/mixed allocation proofs plus
object-return/result-contract proofs because allocation/release stack mutation
consume this payload.

## Verification

```bash
git diff --check
bash tools/checks/current_state_pointer_guard.sh
```
