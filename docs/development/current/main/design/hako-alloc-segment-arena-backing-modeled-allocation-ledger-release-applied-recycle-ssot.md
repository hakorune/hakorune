---
Status: SSOT
Decision: accepted
Date: 2026-05-20
Scope: MIMAP-292A segment arena backing modeled allocation-ledger release-applied recycle inventory.
---

# Hako Alloc Segment Arena Backing Modeled Allocation-Ledger Release-Applied Recycle

## Decision

MIMAP-292A adds a scalar/model inventory row that consumes an accepted segment
arena backing allocation-ledger release-apply report and records a modeled
release-applied recycle entry.

The recycle row is a model fact only. It does not recycle real arena backing,
mutate a segment-map, call OSVM/page-source, execute atomic bitmap operations,
or open pointer residence.

## Owner

```text
lang/src/hako_alloc/memory/segment_arena_backing_modeled_allocation_ledger_release_applied_recycle_box.hako
```

## Input Contract

Accepted input:

```text
HakoAllocSegmentArenaBackingModeledAllocationLedgerReleaseApplyReport
  accepted == 1
  release_apply_present == 1
  modeled_release_apply_present == 1
  closed_substrate_blocker_count == 0
```

The caller supplies:

```text
release_applied_recycle_token
```

## Reason Codes

| Code | Meaning |
| ---: | --- |
| 0 | modeled release-applied recycle accepted |
| 1 | release-apply report missing |
| 2 | release-apply report rejected |
| 3 | invalid release-applied recycle token |
| 4 | duplicate release-applied recycle token |
| 5 | closed substrate requirement present |

## Stop Lines

- No real raw pointer residence.
- No pointer-derived lookup or dereference.
- No real arena backing allocation, release, or recycle.
- No real segment-map mutation.
- No real segment allocation/free execution.
- No atomic bitmap execution.
- No OSVM/page-source execution.
- No TLS, worker-local, worker scheduling, or source-level concurrency.
- No provider activation, host allocator replacement, hooks, or
  `#[global_allocator]`.
- No cross-function `Result` direct ABI or runtime sum materialization.
- No backend `.inc` matcher by app, box, owner, or row name.
