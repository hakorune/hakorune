# 293x-895 MIMAP-292A Segment Arena Backing Modeled Allocation-Ledger Release-Applied Recycle Inventory

Status: landed
Date: 2026-05-20

## Decision

Add a scalar/model inventory owner that consumes an accepted modeled
allocation-ledger release-apply report and records a modeled release-applied
recycle entry.

The recycle row is a model fact only. It must not recycle real arena backing,
mutate segment-map state, execute atomic bitmap operations, call
OSVM/page-source, or open raw pointer residence.

## Scope

- Add one owner:

```text
HakoAllocSegmentArenaBackingModeledAllocationLedgerReleaseAppliedRecycleInventory
```

- Add one returned report box and one owner-local `ReportFields` record payload.
- Consume:

```text
HakoAllocSegmentArenaBackingModeledAllocationLedgerReleaseApplyReport
```

- Record a modeled release-applied recycle token and mirror the source allocator
  model fields needed by later continuation / second-release diagnostics rows.
- Keep validation L2 daily unless a new backend route shape appears.

## Input Contract

Accepted input:

```text
HakoAllocSegmentArenaBackingModeledAllocationLedgerReleaseApplyReport
  accepted == 1
  release_apply_present == 1
  modeled_release_apply_present == 1
  closed_substrate_blocker_count == 0
```

The caller supplies:

```text
release_applied_recycle_token
```

## Reason Codes

| Code | Meaning |
| ---: | --- |
| 0 | modeled release-applied recycle accepted |
| 1 | release-apply report missing |
| 2 | release-apply report rejected |
| 3 | invalid release-applied recycle token |
| 4 | duplicate release-applied recycle token |
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
bash tools/checks/k2_wide_hako_alloc_segment_arena_backing_modeled_allocation_ledger_release_applied_recycle_guard.sh
bash tools/checks/current_state_pointer_guard.sh
git diff --check
```

## Implementation

- Added
  `segment_arena_backing_modeled_allocation_ledger_release_applied_recycle_box.hako`.
- Added the manifest-backed proof app
  `apps/hako-alloc-segment-arena-backing-modeled-allocation-ledger-release-applied-recycle-proof`.
- Added the L2 guard and module / memory README exports.
- Added the release-applied recycle SSOT:

```text
docs/development/current/main/design/hako-alloc-segment-arena-backing-modeled-allocation-ledger-release-applied-recycle-ssot.md
```

## Evidence

```text
bash tools/checks/k2_wide_hako_alloc_segment_arena_backing_modeled_allocation_ledger_release_applied_recycle_guard.sh --level L2
bash apps/hako-alloc-segment-arena-backing-modeled-allocation-ledger-release-applied-recycle-proof/test.sh --level L2
bash tools/checks/current_state_pointer_guard.sh
git diff --check
```

## Selected Next Row

MIMAP-293A:
segment arena backing modeled allocation-ledger release-applied recycle
diagnostics.
