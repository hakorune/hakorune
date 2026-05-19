# 293x-891 MIMAP-288A Segment Arena Backing Modeled Allocation-Ledger Release Apply Inventory

Status: landed
Date: 2026-05-20

## Decision

Add a scalar/model inventory owner that consumes an accepted modeled
allocation-ledger release-intent report and records a modeled release-apply
entry.

The release apply row is a model fact only. It must not release real arena
backing, mutate segment-map state, execute atomic bitmap operations, call
OSVM/page-source, or open raw pointer residence.

## Scope

- Add one owner:

```text
HakoAllocSegmentArenaBackingModeledAllocationLedgerReleaseApplyInventory
```

- Add one returned report box and one owner-local `ReportFields` record payload.
- Consume:

```text
HakoAllocSegmentArenaBackingModeledAllocationLedgerReleaseIntentReport
```

- Record a modeled release-apply token and mirror the source allocator model
  fields needed by later recycle/continuation rows.
- Keep validation L2 daily unless a new backend route shape appears.

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

## Required Evidence

```text
bash tools/checks/k2_wide_hako_alloc_segment_arena_backing_modeled_allocation_ledger_release_apply_guard.sh
bash tools/checks/current_state_pointer_guard.sh
git diff --check
```

## Implementation

- Added
  `HakoAllocSegmentArenaBackingModeledAllocationLedgerReleaseApplyInventory`
  as a scalar/model owner.
- Added
  `HakoAllocSegmentArenaBackingModeledAllocationLedgerReleaseApplyReportFields`
  as the owner-local record payload for report construction.
- Added the MIMAP-288A proof app, proof manifest row, module export, memory
  README entry, design SSOT, and guard index entry.
- Kept L3/L4 EXE evidence deferred to the future release-apply closeout pack.

## Evidence

```text
bash tools/checks/k2_wide_hako_alloc_segment_arena_backing_modeled_allocation_ledger_release_apply_guard.sh --level L2
```

## Selected Next Row

`MIMAP-289A`:

```text
segment arena backing modeled allocation-ledger release apply diagnostics
```

The next row should observe MIMAP-288A release-apply counters and last apply
facts without recording new release-apply rows or opening any real allocator
execution seam.
