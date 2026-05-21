# 293x-1063 MIMAP-441A Allocator Comparison Controlled Benchmark Execution Diagnostics

Status: landed
Date: 2026-05-21

## Purpose

Add observer-only diagnostics for the MIMAP-440A controlled benchmark execution
inventory. This row should classify missing execution-shape inputs and open
closed-seam inputs without executing a benchmark.

## Scope

- Consume the MIMAP-440A inventory report.
- Classify missing runner, source, output, evidence, and representative-run
  inputs.
- Classify process allocator replacement, hook, backend matcher, global
  allocator, and hidden-env open-state reasons.
- Keep actual benchmark execution and process-global activation closed.

## Stop Lines

- No benchmark execution.
- No hook installation.
- No backend matcher additions.
- No process allocator replacement.
- No `#[global_allocator]`.
- No hidden env, implicit discovery, or process-global activation config.
- No source-level worker-local or concurrency surface.
- No cross-function `Result` direct ABI or runtime sum materialization.

## Landed Scope

- Added `HakoAllocAllocatorComparisonControlledBenchmarkExecutionDiagnostic`
  as an observer-only diagnostic owner for the MIMAP-440A inventory report.
- Added a manifest-backed proof app and L2 guard for missing controlled
  execution-shape inputs and closed-seam execution leaks.
- Added the MIMAP-441A diagnostics SSOT and selected MIMAP-442A closeout.
- Kept benchmark execution, process allocator replacement, hooks, backend
  matcher additions, global allocator installation, hidden env discovery, and
  worker/thread execution closed.

## Validation

- `bash tools/checks/k2_wide_hako_alloc_allocator_comparison_controlled_benchmark_execution_diagnostics_guard.sh --level L2`
- `bash tools/checks/run_proof_app.sh --only MIMAP-441A --level L2`
- `bash tools/checks/current_state_pointer_guard.sh`
- `git diff --check`
