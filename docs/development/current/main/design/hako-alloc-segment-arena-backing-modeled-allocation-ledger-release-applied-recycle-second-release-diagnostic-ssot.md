---
Status: SSOT
Decision: accepted
Date: 2026-05-20
Scope: MIMAP-296A segment arena backing modeled allocation-ledger release-applied recycle second-release diagnostic.
---

# Hako Alloc Segment Arena Backing Modeled Allocation-Ledger Release-Applied Recycle Second-Release Diagnostic

## Decision

MIMAP-296A adds an observer-only scalar/model diagnostic for the boundary after
MIMAP-292A release-applied recycle inventory.

The row observes a modeled release-applied recycle report and inventory and
publishes that a second release attempt after the modeled recycle would be
rejected as duplicate/stale. It must not create another release-applied recycle
row or open real arena backing release, pointer lookup, lifecycle generation,
segment-map mutation, OSVM/page-source, atomics, worker, provider, or backend
matcher seams.

## Owner

```text
lang/src/hako_alloc/memory/segment_arena_backing_modeled_allocation_ledger_release_applied_recycle_second_release_diagnostic_box.hako
```

## Input Contract

Accepted input:

```text
HakoAllocSegmentArenaBackingModeledAllocationLedgerReleaseAppliedRecycleInventory
  inventory_count > 0
  hasReleaseAppliedRecycleToken(report.release_applied_recycle_token) == 1

HakoAllocSegmentArenaBackingModeledAllocationLedgerReleaseAppliedRecycleReport
  accepted == 1
  release_applied_recycle_present == 1
  modeled_release_applied_recycle_present == 1
  release_applied_recycle_token > 0
```

Accepted output:

```text
observed == 1
second_release_rejected == 1
second_release_reason == 4
existing_index == report.row_index
```

## Reason Codes

| Code | Meaning |
| ---: | --- |
| 0 | second-release diagnostic observed |
| 1 | missing release-applied recycle inventory |
| 2 | missing/rejected release-applied recycle report |
| 3 | missing release-applied recycle token or token not in inventory |

`second_release_reason == 4` mirrors the existing modeled duplicate-token
reason used by the release-applied recycle inventory. It is reported as a
diagnostic fact only; this owner does not mutate the inventory duplicate counter.

## Stop Lines

- No new release-applied recycle rows.
- No source release/recycle key migration.
- No lifecycle generation/token introduction.
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

## Validation

Daily validation is L2:

```text
bash tools/checks/k2_wide_hako_alloc_segment_arena_backing_modeled_allocation_ledger_release_applied_recycle_second_release_diagnostic_guard.sh --level L2
bash tools/checks/run_proof_app.sh --only MIMAP-296A
```

L3 EXE evidence is deferred to a future second-release diagnostic closeout pack
unless this row introduces a new backend route shape.
