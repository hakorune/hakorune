# 293x-324 PACKED-002 source PackedArray auto-use pilot

Status: Complete
Date: 2026-05-14

## Decision

Connect explicit source `PackedArray<Record>` field declarations to the existing
C209 non-escaping packed ArrayBox pilot metadata.

This is metadata-only. It does not enable backend lowering, visible record
materialization, hako_alloc migration, or ordinary ArrayBox fallback.

## Scope

- Add MIR metadata row `SourcePackedArrayAutoUsePilotPlan`.
- Add owner `src/mir/source_packed_array_autouse_pilot.rs`.
- Refresh the row from semantic refresh, MIR builder lifecycle, and JSON v0
  bridge lowering.
- Emit `source_packed_array_autouse_pilot_plans` through MIR JSON.
- Preserve stop lines: no backend lowering, no public materialization, no boxed
  fallback.

## Guard

- `tools/checks/k2_wide_source_packed_array_autouse_pilot_guard.sh`

## Validation

- `bash tools/checks/k2_wide_source_packed_array_autouse_pilot_guard.sh`
