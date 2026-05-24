---
Status: Landed
Date: 2026-05-24
Scope: select the next explicit non-negative production field group after page-queue inventory cleanup.
Blocker: HAKO-ALLOC-USIZE-FIELD-GROUP-NEXT-SELECTION-003
Related:
  - lang/src/hako_alloc/memory/page_heap_box.hako
  - lang/src/hako_alloc/memory/NUMERIC_FIELDS.md
  - docs/development/current/main/phases/phase-294x/294x-252-HAKO-ALLOC-USIZE-PAGE-QUEUE-NUMERIC-INVENTORY-DRIFT-CLEANUP.md
---

# 294x-253 Hako Alloc Usize Next Field Group Selection

## Decision

Close:

```text
HAKO-ALLOC-USIZE-FIELD-GROUP-NEXT-SELECTION-003
```

Select the legacy page-heap occupancy pair as the next field group:

```text
HAKO-ALLOC-USIZE-FIELD-GROUP-253-PAGE-HEAP-OCCUPANCY-001
```

Fields:

- `HakoAllocPage.current_used`
- `HakoAllocPage.peak_used`

## Why This Group

`current_used` and `peak_used` are non-negative occupancy counters in the
legacy `page_heap_box.hako` policy-state owner. They are still observed by
`mimalloc-lite`, `allocator-stress`, and object-return/result-contract proof
apps.

They are narrower than the remaining candidates because:

- they are not page/block identity fields;
- they do not carry `-1` sentinels;
- they are not reason/status vocabularies;
- release decrements are guarded by live-handle and `block_used` checks;
- `peak_used` only mirrors the maximum observed `current_used`.

## Stop Line

The migration row must not change:

- `HakoAllocHandle.page_id`, `block_id`, or `requested_size`;
- `HakoAllocPage.page_id`, `block_size`, `capacity`, `free_top`, or
  `requested_bytes`;
- page-model production fields, page-map entries, pointer-like payloads,
  provider/DLL seams, hooks, worker/TLS, atomics, or `#[global_allocator]`.

## Next Row

Implement:

```text
HAKO-ALLOC-USIZE-FIELD-GROUP-253-PAGE-HEAP-OCCUPANCY-001
```

Expected code change:

- `HakoAllocPage.current_used: usize = 0`
- `HakoAllocPage.peak_used: usize = 0`

Expected verification should include the existing `mimalloc-lite` and
`allocator-stress` proof apps because they observe `peak_used`.

## Verification

```bash
git diff --check
bash tools/checks/current_state_pointer_guard.sh
```
