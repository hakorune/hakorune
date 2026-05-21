# 293x-1062 MIMAP-440A Allocator Comparison Controlled Benchmark Execution Inventory

Status: landed
Date: 2026-05-21

## Purpose

Inventory the first controlled allocator comparison benchmark execution shape.
This row should make the benchmark runner, output contract, workload source,
and measurement source explicit while process allocator replacement remains
closed.

## Scope

- Track explicit benchmark runner selection.
- Track explicit workload and measurement-plan sources.
- Track explicit output contract and evidence storage plan.
- Keep process allocator replacement, hooks, backend matcher additions, and
  global allocator installation closed.

## Stop Lines

- No hook installation.
- No backend matcher additions.
- No process allocator replacement.
- No `#[global_allocator]`.
- No hidden env, implicit discovery, or process-global activation config.
- No source-level worker-local or concurrency surface.
- No cross-function `Result` direct ABI or runtime sum materialization.

## Landed Scope

- Added `HakoAllocAllocatorComparisonControlledBenchmarkExecutionInventory`
  with explicit runner, workload source, measurement source, output contract,
  evidence storage, and representative-run inputs.
- Added a manifest-backed proof app and L2 guard for the controlled benchmark
  execution inventory shape.
- Added the MIMAP-440A inventory SSOT and selected MIMAP-441A diagnostics.
- Kept benchmark execution, process allocator replacement, hooks, backend
  matcher additions, global allocator installation, hidden env discovery, and
  worker/thread execution closed.

## Validation

- `bash tools/checks/k2_wide_hako_alloc_allocator_comparison_controlled_benchmark_execution_inventory_guard.sh --level L2`
- `bash tools/checks/run_proof_app.sh --only MIMAP-440A --level L2`
- `bash tools/checks/current_state_pointer_guard.sh`
- `git diff --check`
