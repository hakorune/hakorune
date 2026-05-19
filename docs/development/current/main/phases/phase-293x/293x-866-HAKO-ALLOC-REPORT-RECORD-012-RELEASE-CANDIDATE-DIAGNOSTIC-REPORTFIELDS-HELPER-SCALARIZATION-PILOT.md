# 293x-866 HAKO-ALLOC-REPORT-RECORD-012 Release-Candidate Diagnostic ReportFields Helper Scalarization Pilot

Status: landed
Date: 2026-05-20

## Decision

Apply the landed `RECORD-VALUE-HELPER-001` same-owner helper-argument
scalarization pattern to exactly one additional allocator diagnostic report
owner:

```text
HakoAllocSegmentArenaBackingModeledAllocationLedgerReleaseCandidateDiagnostic
```

The existing owner-local record carrier is:

```text
HakoAllocSegmentArenaBackingModeledAllocationLedgerReleaseCandidateDiagnosticReportFields
```

## Scope

- Add one same-owner helper that accepts the existing diagnostic `ReportFields`
  record.
- Move the repeated diagnostic field-copy block from `makeReport(...)` into
  that helper.
- Keep `makeReport(...)` responsible for creating the `ReportFields` record,
  computing observed diagnostic bits, and updating owner-local last-state
  fields.
- Keep the returned value as the existing ordinary
  `HakoAllocSegmentArenaBackingModeledAllocationLedgerReleaseCandidateDiagnosticReport`
  box.
- Keep direct local record field-read scalarization green.

## Stop Lines

- No broad migration across all diagnostic `ReportFields` owners.
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
bash tools/checks/k2_wide_hako_alloc_segment_arena_backing_modeled_allocation_ledger_release_candidate_diagnostics_guard.sh
bash tools/checks/current_state_pointer_guard.sh
git diff --check
```

## Completion Criteria

- `makeReport(...)` delegates diagnostic report field copying through a
  same-owner helper that takes
  `HakoAllocSegmentArenaBackingModeledAllocationLedgerReleaseCandidateDiagnosticReportFields`.
- The diagnostics guard proves the helper shape.
- The MIR JSON contains no `NewBox` for the diagnostic `ReportFields` record
  type.
- Existing release-candidate diagnostic behavior and exact-`usize` field
  declarations stay green.

## Progress

- Added `makeReleaseCandidateDiagnosticReport(fields: ...DiagnosticReportFields)`.
- Kept `makeReport(...)` as the local diagnostic `ReportFields` construction
  owner and delegated repeated report field copying through the helper.
- Kept returned diagnostics as the existing ordinary diagnostic report box.
- Kept the diagnostic `ReportFields` carrier builder-local; no runtime record
  box is emitted.

## Evidence

```text
bash tools/checks/k2_wide_allocator_record_construction_read_guard.sh
bash tools/checks/k2_wide_hako_alloc_segment_arena_backing_modeled_allocation_ledger_release_candidate_diagnostics_guard.sh
bash tools/checks/current_state_pointer_guard.sh
git diff --check
```

## Next

Select `HAKO-ALLOC-REPORT-RECORD-013` to close out the release-candidate
diagnostic helper-scalarization row before another owner is selected.
