---
Status: SSOT
Decision: accepted
Date: 2026-05-20
Scope: MIMAP-289A segment arena backing modeled allocation-ledger release apply diagnostics.
---

# Hako Alloc Segment Arena Backing Modeled Allocation-Ledger Release Apply Diagnostics

## Decision

MIMAP-289A adds observer-only diagnostics for the MIMAP-288A scalar/model
release-apply inventory.

The diagnostic row reads inventory counters and a caller-provided last
release-apply report. It must not record new release-apply rows or open real
arena backing release, segment-map mutation, OSVM/page-source, atomics, worker,
provider, or backend matcher seams.

## Owner

```text
lang/src/hako_alloc/memory/segment_arena_backing_modeled_allocation_ledger_release_apply_diagnostic_box.hako
```

## Input Contract

Accepted input:

```text
HakoAllocSegmentArenaBackingModeledAllocationLedgerReleaseApplyInventory
  inventory_count > 0

HakoAllocSegmentArenaBackingModeledAllocationLedgerReleaseApplyReport
  release_apply_present == 1
  modeled_release_apply_present == 1
```

## Reason Codes

| Code | Meaning |
| ---: | --- |
| 0 | modeled release apply diagnostics observed |
| 1 | missing release-apply inventory/report |

## Stop Lines

- No new release-apply rows.
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
