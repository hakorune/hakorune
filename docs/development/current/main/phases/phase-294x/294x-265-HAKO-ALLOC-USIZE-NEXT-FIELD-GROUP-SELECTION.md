---
Status: Landed
Date: 2026-05-24
Scope: select the next row after legacy page-heap non-id state migration.
Blocker: HAKO-ALLOC-USIZE-FIELD-GROUP-NEXT-SELECTION-009
Related:
  - lang/src/hako_alloc/memory/page_heap_box.hako
  - lang/src/hako_alloc/memory/NUMERIC_FIELDS.md
  - docs/development/current/main/phases/phase-294x/294x-264-HAKO-ALLOC-USIZE-PAGE-HEAP-FREE-TOP.md
---

# 294x-265 Hako Alloc Usize Next Field Group Selection

## Decision

Close:

```text
HAKO-ALLOC-USIZE-FIELD-GROUP-NEXT-SELECTION-009
```

Do not select another page-heap identity field yet.

Select a closeout row instead:

```text
HAKO-ALLOC-USIZE-PAGE-HEAP-NON-ID-CLOSEOUT-001
```

## Why

The legacy comparison-slice `HakoAllocPage` non-id state is now exact:

- stats counters;
- occupancy counters;
- requested bytes;
- block size;
- capacity;
- free-stack top;
- handle requested-size payload.

The remaining legacy page-heap fields are identity/index seams:

- `HakoAllocPage.page_id`;
- `HakoAllocHandle.page_id`;
- `HakoAllocHandle.block_id`.

Those fields cross object identity, result printing, and stale-handle checks.
They should not be pulled into the exact `usize` lane as a side effect of
finishing size/count storage. Close out the non-id slice first, then decide
whether id/index migration is still needed for the comparison vertical slice.

## Stop Line

The closeout row must not change:

- page/handle id storage;
- method parameter surfaces;
- `requested_sizes` array payload semantics;
- production `HakoAllocPageModel` fields;
- page-map entries, pointer-like payloads, provider/DLL seams, hooks,
  worker/TLS, atomics, or `#[global_allocator]`.

## Next Row

Implement:

```text
HAKO-ALLOC-USIZE-PAGE-HEAP-NON-ID-CLOSEOUT-001
```

Expected verification should assert the exact typed-object storage shape and
run the comparison-slice page-heap proof apps.

## Verification

```bash
git diff --check
bash tools/checks/current_state_pointer_guard.sh
```
