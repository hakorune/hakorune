# 293x-1082 ARG-DATA-004 Diagnostic Report Fields Cleanup Pilot

Status: completed
Date: 2026-05-21

## Purpose

Use the record construction ergonomics from ARG-DATA-003 on one wide
diagnostic report owner before continuing allocator comparison closeout work.

The goal is to reduce wide diagnostic field construction noise without opening
automatic record-to-box copy, ordinary-box `with`, spread copy, or new runtime
record materialization semantics.

## Scope

- Pilot owner:
  `lang/src/hako_alloc/memory/allocator_comparison_c_mimalloc_result_ledger_diagnostic_box.hako`
- Add defaults to the owner-local `DiagnosticReportFields` record.
- Build the report fields through `Fields {}` plus tracked-record `with`.
- Use same-name shorthand where local variable names already match report field
  names.
- Keep the returned value as the existing ordinary diagnostic report box.
- Keep the record-to-box copy in one explicit same-owner helper.

## Stop Lines

- No `...fields` spread.
- No automatic record-to-box copy.
- No ordinary-box `with` copy/update.
- No named function arguments.
- No `::default()` surface.
- No runtime record object.
- No record return ABI.
- No backend record route.
- No benchmark rerun.
- No performance or memory-use conclusion.
- No process allocator replacement, hook install, backend matcher, global
  allocator install, provider package generation, or thread execution.

## Validation

Required:

```bash
cargo test -q record_construction_ergonomics
bash tools/checks/k2_wide_hako_alloc_allocator_comparison_c_mimalloc_result_ledger_diagnostics_guard.sh --level L2
bash tools/checks/current_state_pointer_guard.sh
git diff --check
```

Evidence:

- `cargo test -q record_construction_ergonomics`
- `bash tools/checks/k2_wide_hako_alloc_allocator_comparison_c_mimalloc_result_ledger_diagnostics_guard.sh --level L2`
- `bash tools/checks/current_state_pointer_guard.sh`
- `git diff --check`

## Resume

After ARG-DATA-004 lands, return to MIMAP-456A result ledger closeout.
