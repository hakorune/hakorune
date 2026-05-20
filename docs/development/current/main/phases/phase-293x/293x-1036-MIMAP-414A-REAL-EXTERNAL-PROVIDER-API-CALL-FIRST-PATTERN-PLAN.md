# 293x-1036 MIMAP-414A Real External Provider API Call First-Pattern Plan

Status: landed
Date: 2026-05-21

## Purpose

Plan the first real external provider API call seam after the external provider
API adapter execution preflight closeout. This row defines the boundary and
acceptance criteria before any later row executes a real external provider API
call.

## Scope

- Define the first-pattern real external provider API call owner boundary.
- Record the required input evidence from the MIMAP-410A preflight report.
- Define the report fields for an eventual real-call pilot.
- Keep actual external provider API execution closed in this planning row.
- Keep host allocator replacement, hooks, backend matcher additions,
  worker/thread execution, and global allocator install closed.

## Stop Lines

- No actual external provider API execution.
- No host allocator replacement, hooks, backend matcher additions, or
  `#[global_allocator]`.
- No hidden env, implicit discovery, or process-global activation config.
- No source-level worker-local or concurrency surface.
- No cross-function `Result` direct ABI or runtime sum materialization.

## Validation

Planning validation is L0:

```text
current state pointer guard
git diff --check
```

## Required Evidence

```text
bash tools/checks/k2_wide_hako_alloc_real_external_provider_api_call_first_pattern_plan_guard.sh
bash tools/checks/current_state_pointer_guard.sh
git diff --check
```

## Landed Evidence

- Added the real external provider API call first-pattern plan SSOT.
- Added the planning guard.
- Selected the first-pattern pilot as the next row.
- Kept actual external provider API execution closed in this planning row.
- Kept host allocator replacement, hooks, backend matcher additions,
  worker/thread execution, and global allocator install closed.
