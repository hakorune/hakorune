# 293x-1083 ARG-DATA-005 Diagnostic ReportFields Horizontal Batch

Status: completed
Date: 2026-05-21

## Purpose

Horizontally apply the ARG-DATA-004 diagnostic ReportFields cleanup shape to a
small allocator-comparison batch.

This keeps the record ergonomics rollout narrow enough to validate per-owner,
while proving the pattern is not a one-off for the MIMAP-455A result-ledger
diagnostic owner.

## Scope

Target owners:

- `lang/src/hako_alloc/memory/allocator_comparison_baseline_diagnostic_box.hako`
- `lang/src/hako_alloc/memory/allocator_comparison_workload_matrix_diagnostic_box.hako`
- `lang/src/hako_alloc/memory/allocator_comparison_measurement_plan_diagnostic_box.hako`

For each target:

- Add defaults to the owner-local `DiagnosticReportFields` record.
- Build report fields through `Fields {}` plus tracked-record `with`.
- Use same-name shorthand where local variable names match report field names.
- Keep the returned value as the existing ordinary diagnostic report box.
- Keep record-to-box copy in the existing explicit same-owner helper.
- Update the three focused guards so closed-seam evidence can be either an
  explicit initializer value or the owner-local ReportFields scalar default.

## Stop Lines

- No `...fields` spread.
- No automatic record-to-box copy.
- No ordinary-box `with` copy/update.
- No named function arguments.
- No `::default()` surface.
- No runtime record object.
- No record return ABI.
- No backend record route.
- No benchmark execution.
- No performance or memory-use conclusion.
- No process allocator replacement, hook install, backend matcher, global
  allocator install, provider package generation, or thread execution.

## Validation

Required:

```bash
cargo test -q record_construction_ergonomics
bash tools/checks/k2_wide_hako_alloc_allocator_comparison_baseline_diagnostics_guard.sh --level L2
bash tools/checks/k2_wide_hako_alloc_allocator_comparison_workload_matrix_diagnostics_guard.sh --level L2
bash tools/checks/k2_wide_hako_alloc_allocator_comparison_measurement_plan_diagnostics_guard.sh --level L2
bash tools/checks/current_state_pointer_guard.sh
git diff --check
```

Evidence:

- `cargo test -q record_construction_ergonomics`
- `bash tools/checks/k2_wide_hako_alloc_allocator_comparison_baseline_diagnostics_guard.sh --level L2`
- `bash tools/checks/k2_wide_hako_alloc_allocator_comparison_workload_matrix_diagnostics_guard.sh --level L2`
- `bash tools/checks/k2_wide_hako_alloc_allocator_comparison_measurement_plan_diagnostics_guard.sh --level L2`
- `bash tools/checks/current_state_pointer_guard.sh`
- `git diff --check`

## Resume

After ARG-DATA-005 lands, return to MIMAP-456A result ledger closeout.
