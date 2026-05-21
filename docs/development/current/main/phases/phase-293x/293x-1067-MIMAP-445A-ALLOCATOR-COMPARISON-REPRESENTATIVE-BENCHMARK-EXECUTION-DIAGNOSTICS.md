# 293x-1067 MIMAP-445A Allocator Comparison Representative Benchmark Execution Diagnostics

Status: landed
Date: 2026-05-21

## Purpose

Add observer-only diagnostics for the MIMAP-444A representative benchmark
execution pilot.

## Scope

- Consume the MIMAP-444A execution report.
- Classify not-ready input, invalid run count, missing output/evidence, and
  closed-seam reasons.
- Keep process allocator replacement, hooks, backend matcher additions, global
  allocator installation, C mimalloc execution, and worker/thread execution
  closed.

## Stop Lines

- No hook installation.
- No backend matcher additions.
- No process allocator replacement.
- No `#[global_allocator]`.
- No C mimalloc execution.
- No hidden env, implicit discovery, or process-global activation config.
- No source-level worker-local or concurrency surface.
- No cross-function `Result` direct ABI or runtime sum materialization.

## Validation

Validation profile: `scalar-mir`.

Evidence:

- `bash tools/checks/k2_wide_hako_alloc_allocator_comparison_representative_benchmark_execution_diagnostics_guard.sh --level L2`
- `bash tools/checks/run_proof_app.sh --only MIMAP-445A --level L2`

L3/L4 evidence is deferred to MIMAP-446A closeout.
