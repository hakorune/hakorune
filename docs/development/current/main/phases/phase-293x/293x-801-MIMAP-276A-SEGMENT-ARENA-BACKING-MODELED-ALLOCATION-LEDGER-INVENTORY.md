# 293x-801 MIMAP-276A Segment Arena Backing Modeled Allocation Ledger Inventory

Status: landed
Date: 2026-05-19

## Decision

Consume an accepted MIMAP-272A allocation-apply report and record a model-only
modeled allocation ledger row before real arena backing allocation opens.

## Context

MIMAP-272A records model-only allocation-apply facts. MIMAP-273A observes those
facts and MIMAP-274A closed out the family. The next durable row should keep a
ledger of applied allocations in scalar/model space so future release/recycle
rows have a stable source.

## Scope

- Add a scalar/model allocation-ledger inventory owner.
- Consume accepted allocation-apply reports only.
- Publish ledger token, apply token, plan/source identity, applied backing
  bytes, applied committed bytes, and remaining source bytes.
- Reject missing/rejected apply reports, invalid ledger token, duplicate ledger
  token, and closed substrate requirements.
- Keep this row L2 daily unless it introduces a new backend route shape.

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
- No backend `.inc` matcher by app or owner name.

## Required Evidence

```text
bash tools/checks/k2_wide_hako_alloc_segment_arena_backing_modeled_allocation_ledger_guard.sh
bash tools/checks/current_state_pointer_guard.sh
git diff --check
```

## Landed Scope

- Added `HakoAllocSegmentArenaBackingModeledAllocationLedgerInventory`.
- Added a proof app that records an accepted model-only allocation ledger row
  from an accepted MIMAP-272A allocation-apply report.
- Fixed missing/rejected apply, invalid ledger token, duplicate ledger token,
  and closed-substrate rejects.
- Kept L3 evidence deferred to a future closeout pack.

## Next Row

MIMAP-277A observes MIMAP-276A allocation-ledger counters and last-ledger facts
without recording new ledger rows.
