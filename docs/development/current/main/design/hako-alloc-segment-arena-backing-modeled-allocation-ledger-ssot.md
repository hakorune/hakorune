---
Status: SSOT
Decision: accepted
Date: 2026-05-19
Scope: MIMAP-276A segment arena backing modeled allocation ledger inventory.
---

# Hako Alloc Segment Arena Backing Modeled Allocation Ledger

## Decision

MIMAP-276A adds a scalar/model inventory row that consumes an accepted segment
arena backing allocation-apply report and records a modeled allocation ledger
entry.

The ledger is a model fact only. It does not allocate real arena backing or open
pointer residence.

## Owner

```text
lang/src/hako_alloc/memory/segment_arena_backing_modeled_allocation_ledger_box.hako
```

## Input Contract

Accepted input:

```text
HakoAllocSegmentArenaBackingModeledAllocationApplyReport
  accepted == 1
  allocation_apply_present == 1
  modeled_allocation_apply_present == 1
  closed_substrate_blocker_count == 0
```

The caller supplies:

```text
ledger_token
```

## Reason Codes

| Code | Meaning |
| ---: | --- |
| 0 | modeled allocation ledger accepted |
| 1 | allocation-apply report missing |
| 2 | allocation-apply report rejected |
| 3 | invalid ledger token |
| 4 | duplicate ledger token |
| 5 | closed substrate requirement present |

## Stop Lines

- No real raw pointer residence.
- No pointer-derived lookup or dereference.
- No real arena backing allocation.
- No real segment-map mutation.
- No real segment allocation/free execution.
- No atomic bitmap execution.
- No OSVM/page-source execution.
- No TLS, worker-local, worker scheduling, or source-level concurrency.
- No provider activation, host allocator replacement, hooks, or
  `#[global_allocator]`.
- No cross-function `Result` direct ABI or runtime sum materialization.
- No backend `.inc` matcher by app, box, owner, or row name.
