# 293x-1071 MIMAP-449A Allocator Comparison C Mimalloc Execution Diagnostics

Status: landed
Date: 2026-05-21

## Purpose

Add observer-only diagnostics for the MIMAP-448A C mimalloc execution
inventory.

## Scope

- Consume the MIMAP-448A inventory report.
- Classify missing runner, missing workload, missing Hako metrics, missing
  output contract, missing memory-usage contract, missing evidence storage,
  missing run count, and invalid run count.
- Keep C mimalloc execution, process allocator replacement, hooks, backend
  matcher additions, global allocator installation, and worker/thread execution
  closed.

## Stop Lines

- No C mimalloc execution.
- No process allocator replacement.
- No hook installation.
- No backend matcher additions.
- No `#[global_allocator]`.
- No implicit C mimalloc execution.
- No hidden env, implicit discovery, or process-global activation config.
- No source-level worker-local or concurrency surface.
- No cross-function `Result` direct ABI or runtime sum materialization.

## Validation

Validation profile: `scalar-mir`.

Evidence:

- `bash tools/checks/k2_wide_hako_alloc_allocator_comparison_c_mimalloc_execution_diagnostics_guard.sh --level L2`
- `bash tools/checks/run_proof_app.sh --only MIMAP-449A --level L2`

L3/L4 evidence is deferred to MIMAP-450A closeout.
