# 293x-791 HAKO-ALLOC-REPORT-RECORD-004 Segment Arena Backing Source Accounting Diagnostic ReportFields Pilot

Status: selected current
Date: 2026-05-19

## Decision

Apply the first segment-arena-backing report-record cleanup to the MIMAP-265A
source accounting diagnostic report construction.

## Owner

```text
lang/src/hako_alloc/memory/segment_arena_backing_modeled_source_accounting_diagnostic_box.hako
tools/checks/k2_wide_hako_alloc_segment_arena_backing_modeled_source_accounting_diagnostics_guard.sh
apps/hako-alloc-segment-arena-backing-modeled-source-accounting-diagnostics-proof/main.hako
```

## Scope

- Add an owner-local
  `HakoAllocSegmentArenaBackingModeledSourceAccountingDiagnosticReportFields`
  record.
- Build that record inside `makeReport(...)` and copy fields into the existing
  `HakoAllocSegmentArenaBackingModeledSourceAccountingDiagnosticReport` box.
- Preserve all proof output and MIR-visible typed report fields.
- Extend the existing MIMAP-265A guard to require the local record payload.

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
bash tools/checks/k2_wide_hako_alloc_segment_arena_backing_modeled_source_accounting_diagnostics_guard.sh
bash tools/checks/current_state_pointer_guard.sh
git diff --check
```
