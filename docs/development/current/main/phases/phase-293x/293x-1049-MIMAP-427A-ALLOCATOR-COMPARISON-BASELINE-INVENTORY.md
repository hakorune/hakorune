# 293x-1049 MIMAP-427A Allocator Comparison Baseline Inventory

Status: selected current
Date: 2026-05-21

## Purpose

Inventory the comparison baseline needed to judge `.hako` / `hako_alloc`
against C mimalloc. This row should define the measurement inputs without
changing allocator behavior or replacing the process allocator.

## Scope

- Name throughput and memory-usage baseline inputs.
- Keep process replacement parked.
- Keep optional replacement execution parked.
- Keep the comparison target explicit: C mimalloc performance and memory
  usage.

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
