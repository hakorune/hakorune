# 293x-883 HAKO-ALLOC-REPORT-RECORD-028 Local-Free Integration ReportFields Helper Scalarization Closeout

Status: selected current
Date: 2026-05-20

## Decision

Close out the local-free integration `ReportFields` helper-argument
scalarization owner before selecting another owner.

Covered owner:

```text
HakoAllocSegmentAllocationModeledLocalFreeIntegrationReportFields
```

SSOT:

```text
docs/development/current/main/design/record-local-scalarization-ssot.md
```

## Scope

- Confirm the owner keeps its `ReportFields` helper argument builder-local.
- Confirm all candidate reject, apply-plan reject, page-apply reject, and
  success construction paths use the same-owner helper.
- Confirm the returned value remains the existing ordinary local-free
  integration report box.
- Confirm no additional `ReportFields` owner migrated in the pilot row.

## Stop Lines

- No scheduler `ReportFields` migration in this row.
- No broad migration across all report `ReportFields` owners.
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
bash tools/checks/k2_wide_hako_alloc_segment_allocation_modeled_local_free_integration_guard.sh
bash tools/checks/current_state_pointer_guard.sh
git diff --check
```

## Completion Criteria

- The closeout evidence proves the local-free integration helper-scalarized
  `ReportFields` owner.
- The target guard proves the helper-backed copy path and keeps MIMAP-119A
  behavior unchanged.
- The closeout selects the next narrow row only after evidence is recorded.
