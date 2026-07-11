# 293x-1086 ARG-DATA-008 Readiness and Support Requirement Horizontal Batch

Status: completed
Date: 2026-05-21

## Purpose

Horizontally apply the allocator-comparison diagnostic ReportFields cleanup shape
to the execution readiness and support requirement matrix diagnostic owners.

This keeps the record ergonomics rollout narrow enough to validate per-owner,
while proving the pattern extends to the remaining execution prerequisite
matrix diagnostics.

## Scope

Target owners:

- `lang/src/hako_alloc/memory/segment_arena_backing_modeled_allocation_ledger_release_recycle_execution_readiness_matrix_diagnostic_box.hako`
- `lang/src/hako_alloc/memory/segment_arena_backing_modeled_allocation_ledger_release_recycle_execution_support_requirement_matrix_diagnostic_box.hako`

For each target:

- Add defaults to the owner-local `DiagnosticReportFields` record.
- Build report fields through `Fields {}` plus tracked-record `with`.
- Use same-name shorthand where local variable names match report field names.
- Keep the returned value as the existing ordinary diagnostic report box.
- Keep record-to-box copy in the existing explicit same-owner helper.
- Update the focused guards so closed-seam evidence can be either an explicit
  initializer value or the owner-local ReportFields scalar default.

## Stop Lines

- No `...fields` spread.
- No automatic record-to-box copy.
- No ordinary-box `with` copy/update.
- No named function arguments.
- No `::default()` surface.
- No runtime record object.
- No record return ABI.
- No backend record route.
- No readiness / support execution.
- No performance or memory-use conclusion.
- No process allocator replacement, hook install, backend matcher, global
  allocator install, provider package generation, or thread execution.

## Validation

Required:

```bash
cargo test -q record_construction_ergonomics
bash tools/checks/k2_wide_hako_alloc_segment_arena_backing_modeled_allocation_ledger_release_recycle_execution_readiness_matrix_diagnostics_guard.sh --level L2
bash tools/checks/k2_wide_hako_alloc_segment_arena_backing_modeled_allocation_ledger_release_recycle_execution_support_requirement_matrix_diagnostics_guard.sh --level L2
bash tools/checks/current_state_pointer_guard.sh
git diff --check
```

Evidence:

- `cargo test -q record_construction_ergonomics`
- `bash tools/checks/k2_wide_hako_alloc_segment_arena_backing_modeled_allocation_ledger_release_recycle_execution_readiness_matrix_diagnostics_guard.sh --level L2`
- `bash tools/checks/k2_wide_hako_alloc_segment_arena_backing_modeled_allocation_ledger_release_recycle_execution_support_requirement_matrix_diagnostics_guard.sh --level L2`
- `bash tools/checks/current_state_pointer_guard.sh`
- `git diff --check`

## Resume

After ARG-DATA-008 lands, return to MIMAP-456A result ledger closeout.
