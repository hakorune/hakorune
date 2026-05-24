---
Status: Landed
Date: 2026-05-24
Scope: select the next explicit non-negative production field group after page-heap block-size migration.
Blocker: HAKO-ALLOC-USIZE-FIELD-GROUP-NEXT-SELECTION-007
Related:
  - lang/src/hako_alloc/memory/page_heap_box.hako
  - lang/src/hako_alloc/memory/NUMERIC_FIELDS.md
  - docs/development/current/main/phases/phase-294x/294x-260-HAKO-ALLOC-USIZE-PAGE-HEAP-BLOCK-SIZE.md
---

# 294x-261 Hako Alloc Usize Next Field Group Selection

## Decision

Close:

```text
HAKO-ALLOC-USIZE-FIELD-GROUP-NEXT-SELECTION-007
```

Select the legacy page-heap capacity payload as the next field group:

```text
HAKO-ALLOC-USIZE-FIELD-GROUP-261-PAGE-HEAP-CAPACITY-001
```

Field:

- `HakoAllocPage.capacity`

## Why This Group

`HakoAllocPage.capacity` is a non-negative capacity payload initialized from
`LayoutBox.class_capacity(...)` and used by the legacy comparison-slice seed and
live-handle bounds. It is narrower than `free_top` because it does not own
mutable stack-top decrement/increment or underflow behavior.

## Stop Line

The migration row must not change:

- `HakoAllocPage.page_id` or `free_top`;
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
HAKO-ALLOC-USIZE-FIELD-GROUP-261-PAGE-HEAP-CAPACITY-001
```

Expected code change:

- `HakoAllocPage.capacity: usize`

Expected verification should include small/mixed allocation proofs plus
object-return/result-contract proofs because seed and live-handle bounds consume
capacity.

## Verification

```bash
git diff --check
bash tools/checks/current_state_pointer_guard.sh
```
