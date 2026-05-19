---
Status: SSOT
Decision: accepted
Date: 2026-05-19
Scope: MIMAP-277A segment arena backing modeled allocation ledger diagnostics.
---

# Hako Alloc Segment Arena Backing Modeled Allocation Ledger Diagnostics

## Decision

MIMAP-277A adds an observer-only scalar diagnostic row for MIMAP-276A modeled
allocation ledger inventory facts.

The diagnostic owner may summarize counters and last-ledger facts. It must not
record new allocation-ledger rows or open real allocator execution.

## Owner

```text
lang/src/hako_alloc/memory/segment_arena_backing_modeled_allocation_ledger_diagnostic_box.hako
```

## Input Contract

Inputs:

```text
HakoAllocSegmentArenaBackingModeledAllocationLedgerInventory
HakoAllocSegmentArenaBackingModeledAllocationLedgerReport
```

Accepted observation:

```text
inventory_count > 0
last_report.allocation_ledger_present == 1
last_report.modeled_allocation_ledger_present == 1
```

## Reason Codes

| Code | Meaning |
| ---: | --- |
| 0 | diagnostic observation accepted |
| 1 | allocation-ledger inventory/report missing |

The observed report also mirrors the last MIMAP-276A report reason so callers
can distinguish missing apply, rejected apply, invalid ledger token, duplicate
ledger token, and closed substrate categories.

## Stop Lines

- No new allocation-ledger rows.
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
