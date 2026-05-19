---
Status: SSOT
Decision: accepted
Date: 2026-05-19
Scope: MIMAP-280A segment arena backing modeled allocation-ledger release candidate inventory.
---

# Hako Alloc Segment Arena Backing Modeled Allocation-Ledger Release Candidate

## Decision

MIMAP-280A adds a scalar/model inventory row that consumes an accepted segment
arena backing allocation-ledger report and records a modeled release-candidate
entry.

The release candidate is a model fact only. It does not release real arena
backing, mutate a segment-map, or open pointer residence.

## Owner

```text
lang/src/hako_alloc/memory/segment_arena_backing_modeled_allocation_ledger_release_candidate_box.hako
```

## Input Contract

Accepted input:

```text
HakoAllocSegmentArenaBackingModeledAllocationLedgerReport
  accepted == 1
  allocation_ledger_present == 1
  modeled_allocation_ledger_present == 1
  closed_substrate_blocker_count == 0
```

The caller supplies:

```text
release_candidate_token
```

## Reason Codes

| Code | Meaning |
| ---: | --- |
| 0 | modeled release candidate accepted |
| 1 | allocation-ledger report missing |
| 2 | allocation-ledger report rejected |
| 3 | invalid release candidate token |
| 4 | duplicate release candidate token |
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
