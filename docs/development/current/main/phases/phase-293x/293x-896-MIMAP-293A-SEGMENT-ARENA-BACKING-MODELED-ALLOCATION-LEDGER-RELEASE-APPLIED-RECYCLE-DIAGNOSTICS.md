# 293x-896 MIMAP-293A Segment Arena Backing Modeled Allocation-Ledger Release-Applied Recycle Diagnostics

Status: landed
Date: 2026-05-20

## Decision

Add an observer-only diagnostics row for the MIMAP-292A modeled
release-applied recycle inventory.

The diagnostics owner should read already-recorded scalar counters and last
facts from the MIMAP-292A inventory and summarize the acceptance / rejection
surface. It must not create additional release-applied recycle rows.

## Scope

- Add one diagnostics owner for:

```text
HakoAllocSegmentArenaBackingModeledAllocationLedgerReleaseAppliedRecycleInventory
```

- Add one proof app and L2 guard.
- Summarize:
  - inventory / accepted / rejected counts
  - missing / rejected apply / invalid token / duplicate / closed-substrate
    reject counts
  - last reason, segment, arena, and release-applied recycle token
- Keep validation L2 daily unless a new backend route shape appears.

## Stop Lines

- No new release-applied recycle row creation by the diagnostics owner.
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

## Required Evidence

```text
bash tools/checks/k2_wide_hako_alloc_segment_arena_backing_modeled_allocation_ledger_release_applied_recycle_diagnostics_guard.sh
bash tools/checks/current_state_pointer_guard.sh
git diff --check
```

## Implementation

- Added
  `segment_arena_backing_modeled_allocation_ledger_release_applied_recycle_diagnostic_box.hako`.
- Added the manifest-backed diagnostics proof app.
- Added the L2 diagnostics guard and module / memory README exports.
- Added the diagnostics SSOT:

```text
docs/development/current/main/design/hako-alloc-segment-arena-backing-modeled-allocation-ledger-release-applied-recycle-diagnostics-ssot.md
```

## Evidence

```text
bash tools/checks/k2_wide_hako_alloc_segment_arena_backing_modeled_allocation_ledger_release_applied_recycle_diagnostics_guard.sh --level L2
bash apps/hako-alloc-segment-arena-backing-modeled-allocation-ledger-release-applied-recycle-diagnostics-proof/test.sh --level L2
bash tools/checks/current_state_pointer_guard.sh
git diff --check
```

## Selected Next Row

MIMAP-294A:
segment arena backing modeled allocation-ledger release-applied recycle closeout
pack.
