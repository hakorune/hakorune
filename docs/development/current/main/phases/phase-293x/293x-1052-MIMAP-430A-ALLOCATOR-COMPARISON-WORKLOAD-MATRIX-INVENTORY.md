# 293x-1052 MIMAP-430A Allocator Comparison Workload Matrix Inventory

Status: selected current
Date: 2026-05-21

## Purpose

Inventory the comparison workload matrix after the allocator comparison
baseline pack is closed out. This row should name the workload families needed
before `.hako` / `hako_alloc` can be compared against C mimalloc for throughput
and memory usage.

## Scope

- Add an explicit workload-matrix owner or planning row.
- Track throughput and memory-usage comparison workload families.
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
