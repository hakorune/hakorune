# 293x-1068 MIMAP-446A Allocator Comparison Representative Benchmark Execution Closeout

Status: landed
Date: 2026-05-21

## Purpose

Close the representative benchmark execution pack after MIMAP-444A execution
pilot and MIMAP-445A diagnostics.

## Scope

- Validate the MIMAP-444A representative benchmark execution pilot evidence.
- Validate the MIMAP-445A observer-only diagnostics.
- Keep the closeout focused on HakoAllocProductionFacade representative metrics.
- Select the next allocator comparison row after the closeout.

## Stop Lines

- No process allocator replacement.
- No hook installation.
- No backend matcher additions.
- No `#[global_allocator]`.
- No C mimalloc execution unless a later row explicitly opens it.
- No hidden env, implicit discovery, or process-global activation config.
- No source-level worker-local or concurrency surface.
- No cross-function `Result` direct ABI or runtime sum materialization.

## Landed Scope

- Added the representative benchmark execution closeout SSOT.
- Added a closeout guard that re-runs MIMAP-444A pilot and MIMAP-445A
  diagnostics L2 evidence.
- Selected MIMAP-447A as the C mimalloc comparison execution plan row.
- Kept process allocator replacement, hooks, backend matcher additions, global
  allocator installation, C mimalloc execution, hidden env discovery, and
  worker/thread execution closed.

## Validation

- `bash tools/checks/k2_wide_hako_alloc_allocator_comparison_representative_benchmark_execution_closeout_guard.sh`
- `bash tools/checks/current_state_pointer_guard.sh`
- `git diff --check`
