# 293x-1057 MIMAP-435A Allocator Comparison Measurement Plan Closeout

Status: selected current
Date: 2026-05-21

## Purpose

Close out the allocator comparison measurement plan inventory and diagnostics
pack before any benchmark execution row is selected.

## Scope

- Re-run MIMAP-433A measurement plan inventory validation.
- Re-run MIMAP-434A measurement plan diagnostics validation.
- Keep benchmark execution and process replacement closed.

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

Closeout validation should include the L2 row guards. L3 remains optional and
must stay representative-only unless a later row explicitly opens benchmark
execution.
