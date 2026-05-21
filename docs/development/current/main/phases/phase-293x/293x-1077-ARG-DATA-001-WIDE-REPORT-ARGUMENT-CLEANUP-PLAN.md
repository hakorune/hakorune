# 293x-1077 ARG-DATA-001 Wide Report Argument Cleanup Plan

Status: landed
Date: 2026-05-21

## Purpose

Insert a short BoxShape sidecar before MIMAP-454A to reduce wide report and
long positional argument pressure.

## Scope

- Document the cleanup order for wide allocator reports.
- Prefer structure and owner-local context records before new language syntax.
- Keep `new Box { field: expr }` as the current canonical field-set contract.
- Park record defaults, same-name shorthand, spread, named args, and automatic
  record-to-box copy until later language rows.
- Select ARG-DATA-002 as the first implementation row.

## Design

SSOT:

```text
docs/development/current/main/design/hako-alloc-wide-report-argument-cleanup-ssot.md
```

## Stop Lines

- No new source syntax.
- No `...fields` / spread syntax.
- No named argument syntax.
- No record default value semantics.
- No automatic record-to-box copy semantics.
- No runtime record object materialization.
- No backend route additions.
- No allocator replacement / hook / backend matcher / `#[global_allocator]`.

## Validation

```bash
bash tools/checks/k2_wide_hako_alloc_wide_report_argument_cleanup_plan_guard.sh
```

## Completed

- Added the wide report argument cleanup SSOT.
- Parked syntax changes behind future language rows.
- Selected ARG-DATA-002 for the first context-record cleanup pilot.
