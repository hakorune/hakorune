---
Status: Landed
Date: 2026-05-31
Scope: row414 MSR-001 object lifecycle facade scan
Related:
  - docs/development/current/main/phases/phase-296x/296x-414-MIMALLOC-SOURCE-LEVEL-OWNER-REFRESH.md
  - lang/src/hako_alloc/memory/object_lifecycle_facade_box.hako
---

# MSR-001 Object Lifecycle Facade Scan

## Input

- `lang/src/hako_alloc/memory/object_lifecycle_facade_box.hako`

## Note

`HakoAllocObjectLifecycleFacade` is still a thin source-level owner surface.
It composes queue selection, small-alloc bookkeeping, release bookkeeping,
alignment normalization, and realloc bookkeeping, but it does not own provider
activation, OS page sourcing, remote-free, or backend shortcuts.

object_lifecycle_facade remains the top source-level owner surface for row414.
The scan does not justify a new fast path.

## Verdict

Keep `object_lifecycle_facade` as the source-level owner surface for this row.
Do not reopen any helper or substrate lane from this scan.
