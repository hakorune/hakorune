# 293x-807 HAKO-ALLOC-REPORT-RECORD-005 Allocation-Ledger Release Candidate ReportFields Pilot

Status: landed
Date: 2026-05-19

## Decision

Add an owner-local `ReportFields` record payload to the MIMAP-280A
allocation-ledger release-candidate report construction.

## Why Here

`HakoAllocSegmentArenaBackingModeledAllocationLedgerReleaseCandidateReport` is
an all-`i64` scalar fact carrier. It is semantically identity-free, but the
stable cross-function carrier is still the returned report `box`.

This row reduces report-construction debt now without opening full record
return/pass/store escape or backend record lowering.

## Owner

```text
lang/src/hako_alloc/memory/segment_arena_backing_modeled_allocation_ledger_release_candidate_box.hako
tools/checks/k2_wide_hako_alloc_segment_arena_backing_modeled_allocation_ledger_release_candidate_guard.sh
apps/hako-alloc-segment-arena-backing-modeled-allocation-ledger-release-candidate-proof/main.hako
```

## Scope

- Add
  `HakoAllocSegmentArenaBackingModeledAllocationLedgerReleaseCandidateReportFields`.
- Build that record inside `makeReport(...)` and copy fields into the existing
  `HakoAllocSegmentArenaBackingModeledAllocationLedgerReleaseCandidateReport`
  box.
- Preserve all proof output and MIR-visible typed report fields.
- Extend the existing MIMAP-280A guard to require the local record payload.

## Stop Lines

- No allocator behavior change.
- No broad segment arena backing report rewrite.
- No cross-function record return.
- No record pass/store escape.
- No packed/backend record lowering.
- No provider activation, host allocator replacement, hooks, or
  `#[global_allocator]`.
- No backend `.inc` matcher by app, box, or owner name.
- No silent fallback.

## Required Evidence

```text
bash tools/checks/k2_wide_hako_alloc_segment_arena_backing_modeled_allocation_ledger_release_candidate_guard.sh
bash tools/checks/current_state_pointer_guard.sh
git diff --check
```

## Landed Scope

- Added
  `HakoAllocSegmentArenaBackingModeledAllocationLedgerReleaseCandidateReportFields`.
- Rebuilt the release-candidate report construction path so `makeReport(...)`
  creates a local record payload and then copies it into the existing returned
  report box.
- Extended the MIMAP-280A guard to require the local record payload in source
  and MIR `record_decls`.
- Preserved proof output and the returned report box shape.

## Selected Next Row

```text
MIMAP-281A
  segment arena backing modeled allocation-ledger release candidate diagnostics
```
