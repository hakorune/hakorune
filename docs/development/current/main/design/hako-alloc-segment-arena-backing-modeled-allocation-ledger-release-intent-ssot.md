---
Status: SSOT
Decision: accepted
Date: 2026-05-20
Scope: MIMAP-284A segment arena backing modeled allocation-ledger release intent inventory.
---

# Hako Alloc Segment Arena Backing Modeled Allocation-Ledger Release Intent

## Decision

MIMAP-284A adds a scalar/model inventory row that consumes an accepted segment
arena backing allocation-ledger release-candidate report and records a modeled
release-intent entry.

The release intent is a model fact only. It does not release real arena
backing, mutate a segment-map, call OSVM/page-source, or open pointer
residence.

## Owner

```text
lang/src/hako_alloc/memory/segment_arena_backing_modeled_allocation_ledger_release_intent_box.hako
```

## Input Contract

Accepted input:

```text
HakoAllocSegmentArenaBackingModeledAllocationLedgerReleaseCandidateReport
  accepted == 1
  release_candidate_present == 1
  modeled_release_candidate_present == 1
  closed_substrate_blocker_count == 0
```

The caller supplies:

```text
release_intent_token
```

## Reason Codes

| Code | Meaning |
| ---: | --- |
| 0 | modeled release intent accepted |
| 1 | release-candidate report missing |
| 2 | release-candidate report rejected |
| 3 | invalid release intent token |
| 4 | duplicate release intent token |
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
