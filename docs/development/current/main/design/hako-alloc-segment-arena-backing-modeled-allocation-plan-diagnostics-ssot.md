---
Status: SSOT
Decision: accepted
Date: 2026-05-19
Scope: MIMAP-269A segment arena backing modeled allocation plan diagnostics.
---

# Hako Alloc Segment Arena Backing Modeled Allocation Plan Diagnostics

## Decision

MIMAP-269A adds observer-only diagnostics over the MIMAP-268A modeled
allocation-plan inventory.

The diagnostic owner may summarize counters and mirror the latest allocation
plan report. It must not record new allocation-plan rows or open real arena
backing behavior.

## Owner

```text
lang/src/hako_alloc/memory/segment_arena_backing_modeled_allocation_plan_diagnostic_box.hako
```

## Input Contract

The observer consumes:

```text
HakoAllocSegmentArenaBackingModeledAllocationPlanInventory
HakoAllocSegmentArenaBackingModeledAllocationPlanReport
```

Accepted observation requires:

```text
inventory_count > 0
last_report.allocation_plan_present == 1
last_report.modeled_allocation_plan_present == 1
```

## Reason Codes

| Code | Meaning |
| ---: | --- |
| 0 | allocation-plan diagnostics observed |
| 1 | allocation-plan inventory/report missing |

MIMAP-268A inventory reject reason counts remain the source of category
diagnostics:

```text
missing_accounting_reject_count
rejected_accounting_reject_count
invalid_plan_token_reject_count
invalid_plan_geometry_reject_count
closed_substrate_reject_count
```

## Stop Lines

- No new allocation-plan inventory rows.
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
