# 293x-233 M193 Purge Decommit Dry-Run Observer

Status: Complete

## Purpose

M193 connects the M192 purge/decommit policy inventory to OSVM-backed
page/backing-shaped observers. The row proves a future purge route can observe
page state and backing metadata and produce a candidate decision without
executing decommit, unreserve, or OSVM release.

## Decision

Decision: accepted.

Add:

```text
lang/src/hako_alloc/memory/purge_dry_run_box.hako
```

`HakoAllocPurgeDryRunObserver.observeHeapPage(heap, page_id)` reads the
existing page model and backing metadata, then delegates to
`HakoAllocPurgePolicyInventory.classifyLocalPage(...)`.

This is a dry-run observer only. It may update observer counters, but it must
not mutate heap/page state or call page-source APIs.

## Stop Lines

- Do not call `HakoAllocPageSourcePolicy`.
- Do not call OSVM decommit/unreserve/release.
- Do not mutate heap/page state from the observer.
- Do not change allocation, release, realloc, aligned, or huge behavior.
- Do not add provider activation, hooks, or process allocator replacement.
- Do not add env toggles or mutable allocator options.

## Acceptance

- Live OSVM-backed page observation rejects as live.
- Empty retired OSVM-backed page observation is classified as an eligible
  candidate with all execution booleans still false.
- Missing page/backing rejects through the M192 policy.
- VM and pure-first EXE proof output match the dry-run matrix.
- The guard confirms no page-source or `.inc` matcher leak.

## Verification

```bash
bash tools/checks/k2_wide_hako_alloc_purge_dry_run_guard.sh
```
