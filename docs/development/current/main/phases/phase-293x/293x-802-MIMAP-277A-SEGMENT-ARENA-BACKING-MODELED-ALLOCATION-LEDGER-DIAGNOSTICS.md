# 293x-802 MIMAP-277A Segment Arena Backing Modeled Allocation Ledger Diagnostics

Status: landed
Date: 2026-05-19

## Decision

Add observer-only diagnostics for MIMAP-276A allocation-ledger inventory
counters and last-ledger facts without recording new ledger rows or opening
real allocator execution.

## Context

MIMAP-276A records a model-only allocation ledger row from accepted modeled
allocation-apply facts. The next row should expose scalar diagnostic summary
facts so the family can be closed out before any real arena backing allocation
or raw pointer residence opens.

## Scope

- Add a scalar diagnostic owner for MIMAP-276A allocation-ledger inventory facts.
- Publish inventory / accepted / reject counters.
- Publish missing/rejected apply, invalid ledger token, duplicate ledger token,
  and closed-substrate reject category facts.
- Publish last reason, last segment, last arena, and last ledger token.
- Keep this row L2 daily unless it introduces a new backend route shape.

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
- No backend `.inc` matcher by app or owner name.

## Required Evidence

```text
bash tools/checks/k2_wide_hako_alloc_segment_arena_backing_modeled_allocation_ledger_diagnostics_guard.sh
bash tools/checks/current_state_pointer_guard.sh
git diff --check
```

## Landed Scope

- Added `HakoAllocSegmentArenaBackingModeledAllocationLedgerDiagnostic`.
- Added a proof app that observes MIMAP-276A inventory counters and last-ledger
  facts without recording new allocation-ledger rows.
- Fixed reject category seen bits for missing apply, rejected apply, invalid
  ledger token, duplicate ledger token, and closed substrate.
- Kept L3 evidence deferred to a future closeout pack.

## Next Row

MIMAP-278A closes out the modeled allocation-ledger family with L3 evidence.
