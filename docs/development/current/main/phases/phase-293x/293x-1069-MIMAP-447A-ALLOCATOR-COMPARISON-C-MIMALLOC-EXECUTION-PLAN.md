# 293x-1069 MIMAP-447A Allocator Comparison C Mimalloc Execution Plan

Status: selected current
Date: 2026-05-21

## Purpose

Plan the first C mimalloc comparison execution seam after the Hako representative
benchmark execution pack is closed.

## Scope

- Define the C mimalloc comparison workload boundary.
- Define the output and memory-use evidence contract.
- Keep the row planning-only unless the card explicitly opens execution.
- Preserve the Hako representative benchmark metrics as the comparison input.

## Stop Lines

- No process allocator replacement.
- No hook installation.
- No backend matcher additions.
- No `#[global_allocator]`.
- No implicit C mimalloc execution.
- No hidden env, implicit discovery, or process-global activation config.
- No source-level worker-local or concurrency surface.
- No cross-function `Result` direct ABI or runtime sum materialization.

## Validation

Planning validation should be L0/L1 unless a proof app or runner contract is
added by this row.
