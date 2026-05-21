# 293x-1069 MIMAP-447A Allocator Comparison C Mimalloc Execution Plan

Status: landed
Date: 2026-05-21

## Purpose

Plan the first C mimalloc comparison execution seam after the Hako representative
benchmark execution pack is closed.

## Scope

- Define the C mimalloc comparison workload boundary.
- Define the output and memory-use evidence contract.
- Keep the row planning-only unless the card explicitly opens execution.
- Preserve the Hako representative benchmark metrics as the comparison input.

## Stop Lines

- No process allocator replacement.
- No hook installation.
- No backend matcher additions.
- No `#[global_allocator]`.
- No implicit C mimalloc execution.
- No hidden env, implicit discovery, or process-global activation config.
- No source-level worker-local or concurrency surface.
- No cross-function `Result` direct ABI or runtime sum materialization.

## Validation

Planning validation is L0.

## Landed Scope

- Added the C mimalloc execution plan SSOT.
- Selected MIMAP-448A as the C mimalloc execution inventory row.
- Kept C mimalloc execution explicit and still closed until an execution row
  opens it.
- Kept process allocator replacement, hooks, backend matcher additions, global
  allocator installation, hidden env discovery, and worker/thread execution
  closed.

## Evidence

- `bash tools/checks/k2_wide_hako_alloc_allocator_comparison_c_mimalloc_execution_plan_guard.sh`
- `bash tools/checks/current_state_pointer_guard.sh`
- `git diff --check`
