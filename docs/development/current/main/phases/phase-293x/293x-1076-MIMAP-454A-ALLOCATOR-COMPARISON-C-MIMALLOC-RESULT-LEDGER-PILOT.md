# 293x-1076 MIMAP-454A Allocator Comparison C Mimalloc Result Ledger Pilot

Status: landed
Date: 2026-05-21

## Purpose

Open a narrow comparison-result ledger over explicit C mimalloc runner evidence
and existing Hako representative metrics.

## Scope

- Consume MIMAP-451A / MIMAP-452A explicit C runner evidence/diagnostics.
- Consume existing Hako representative metrics from the allocator comparison
  lane.
- Record comparison availability and scalar result fields.
- Keep this as a ledger row, not a performance conclusion or replacement row.

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

## Queue Note

ARG-DATA-001/002 inserted and landed a short BoxShape sidecar before this row.
MIMAP-454A resumes after reducing owner-local explicit-runner argument
transport without changing runner/report behavior.

## Design

SSOT:

```text
docs/development/current/main/design/hako-alloc-allocator-comparison-c-mimalloc-result-ledger-ssot.md
```

## Completed

- Added `HakoAllocAllocatorComparisonCMimallocResultLedger`.
- Added the MIMAP-454A proof app and manifest-backed guard.
- Recorded Hako representative benchmark scalar evidence alongside explicit C
  mimalloc runner scalar / memory evidence.
- Kept repeated benchmark execution, conclusions, allocator replacement, hooks,
  backend matchers, `#[global_allocator]`, provider package generation, and
  worker/thread execution closed.

Validated:

```bash
bash tools/checks/k2_wide_hako_alloc_allocator_comparison_c_mimalloc_result_ledger_guard.sh --level L2
```
