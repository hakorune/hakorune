# 293x-231 M191 Hako Alloc Stats Surface

Status: Complete

## Purpose

M191 returns to the hako_alloc algorithm lane after C194 and adds an
allocator-owned stats surface. The goal is observability for allocation,
release, reject, requested-byte, outstanding-block, and page-local counters
without changing allocation behavior.

## Decision

Decision: accepted.

Add a separate stats owner:

```text
lang/src/hako_alloc/memory/stats_box.hako
```

`HakoAllocProductionFacade.statsSnapshot()` delegates to this owner and returns
a `HakoAllocStatsSnapshot`. Existing facade counters remain authoritative for
allocation/release/reject counts, and existing page/heap observers remain
authoritative for requested bytes, outstanding blocks, free counts, reuse, and
peak usage.

M191 is stats-first. Mutable options, env toggles, purge/decommit, provider
activation, hooks, and process allocator replacement remain out of scope.

## Row Contract

M191 adds:

```text
HakoAllocStatsSurface.snapshot(...)
HakoAllocStatsSnapshot
HakoAllocProductionFacade.statsSnapshot()
```

Snapshot fields:

```text
allocation_count
release_count
reject_count
requested_bytes
outstanding_blocks
small_alloc_count
small_release_count
small_reuse_count
small_peak_used
small_free_count
medium_alloc_count
medium_release_count
medium_reuse_count
medium_peak_used
medium_free_count
```

## Stop Lines

- Do not change allocation/release/realloc behavior.
- Do not add mutable allocator options.
- Do not add environment variables or CLI toggles.
- Do not add purge/decommit, OSVM release, provider activation, hooks, or
  process allocator replacement.
- Do not move stats ownership into `page_heap_box.hako`.

## Acceptance

- `HakoAllocProductionFacade.statsSnapshot()` returns a stats object whose
  fields match existing facade and heap observers.
- Existing M46 production facade proof remains compatible.
- New stats proof passes VM and pure-first EXE.
- M191 guard stays local-run / index-listed and is not added to quick/dev gates.

## Verification

```bash
bash tools/checks/k2_wide_hako_alloc_stats_surface_guard.sh
```
