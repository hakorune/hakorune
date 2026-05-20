# 293x-1058 MIMAP-436A Allocator Comparison Benchmark Execution Preflight Inventory

Status: landed
Date: 2026-05-21

## Purpose

Inventory the explicit preflight conditions for a future allocator comparison
benchmark execution row. This row should not run the benchmark; it should
record what must be true before benchmark execution can be opened.

## Scope

- Track closed-state preconditions for benchmark runner selection.
- Track closed-state preconditions for output capture and measurement storage.
- Track that process allocator replacement and global allocator installation
  remain inactive.

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
bash tools/checks/k2_wide_hako_alloc_allocator_comparison_benchmark_execution_preflight_inventory_guard.sh --level L2
bash tools/checks/current_state_pointer_guard.sh
git diff --check
```

## Landed Evidence

- Added the benchmark execution preflight inventory owner and report.
- Added a scalar proof app for benchmark runner, output capture, measurement
  storage, workload matrix, measurement plan, and closed-seam readiness.
- Kept benchmark execution, hook installation, backend matcher additions,
  process allocator replacement, worker/thread execution, and global allocator
  install closed.
