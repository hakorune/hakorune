---
Status: Landed
Date: 2026-05-24
Scope: refresh the mimalloc comparison vertical slice after the page-heap
  non-id exact `usize` closeout.
Blocker: MIMALLOC-COMPARISON-VSLICE-PAGE-HEAP-USIZE-REFRESH-001
Related:
  - docs/development/current/main/phases/phase-294x/294x-267-HAKO-ALLOC-USIZE-NEXT-FIELD-GROUP-SELECTION.md
  - docs/development/current/main/phases/phase-294x/294x-266-HAKO-ALLOC-USIZE-PAGE-HEAP-NON-ID-CLOSEOUT.md
  - docs/development/current/main/phases/phase-294x/294x-228-MIMALLOC-COMPARISON-VSLICE-REFRESH.md
  - tools/checks/k2_wide_hako_alloc_usize_page_heap_non_id_closeout_guard.sh
  - tools/checks/k2_wide_hako_alloc_mimalloc_comparison_vertical_slice_closeout_guard.sh
---

# 294x-268 Mimalloc Comparison Vslice Page Heap Usize Refresh

## Decision

Close:

```text
MIMALLOC-COMPARISON-VSLICE-PAGE-HEAP-USIZE-REFRESH-001
```

The comparison vertical-slice closeout remains stable after the page-heap
non-id exact `usize` closeout. The refreshed evidence composes:

- `294x-266` page-heap non-id exact `usize` storage closeout;
- existing V2/V3/V4/V5 mimalloc comparison vertical-slice closeout.

## Evidence

The stable V5 schema remains:

```text
schema=vertical-slice-v1
hako_slices=1,1,1
hako_requested=48,216,4194321,4194585
hako_evidence=4194433,7,4,6,6,0
hako_details=4,16,2
c_mimalloc=1,1,1,1,64,64,33254,4096,4096,0,1
schema_bridge=1,1,0,4194585,33254
closed=0,0,0,0,0,0,0,0
summary=ok
```

## Stop Line

This refresh does not:

- migrate any additional exact `usize` field;
- migrate page/handle ids, indexes, sentinels, or pointer-like payloads;
- add a new `.hako` owner or comparison schema;
- run repeated benchmark packs;
- enable provider package / DLL generation, provider activation, host
  allocator replacement, hooks, backend matchers, worker/TLS, atomics,
  remote-free stress, abandoned heap stress, or `#[global_allocator]`;
- claim native allocator replacement or performance parity.

## Next Row

Return to selection from:

```text
HAKO-ALLOC-USIZE-FIELD-GROUP-NEXT-SELECTION-011
```

The next selection should choose a comparison-needed non-id field group only if
the comparison workload consumes it. Otherwise, park field migration and move
to a comparison evidence/presentation row.

## Verification

```bash
bash tools/checks/k2_wide_hako_alloc_mimalloc_comparison_page_heap_usize_refresh_guard.sh
git diff --check
bash tools/checks/current_state_pointer_guard.sh
```
