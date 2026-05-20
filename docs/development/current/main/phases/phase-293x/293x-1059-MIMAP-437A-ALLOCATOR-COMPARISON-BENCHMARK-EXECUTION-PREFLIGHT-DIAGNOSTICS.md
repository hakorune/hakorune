# 293x-1059 MIMAP-437A Allocator Comparison Benchmark Execution Preflight Diagnostics

Status: landed
Date: 2026-05-21

## Purpose

Consume the MIMAP-436A benchmark execution preflight inventory report and
publish observer-only diagnostics for missing preflight inputs or open
execution seams.

## Scope

- Classify missing runner, output capture, and measurement storage inputs.
- Classify not-ready workload matrix and measurement plan inputs.
- Classify process replacement, hook, backend matcher, global allocator, and
  hidden-env seams as blockers.
- Keep benchmark execution closed.

## Stop Lines

- No benchmark execution.
- No hook installation.
- No backend matcher additions.
- No process allocator replacement.
- No `#[global_allocator]`.
- No hidden env, implicit discovery, or process-global activation config.
- No source-level worker-local or concurrency surface.
- No cross-function `Result` direct ABI or runtime sum materialization.

## Validation

```text
bash tools/checks/k2_wide_hako_alloc_allocator_comparison_benchmark_execution_preflight_diagnostics_guard.sh --level L2
bash tools/checks/current_state_pointer_guard.sh
git diff --check
```

## Landed Evidence

- Added the benchmark execution preflight diagnostics owner and report.
- Added a scalar proof app for missing preflight input and open-seam
  diagnostics.
- Kept benchmark execution, hook installation, backend matcher additions,
  process allocator replacement, worker/thread execution, and global allocator
  install closed.
