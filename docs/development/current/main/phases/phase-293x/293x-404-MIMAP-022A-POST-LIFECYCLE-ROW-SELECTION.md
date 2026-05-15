# 293x-404 MIMAP-022A Post-Lifecycle Row Selection

Status: landed
Date: 2026-05-16

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

## Closeout

`MIMAP-022A` selects
`MIMAP-022B facade huge-request fail-fast routing` as the next allocator
behavior row.

Rationale:

- `MIMAP-021B` and `MIMAP-021C` connected the active object-lifecycle facade to
  page-source by adding exactly one fresh page and retrying one small
  allocation after a `small_no_page()` miss.
- Lifecycle cleanup is closed through `REUSE-LIFECYCLE-001`, so the next
  allocator behavior should stay on the facade/page-source path instead of
  reopening construction semantics.
- The existing `M179` huge-threshold router is complete on the older
  page-map-backed path, but the current facade/page-source route still needs a
  narrow fail-fast boundary for oversized requests.
- The smallest honest next row is therefore a facade-local huge-request
  classifier that rejects huge requests before page-source attach/retry. It
  must not implement a huge page model, page-map lookup, provider activation,
  host allocator replacement, or `#[global_allocator]`.

Selected next row:

```text
docs/development/current/main/phases/phase-293x/293x-434-MIMAP-022B-FACADE-HUGE-REQUEST-FAILFAST-ROUTING.md
```

## Required Evidence

```text
bash tools/checks/k2_wide_mimap022a_next_row_selection_guard.sh
bash tools/checks/current_state_pointer_guard.sh
tools/checks/dev_gate.sh quick
```
