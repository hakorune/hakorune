---
Status: Landed
Date: 2026-05-24
Scope: select the next narrow usize-maintenance row after realloc requested-size migration.
Blocker: HAKO-ALLOC-USIZE-FIELD-GROUP-NEXT-SELECTION-002
Related:
  - lang/src/hako_alloc/memory/NUMERIC_FIELDS.md
  - lang/src/hako_alloc/memory/page_queue_box.hako
  - docs/development/current/main/phases/phase-294x/294x-250-HAKO-ALLOC-USIZE-REALLOC-REQUESTED-SIZE-RESULT-OBSERVER.md
---

# 294x-251 Hako Alloc Usize Next Field Group Selection

## Decision

Close:

```text
HAKO-ALLOC-USIZE-FIELD-GROUP-NEXT-SELECTION-002
```

Before selecting another code migration, repair the stale page-queue numeric
inventory rows.

Selected next blocker:

```text
HAKO-ALLOC-USIZE-NUMERIC-INVENTORY-PAGE-QUEUE-DRIFT-CLEANUP-001
```

## Why This First

`HakoAllocPageQueue` storage is already exact for:

- `bin`
- `page_count`
- `direct_page_index`
- `add_count`
- `select_count`
- `direct_hit_count`
- `refresh_count`
- `reject_count`

However, the detailed `NUMERIC_FIELDS.md` rows still classify part of that
group as `i64`. The summary row already says those fields are exact via
`HAKO-ALLOC-USIZE-FIELD-GROUP-058`, so this is metadata drift, not a new
allocator behavior row.

## Stop Line

The cleanup row must not change code semantics. It only synchronizes numeric
inventory metadata and a narrow guard. Do not migrate `has_direct_page`,
signed sentinels, selection-kind vocabularies, ids/indexes beyond already
exact storage, broad owner state, provider/DLL seams, hooks, worker/TLS,
atomics, or `#[global_allocator]`.

## Next Row

Implement:

```text
HAKO-ALLOC-USIZE-NUMERIC-INVENTORY-PAGE-QUEUE-DRIFT-CLEANUP-001
```

After that cleanup lands, return to field-group selection.

## Verification

```bash
git diff --check
bash tools/checks/current_state_pointer_guard.sh
```
