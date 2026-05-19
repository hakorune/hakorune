# 293x-867 HAKO-ALLOC-REPORT-RECORD-013 Release-Candidate Diagnostic ReportFields Helper Scalarization Closeout

Status: selected current
Date: 2026-05-20

## Decision

Close out the release-candidate diagnostic `ReportFields` helper-argument
scalarization row before selecting another owner.

Covered owner:

```text
HakoAllocSegmentArenaBackingModeledAllocationLedgerReleaseCandidateDiagnostic
```

## Scope

- Keep the closeout focused on evidence and guard coverage.
- Confirm the diagnostic owner uses its local `ReportFields` record as a
  builder-local helper argument only.
- Confirm the returned value remains the existing ordinary diagnostic report
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

- The closeout evidence proves the diagnostic helper-scalarized `ReportFields`
  owner.
- The diagnostics guard proves no runtime `NewBox` is emitted for the
  diagnostic `ReportFields` record.
- The closeout selects the next narrow row only after evidence is recorded.
