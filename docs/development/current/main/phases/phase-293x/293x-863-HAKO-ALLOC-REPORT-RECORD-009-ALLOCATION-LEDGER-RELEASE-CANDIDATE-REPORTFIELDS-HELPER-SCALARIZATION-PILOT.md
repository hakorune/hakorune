# 293x-863 HAKO-ALLOC-REPORT-RECORD-009 Allocation-Ledger Release-Candidate ReportFields Helper Scalarization Pilot

Status: selected current
Date: 2026-05-20

## Decision

Apply the landed `RECORD-VALUE-HELPER-001` same-owner helper-argument
scalarization pattern to exactly one additional allocator report owner:

```text
HakoAllocSegmentArenaBackingModeledAllocationLedgerReleaseCandidateInventory
```

The existing owner-local record carrier is:

```text
HakoAllocSegmentArenaBackingModeledAllocationLedgerReleaseCandidateReportFields
```

## Scope

- Add one same-owner helper that accepts the existing `ReportFields` record.
- Move the repeated field-copy block from `makeReport(...)` into that helper.
- Keep `makeReport(...)` responsible for creating the `ReportFields` record and
  updating owner-local last-state fields.
- Keep the returned value as the existing ordinary
  `HakoAllocSegmentArenaBackingModeledAllocationLedgerReleaseCandidateReport`
  box.
- Keep direct local record field-read scalarization green.

## Stop Lines

- No broad migration across all `ReportFields` owners.
- No record return values.
- No record storage in box fields, ArrayBox, MapBox, or globals.
- No backend `.inc` owner-name matchers.
- No packed ArrayBox residence or inline-record storage.
- No cross-function `Result` direct ABI or runtime sum materialization.
- No real raw pointer residence, real segment-map mutation, arena backing
  execution, atomic bitmap execution, OSVM/page-source execution, provider
  activation, hooks, host allocator replacement, or `#[global_allocator]`.

## Required Evidence

```text
bash tools/checks/k2_wide_allocator_record_construction_read_guard.sh
bash tools/checks/k2_wide_hako_alloc_segment_arena_backing_modeled_allocation_ledger_release_candidate_guard.sh
bash tools/checks/current_state_pointer_guard.sh
git diff --check
```

## Completion Criteria

- `makeReport(...)` delegates report field copying through a same-owner helper
  that takes `HakoAllocSegmentArenaBackingModeledAllocationLedgerReleaseCandidateReportFields`.
- The release-candidate guard proves the helper shape.
- The MIR JSON contains no `NewBox` for the `ReportFields` record type.
- Existing release-candidate behavior and exact-`usize` field declarations stay
  green.
