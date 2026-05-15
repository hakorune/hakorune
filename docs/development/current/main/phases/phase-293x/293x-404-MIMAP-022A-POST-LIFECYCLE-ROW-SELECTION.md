# 293x-404 MIMAP-022A Post-Lifecycle Row Selection

Status: ready
Date: 2026-05-15

## Decision

`MIMAP-022A` selects the next allocator behavior row after the page-source
allocation-miss fallback and lifecycle construction/reuse cleanup rows are
closed.

This is a planning row. It should choose one small allocator behavior slice and
update the taskboard / granularity SSOT before implementation starts.

## Scope

- Review the landed MIMAP-021C allocation-on-miss fallback and lifecycle
  cleanup constraints.
- Select the next single allocator behavior row.
- Add or update the next phase card and required guard names.

## Stop Lines

- Do not implement allocator behavior in this row.
- Do not reopen provider hooks, host allocator replacement, or
  `#[global_allocator]`.
- Do not combine selection with cleanup sidecars.

## Required Evidence

```text
bash tools/checks/k2_wide_mimap022a_next_row_selection_guard.sh
bash tools/checks/current_state_pointer_guard.sh
tools/checks/dev_gate.sh quick
```
