---
Status: Landed
Date: 2026-05-24
Scope: migrate one realloc requested-size observer field to exact `usize`.
Blocker: HAKO-ALLOC-USIZE-FIELD-GROUP-249-REALLOC-REQUESTED-SIZE-RESULT-OBSERVER-001
Related:
  - docs/development/current/main/phases/phase-294x/294x-249-HAKO-ALLOC-USIZE-NEXT-FIELD-GROUP-SELECTION.md
  - lang/src/hako_alloc/memory/object_lifecycle_facade_result_box.hako
  - lang/src/hako_alloc/memory/NUMERIC_FIELDS.md
  - tools/checks/k2_wide_hako_alloc_usize_realloc_requested_size_result_observer_guard.sh
---

# 294x-250 Hako Alloc Usize Realloc Requested-Size Result Observer

## Decision

Close:

```text
HAKO-ALLOC-USIZE-FIELD-GROUP-249-REALLOC-REQUESTED-SIZE-RESULT-OBSERVER-001
```

`HakoAllocObjectLifecycleReallocResult.last_requested_size` is now an exact
`usize` field.

## Implementation

- Changed `HakoAllocObjectLifecycleReallocResult.last_requested_size` from
  `i64` to `usize`.
- Kept nearby signed sentinel / reason / status fields unchanged:
  `last_page_id`, `last_block_id`, `last_new_page_id`,
  `last_new_block_id`, `last_reason`, and `last_ok`.
- Kept `HakoAllocObjectLifecycleAlignmentResult.last_requested` and
  `last_normalized` signed.
- Updated `NUMERIC_FIELDS.md` to record the exact field-group decision.
- Updated the existing realloc shrink EXE guard static expectation.
- Added a narrow guard for the field-group boundary.

## Stop Line

This row does not migrate:

- page/block ids or new page/block ids;
- reason vocabulary or ok/status flags;
- alignment result observers;
- huge requested-size observers;
- broad page/heap/queue/handle state;
- mimalloc comparison rows, provider calls, host replacement, hooks, global
  allocator install, worker/TLS, atomics, provider package / DLL generation,
  repeated benchmark packs, or `#[global_allocator]`.

## Next Row

Return to explicit field-group selection:

```text
HAKO-ALLOC-USIZE-FIELD-GROUP-NEXT-SELECTION-002
```

The next row should inspect `NUMERIC_FIELDS.md` and select one narrow
non-negative production field group. Do not migrate signed sentinels,
reason/status fields, ids/indexes, pointer payloads, or broad owner state by
default.

## Verification

```bash
bash tools/checks/k2_wide_hako_alloc_usize_realloc_requested_size_result_observer_guard.sh
bash tools/checks/k2_wide_mimalloc_facade_realloc_shrink_exe_guard.sh
git diff --check
bash tools/checks/current_state_pointer_guard.sh
```
