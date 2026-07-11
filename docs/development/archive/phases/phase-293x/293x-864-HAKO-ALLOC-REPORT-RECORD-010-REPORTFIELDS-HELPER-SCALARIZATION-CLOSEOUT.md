# 293x-864 HAKO-ALLOC-REPORT-RECORD-010 ReportFields Helper Scalarization Closeout

Status: landed
Date: 2026-05-20

## Decision

Close out the first two allocator `ReportFields` helper-argument
scalarization owners before selecting another report owner.

Covered owners:

```text
HakoAllocSegmentAllocationModeledLocalFreeReuseLedgerReleaseApply
HakoAllocSegmentArenaBackingModeledAllocationLedgerReleaseCandidateInventory
```

## Scope

- Keep the closeout focused on evidence and guard coverage.
- Confirm both migrated owners use local `ReportFields` records as
  builder-local helper arguments only.
- Confirm the compiler sidecar supports `ReportFields` values that cross PHI
  joins before the same-owner helper call.
- Keep returned report values as ordinary report boxes.
- Keep direct local record field-read scalarization green.

## Stop Lines

- No broad migration across all `ReportFields` owners.
- No record return values.
- No record storage in box fields, ArrayBox, MapBox, or globals.
- No packed ArrayBox residence or inline-record storage.
- No backend `.inc` owner-name matchers.
- No cross-function `Result` direct ABI or runtime sum materialization.
- No real raw pointer residence, real segment-map mutation, arena backing
  execution, atomic bitmap execution, OSVM/page-source execution, provider
  activation, hooks, host allocator replacement, or `#[global_allocator]`.

## Required Evidence

```text
cargo build --release --bin hakorune
bash tools/checks/k2_wide_allocator_record_construction_read_guard.sh
bash tools/checks/k2_wide_hako_alloc_segment_allocation_modeled_local_free_reuse_ledger_release_apply_guard.sh
bash tools/checks/k2_wide_hako_alloc_segment_arena_backing_modeled_allocation_ledger_release_candidate_guard.sh
bash tools/checks/current_state_pointer_guard.sh
git diff --check
```

## Completion Criteria

- The closeout evidence proves both helper-scalarized `ReportFields` owners.
- The release-candidate owner keeps `ReportFields` unmaterialized even after a
  PHI join before the helper call.
- The closeout selects the next narrow row only after the evidence is recorded.

## Progress

- Confirmed both migrated allocator `ReportFields` helper-argument
  scalarization owners remain green.
- Confirmed the allocation-ledger release-candidate `ReportFields` record stays
  builder-local across a PHI join before the same-owner helper call.
- Kept the closeout as evidence-only; no new report owner was migrated in this
  row.

## Evidence

```text
cargo build --release --bin hakorune
bash tools/checks/k2_wide_allocator_record_construction_read_guard.sh
bash tools/checks/k2_wide_hako_alloc_segment_allocation_modeled_local_free_reuse_ledger_release_apply_guard.sh
bash tools/checks/k2_wide_hako_alloc_segment_arena_backing_modeled_allocation_ledger_release_candidate_guard.sh
bash tools/checks/current_state_pointer_guard.sh
git diff --check
```

## Next

Select `HAKO-ALLOC-REPORT-RECORD-011` to inventory the remaining scalar-only
report-box candidates and choose the next single owner, rather than broadening
the helper pattern implicitly.
