# 293x-1061 MIMAP-439A Allocator Comparison Controlled Benchmark Execution Plan

Status: selected current
Date: 2026-05-21

## Purpose

Select the first controlled allocator comparison benchmark execution shape
after the benchmark preflight pack is closed out.

This row should plan the first explicit benchmark execution seam without
opening process allocator replacement, hooks, backend matcher additions, or
`#[global_allocator]`.

## Scope

- Define the first benchmark execution shape and validation profile.
- Keep the execution bounded and representative.
- Keep process allocator replacement and global allocator installation closed.

## Stop Lines

- No hook installation.
- No backend matcher additions.
- No process allocator replacement.
- No `#[global_allocator]`.
- No hidden env, implicit discovery, or process-global activation config.
- No source-level worker-local or concurrency surface.
- No cross-function `Result` direct ABI or runtime sum materialization.

## Validation

Planning validation is L0 unless this row adds an executable proof.
