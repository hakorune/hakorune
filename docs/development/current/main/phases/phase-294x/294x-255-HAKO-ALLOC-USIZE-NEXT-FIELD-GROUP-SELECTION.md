---
Status: Landed
Date: 2026-05-24
Scope: select the next explicit non-negative production field group after page-heap occupancy migration.
Blocker: HAKO-ALLOC-USIZE-FIELD-GROUP-NEXT-SELECTION-004
Related:
  - lang/src/hako_alloc/memory/page_heap_box.hako
  - lang/src/hako_alloc/memory/NUMERIC_FIELDS.md
  - docs/development/current/main/phases/phase-294x/294x-254-HAKO-ALLOC-USIZE-PAGE-HEAP-OCCUPANCY.md
---

# 294x-255 Hako Alloc Usize Next Field Group Selection

## Decision

Close:

```text
HAKO-ALLOC-USIZE-FIELD-GROUP-NEXT-SELECTION-004
```

Select the legacy page-heap requested-byte accumulator as the next field group:

```text
HAKO-ALLOC-USIZE-FIELD-GROUP-255-PAGE-HEAP-REQUESTED-BYTES-001
```

Field:

- `HakoAllocPage.requested_bytes`

## Why This Group

`requested_bytes` is a non-negative byte-length accumulator in the legacy
`page_heap_box.hako` policy-state owner. It is observed by `mimalloc-lite`,
`allocator-stress`, Boxtorrent mini, and comparison evidence paths.

This is the narrowest remaining page-heap payload candidate because:

- it is not a page/block identity field;
- it does not carry `-1` sentinels;
- it is not a reason/status vocabulary;
- it only accumulates accepted request sizes;
- `current_used` / `peak_used` already migrated, so the next visible page-heap
  metric is the byte-sum observer.

## Stop Line

The migration row must not change:

- `HakoAllocHandle.page_id`, `block_id`, or `requested_size`;
- `HakoAllocPage.page_id`, `block_size`, `capacity`, or `free_top`;
- `HakoAllocPage.requested_sizes` array payload semantics;
- production `HakoAllocPageModel` fields;
- page-map entries, pointer-like payloads, provider/DLL seams, hooks,
  worker/TLS, atomics, or `#[global_allocator]`.

## Next Row

Implement:

```text
HAKO-ALLOC-USIZE-FIELD-GROUP-255-PAGE-HEAP-REQUESTED-BYTES-001
```

Expected code change:

- `HakoAllocPage.requested_bytes: usize = 0`

Expected verification should include apps that print or compare requested byte
totals.

## Verification

```bash
git diff --check
bash tools/checks/current_state_pointer_guard.sh
```
