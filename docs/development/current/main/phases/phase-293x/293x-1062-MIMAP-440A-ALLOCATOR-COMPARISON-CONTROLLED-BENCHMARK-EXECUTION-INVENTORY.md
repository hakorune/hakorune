# 293x-1062 MIMAP-440A Allocator Comparison Controlled Benchmark Execution Inventory

Status: selected current
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

## Validation

Daily validation should be L0/L1 unless a proof app is added.
