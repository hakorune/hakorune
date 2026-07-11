# 293x-873 HAKO-ALLOC-REPORT-RECORD-018 Allocation-Apply Diagnostic ReportFields Helper Scalarization Pilot

Status: landed
Date: 2026-05-20

## Decision

Apply record-local helper-argument scalarization to one allocator diagnostic
`ReportFields` owner:

```text
HakoAllocSegmentArenaBackingModeledAllocationApplyDiagnosticReportFields
```

SSOT:

```text
docs/development/current/main/design/record-local-scalarization-ssot.md
```

## Scope

- Add one same-owner helper that accepts the local `ReportFields` record and
  builds the existing ordinary report box.
- Keep `makeReport(...)` responsible for computing scalar fields and updating
  owner counters / last-state before it calls the helper.
- Keep the record carrier builder-local only.
- Keep validation at L2 unless the guard requires a stronger profile.

## Stop Lines

- No additional `ReportFields` owner migration in this row.
- No broad conversion from report boxes to records.
- No record return values.
- No runtime record object, `NewBox`, `typed_object_plan`, packed storage, or
  backend route for the record carrier.
- No broadened helper body profile.
- No cross-function record-local ABI.
- No cross-function `Result` direct ABI or runtime sum materialization.
- No real raw pointer residence, real segment-map mutation, arena backing
  execution, atomic bitmap execution, OSVM/page-source execution, provider
  activation, hooks, host allocator replacement, or `#[global_allocator]`.

## Required Evidence

```text
bash tools/checks/k2_wide_allocator_record_construction_read_guard.sh
bash tools/checks/k2_wide_hako_alloc_segment_arena_backing_modeled_allocation_apply_diagnostics_guard.sh
bash tools/checks/current_state_pointer_guard.sh
git diff --check
```

## Completion Criteria

- The target owner builds the ordinary diagnostic report through a same-owner
  helper that accepts the local `ReportFields` record.
- The target guard stays green and continues proving no runtime `NewBox` for
  the `ReportFields` carrier.
- No other owner is migrated.

## Progress

- Added `makeAllocationApplyDiagnosticReport(fields)` as the same-owner helper
  for the allocation-apply diagnostic `ReportFields` owner.
- Kept `makeReport(...)` responsible for computing scalar fields and updating
  `me.last_reason` before calling the helper.
- Migrated no other owner.

## Evidence

```text
bash tools/checks/k2_wide_allocator_record_construction_read_guard.sh
bash tools/checks/k2_wide_hako_alloc_segment_arena_backing_modeled_allocation_apply_diagnostics_guard.sh
```

## Next

Select `HAKO-ALLOC-REPORT-RECORD-019` to close out the allocation-apply
diagnostic ReportFields helper-scalarization owner before another owner is
selected.
