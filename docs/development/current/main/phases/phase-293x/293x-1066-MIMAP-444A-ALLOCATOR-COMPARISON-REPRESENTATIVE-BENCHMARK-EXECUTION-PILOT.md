# 293x-1066 MIMAP-444A Allocator Comparison Representative Benchmark Execution Pilot

Status: landed
Date: 2026-05-21

## Purpose

Open the first narrow representative benchmark execution seam for allocator
comparison evidence. This row may execute a controlled benchmark runner, but it
must not replace the process allocator or install process-global hooks.

## Scope

- Use the MIMAP-440A controlled execution inventory and MIMAP-441A diagnostics
  as the explicit readiness boundary.
- Execute only a representative, bounded allocator comparison benchmark shape.
- Capture output through the explicit output contract and evidence storage
  selected by the controlled execution pack.
- Keep process allocator replacement separate from benchmark execution.

## Stop Lines

- No hook installation.
- No backend matcher additions.
- No process allocator replacement.
- No `#[global_allocator]`.
- No hidden env, implicit discovery, or process-global activation config.
- No source-level worker-local or concurrency surface.
- No cross-function `Result` direct ABI or runtime sum materialization.

## Validation

This is a first execution seam. Validation should include representative
evidence for the exact runner shape chosen by the row.

## Landed Scope

- Added `HakoAllocAllocatorComparisonRepresentativeBenchmarkExecutionPilot`
  as the first bounded representative benchmark execution owner.
- Added a manifest-backed proof app and L2 guard for the selected
  `HakoAllocProductionFacade` workload metrics.
- Added the MIMAP-444A pilot SSOT and selected MIMAP-445A diagnostics.
- Kept process allocator replacement, hooks, backend matcher additions, global
  allocator installation, C mimalloc execution, hidden env discovery, and
  worker/thread execution closed.

## Evidence

- `bash tools/checks/k2_wide_hako_alloc_allocator_comparison_representative_benchmark_execution_pilot_guard.sh --level L2`
- `bash tools/checks/run_proof_app.sh --only MIMAP-444A --level L2`
- `bash tools/checks/current_state_pointer_guard.sh`
- `git diff --check`
