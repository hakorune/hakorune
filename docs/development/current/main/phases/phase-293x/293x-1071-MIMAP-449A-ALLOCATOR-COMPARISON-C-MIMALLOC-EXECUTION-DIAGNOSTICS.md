# 293x-1071 MIMAP-449A Allocator Comparison C Mimalloc Execution Diagnostics

Status: selected current
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

Daily validation should be L0/L1 unless a proof app is added.
