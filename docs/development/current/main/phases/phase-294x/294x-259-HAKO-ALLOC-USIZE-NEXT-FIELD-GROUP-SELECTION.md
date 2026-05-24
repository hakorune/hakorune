---
Status: Landed
Date: 2026-05-24
Scope: select the next explicit non-negative production field group after handle requested-size migration.
Blocker: HAKO-ALLOC-USIZE-FIELD-GROUP-NEXT-SELECTION-006
Related:
  - lang/src/hako_alloc/memory/page_heap_box.hako
  - lang/src/hako_alloc/memory/NUMERIC_FIELDS.md
  - docs/development/current/main/phases/phase-294x/294x-258-HAKO-ALLOC-USIZE-PAGE-HEAP-HANDLE-REQUESTED-SIZE.md
---

# 294x-259 Hako Alloc Usize Next Field Group Selection

## Decision

Close:

```text
HAKO-ALLOC-USIZE-FIELD-GROUP-NEXT-SELECTION-006
```

Select the legacy page-heap block-size payload as the next field group:

```text
HAKO-ALLOC-USIZE-FIELD-GROUP-259-PAGE-HEAP-BLOCK-SIZE-001
```

Field:

- `HakoAllocPage.block_size`

## Why This Group

`HakoAllocPage.block_size` is a non-negative size-class payload initialized
from `LayoutBox.class_size(...)` and consumed by the legacy comparison-slice
allocation/realloc checks. It is narrower than `capacity` / `free_top` because
it does not own stack-top mutation, loop bounds, or underflow behavior.

The source request and method parameter surfaces stay current-lane. This row
only migrates the stored page block-size payload.

## Stop Line

The migration row must not change:

- `HakoAllocPage.page_id`, `capacity`, or `free_top`;
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
HAKO-ALLOC-USIZE-FIELD-GROUP-259-PAGE-HEAP-BLOCK-SIZE-001
```

Expected code change:

- `HakoAllocPage.block_size: usize`

Expected verification should include the existing page-heap requested-bytes and
object-return/result-contract proof apps because allocation/realloc compare
the request size against this payload.

## Verification

```bash
git diff --check
bash tools/checks/current_state_pointer_guard.sh
```
