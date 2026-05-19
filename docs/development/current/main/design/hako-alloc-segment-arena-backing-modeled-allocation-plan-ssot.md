---
Status: SSOT
Decision: accepted
Date: 2026-05-19
Scope: MIMAP-268A segment arena backing modeled allocation plan inventory.
---

# Hako Alloc Segment Arena Backing Modeled Allocation Plan

## Decision

MIMAP-268A adds a scalar/model inventory row that consumes an accepted segment
arena backing source-accounting report and records a modeled allocation plan.

The plan is a ledger fact only. It does not allocate real arena backing or open
pointer residence.

## Owner

```text
lang/src/hako_alloc/memory/segment_arena_backing_modeled_allocation_plan_box.hako
```

## Input Contract

Accepted input:

```text
HakoAllocSegmentArenaBackingModeledSourceAccountingReport
  accepted == 1
  source_accounting_present == 1
  modeled_source_accounting_present == 1
  closed_substrate_blocker_count == 0
```

The caller supplies:

```text
plan_token
planned_backing_bytes
planned_committed_bytes
```

## Reason Codes

| Code | Meaning |
| ---: | --- |
| 0 | modeled allocation plan accepted |
| 1 | source accounting report missing |
| 2 | source accounting report rejected |
| 3 | invalid plan token |
| 4 | invalid plan geometry |
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
