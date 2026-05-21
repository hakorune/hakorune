# 293x-1079 MIMAP-455A Allocator Comparison C Mimalloc Result Ledger Diagnostics

Status: selected current
Date: 2026-05-21

## Purpose

Add observer-only diagnostics over the MIMAP-454A comparison result ledger.

## Scope

- Consume `HakoAllocAllocatorComparisonCMimallocResultLedgerReport`.
- Classify accepted / missing-Hako / blocked-Hako / missing-C / blocked-C
  result ledger rows.
- Preserve scalar evidence and stop-line fields.
- Do not rerun Hako or C benchmarks.
- Do not make a performance or memory-use conclusion.

## Stop Lines

- No repeated or heavy benchmark pack.
- No process allocator replacement.
- No hook installation.
- No backend matcher additions.
- No `#[global_allocator]`.
- No provider package / DLL generation.
- No hidden env, implicit discovery, or process-global activation config.
- No source-level worker-local or concurrency surface.
- No worker/thread execution.
- No cross-function `Result` direct ABI or runtime sum materialization.

## Validation

Planned validation profile: `scalar-mir`.
