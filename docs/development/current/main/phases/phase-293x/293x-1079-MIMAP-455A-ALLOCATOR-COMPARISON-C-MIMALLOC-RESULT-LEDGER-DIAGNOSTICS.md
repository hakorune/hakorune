# 293x-1079 MIMAP-455A Allocator Comparison C Mimalloc Result Ledger Diagnostics

Status: landed
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

## Design

SSOT:

```text
docs/development/current/main/design/hako-alloc-allocator-comparison-c-mimalloc-result-ledger-diagnostics-ssot.md
```

## Completed

- Added `HakoAllocAllocatorComparisonCMimallocResultLedgerDiagnostic`.
- Added the MIMAP-455A proof app and manifest-backed guard.
- Classified accepted / missing-Hako / blocked-Hako / missing-C / blocked-C
  ledger rows.
- Preserved scalar metrics and stop-line fields without rerunning benchmarks or
  making performance / memory-use conclusions.

Validated:

```bash
bash tools/checks/k2_wide_hako_alloc_allocator_comparison_c_mimalloc_result_ledger_diagnostics_guard.sh --level L2
```
