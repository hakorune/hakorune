# 293x-1064 MIMAP-442A Allocator Comparison Controlled Benchmark Execution Closeout

Status: selected current
Date: 2026-05-21

## Purpose

Close the controlled benchmark execution inventory and diagnostics pack before
opening any representative benchmark execution row.

## Scope

- Re-run MIMAP-440A inventory evidence.
- Re-run MIMAP-441A diagnostics evidence.
- Confirm both rows share the controlled benchmark execution closeout pack.
- Keep process allocator replacement, hooks, backend matcher additions, global
  allocator installation, hidden env discovery, and worker/thread execution
  closed.

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

Closeout validation should remain representative and avoid enabling benchmark
execution until the next explicit execution row.
