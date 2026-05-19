---
Status: SSOT
Decision: accepted
Date: 2026-05-20
Scope: MIMAP-288A segment arena backing modeled allocation-ledger release apply inventory.
---

# Hako Alloc Segment Arena Backing Modeled Allocation-Ledger Release Apply

## Decision

MIMAP-288A adds a scalar/model inventory row that consumes an accepted segment
arena backing allocation-ledger release-intent report and records a modeled
release-apply entry.

The release apply row is a model fact only. It does not release real arena
backing, mutate a segment-map, call OSVM/page-source, or open pointer
residence.

## Owner

```text
lang/src/hako_alloc/memory/segment_arena_backing_modeled_allocation_ledger_release_apply_box.hako
```

## Input Contract

Accepted input:

```text
HakoAllocSegmentArenaBackingModeledAllocationLedgerReleaseIntentReport
  accepted == 1
  release_intent_present == 1
  modeled_release_intent_present == 1
  closed_substrate_blocker_count == 0
```

The caller supplies:

```text
release_apply_token
```

## Reason Codes

| Code | Meaning |
| ---: | --- |
| 0 | modeled release apply accepted |
| 1 | release-intent report missing |
| 2 | release-intent report rejected |
| 3 | invalid release apply token |
| 4 | duplicate release apply token |
| 5 | closed substrate requirement present |

## Stop Lines

- No real raw pointer residence.
- No pointer-derived lookup or dereference.
- No real arena backing allocation or release.
- No real segment-map mutation.
- No real segment allocation/free execution.
- No atomic bitmap execution.
- No OSVM/page-source execution.
- No TLS, worker-local, worker scheduling, or source-level concurrency.
- No provider activation, host allocator replacement, hooks, or
  `#[global_allocator]`.
- No cross-function `Result` direct ABI or runtime sum materialization.
- No backend `.inc` matcher by app, box, owner, or row name.
