---
Status: SSOT
Decision: accepted
Date: 2026-05-19
Scope: MIMAP-272A segment arena backing modeled allocation apply inventory.
---

# Hako Alloc Segment Arena Backing Modeled Allocation Apply

## Decision

MIMAP-272A adds a scalar/model inventory row that consumes an accepted segment
arena backing allocation-plan report and records a modeled allocation apply
fact.

The apply is a ledger fact only. It does not allocate real arena backing or open
pointer residence.

## Owner

```text
lang/src/hako_alloc/memory/segment_arena_backing_modeled_allocation_apply_box.hako
```

## Input Contract

Accepted input:

```text
HakoAllocSegmentArenaBackingModeledAllocationPlanReport
  accepted == 1
  allocation_plan_present == 1
  modeled_allocation_plan_present == 1
  closed_substrate_blocker_count == 0
```

The caller supplies:

```text
apply_token
applied_backing_bytes
applied_committed_bytes
```

## Reason Codes

| Code | Meaning |
| ---: | --- |
| 0 | modeled allocation apply accepted |
| 1 | allocation-plan report missing |
| 2 | allocation-plan report rejected |
| 3 | invalid apply token |
| 4 | invalid apply geometry |
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
