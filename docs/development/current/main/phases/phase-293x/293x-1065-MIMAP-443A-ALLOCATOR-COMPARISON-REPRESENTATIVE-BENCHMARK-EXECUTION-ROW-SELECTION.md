# 293x-1065 MIMAP-443A Allocator Comparison Representative Benchmark Execution Row Selection

Status: landed
Date: 2026-05-21

## Purpose

Select the next explicit row after controlled benchmark execution closeout.
The likely next boundary is a representative benchmark execution pilot, but
this selection row must keep process allocator replacement and process-global
activation closed unless the next card opens a narrow execution seam.

## Scope

- Review the MIMAP-440A / MIMAP-441A / MIMAP-442A controlled execution pack.
- Select the next smallest representative benchmark execution row.
- Preserve the distinction between running a controlled comparison benchmark
  and replacing the process allocator.

Selected row:

```text
MIMAP-444A Allocator Comparison Representative Benchmark Execution Pilot
```

## Stop Lines

- No hook installation.
- No backend matcher additions.
- No process allocator replacement.
- No `#[global_allocator]`.
- No hidden env, implicit discovery, or process-global activation config.
- No source-level worker-local or concurrency surface.
- No cross-function `Result` direct ABI or runtime sum materialization.

## Validation

Planning validation only unless this row adds an executable proof.

## Landed Scope

- Selected MIMAP-444A as the first representative benchmark execution pilot.
- Kept benchmark execution distinct from process allocator replacement.
- Kept hooks, backend matcher additions, global allocator installation, hidden
  env discovery, and source-level concurrency closed.

## Evidence

- `bash tools/checks/k2_wide_hako_alloc_allocator_comparison_representative_benchmark_execution_row_selection_guard.sh`
- `bash tools/checks/current_state_pointer_guard.sh`
- `git diff --check`
