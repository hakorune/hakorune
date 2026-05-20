# 293x-1055 MIMAP-433A Allocator Comparison Measurement Plan Inventory

Status: selected current
Date: 2026-05-21

## Purpose

Inventory the measurement plan after the allocator comparison workload matrix
pack is closed out. This row should record the explicit measurement parameters
needed before benchmark execution opens.

## Scope

- Track run count / warmup / output contract inputs.
- Track throughput and memory-usage measurement readiness.
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

Daily validation should be L0/L1 unless a proof app is added.
