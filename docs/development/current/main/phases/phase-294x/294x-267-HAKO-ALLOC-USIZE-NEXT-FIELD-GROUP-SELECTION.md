---
Status: Landed
Date: 2026-05-24
Scope: select the next row after the page-heap non-id exact `usize` closeout.
Blocker: HAKO-ALLOC-USIZE-FIELD-GROUP-NEXT-SELECTION-010
Related:
  - docs/development/current/main/phases/phase-294x/294x-266-HAKO-ALLOC-USIZE-PAGE-HEAP-NON-ID-CLOSEOUT.md
  - docs/development/current/main/phases/phase-294x/294x-228-MIMALLOC-COMPARISON-VSLICE-REFRESH.md
  - docs/development/current/main/phases/phase-294x/294x-90-usize-semantics-taskboard.md
---

# 294x-267 Hako Alloc Usize Next Field Group Selection

## Decision

Close:

```text
HAKO-ALLOC-USIZE-FIELD-GROUP-NEXT-SELECTION-010
```

Do not select another exact `usize` production field group yet.

Select a comparison refresh row instead:

```text
MIMALLOC-COMPARISON-VSLICE-PAGE-HEAP-USIZE-REFRESH-001
```

## Why

`294x-266` closed the legacy page-heap non-id exact `usize` slice:

- handle requested size;
- page block size;
- page capacity;
- free-stack top;
- page allocation/reuse/occupancy/requested-byte counters.

The remaining page-heap fields are identity/index seams:

- `HakoAllocHandle.page_id`;
- `HakoAllocHandle.block_id`;
- `HakoAllocPage.page_id`.

Those fields are not just non-negative counters. They cross identity,
stale-handle checks, result printing, and page/handle ownership. Migrating them
by momentum would blur the current comparison-slice boundary.

Before selecting any further field group, refresh the existing mimalloc
comparison vertical slice against the exact page-heap non-id closeout. If the
comparison proof remains stable, the next useful work should be selected from
comparison evidence needs, not from broad allocator field drainage.

## Stop Line

The refresh row must not:

- migrate page/handle ids, indexes, sentinels, or pointer-like payloads;
- widen `HakoAllocPageModel` or page-map entry storage;
- add new comparison report fields just to mirror existing values;
- reopen provider package / DLL generation, process allocator replacement,
  hooks, backend matchers, worker/TLS, atomics, remote-free stress, abandoned
  heap stress, or `#[global_allocator]`;
- claim native allocator replacement or performance parity.

## Next Row

Implement:

```text
MIMALLOC-COMPARISON-VSLICE-PAGE-HEAP-USIZE-REFRESH-001
```

Expected validation should compose the page-heap non-id exact `usize` closeout
guard with the existing mimalloc comparison vertical-slice closeout guard.

## Verification

```bash
git diff --check
bash tools/checks/current_state_pointer_guard.sh
```
