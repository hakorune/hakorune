---
Status: Landed
Date: 2026-05-23
Scope: C mimalloc result ledger diagnostic owner-local counter exact `usize` migration.
Related:
  - docs/development/current/main/design/usize-semantic-foundation-ssot.md
  - docs/development/current/main/phases/phase-294x/294x-111-HAKO-ALLOC-USIZE-C-MIMALLOC-RESULT-LEDGER-DIAGNOSTIC-COUNTER-SELECTION.md
  - docs/development/current/main/phases/phase-294x/294x-90-usize-semantics-taskboard.md
  - lang/src/hako_alloc/memory/NUMERIC_FIELDS.md
  - lang/src/hako_alloc/memory/allocator_comparison_c_mimalloc_result_ledger_diagnostic_box.hako
  - apps/hako-alloc-allocator-comparison-c-mimalloc-result-ledger-diagnostics-proof/main.hako
  - tools/checks/k2_wide_hako_alloc_allocator_comparison_c_mimalloc_result_ledger_diagnostics_guard.sh
---

# 294x-112 Hako Alloc Usize C Mimalloc Result Ledger Diagnostic Counters

## Decision

Migrate only the selected
`HakoAllocAllocatorComparisonCMimallocResultLedgerDiagnostic` owner-local
monotonic counters to exact `usize` storage:

- `diagnostic_count`
- `ready_count`
- `blocked_count`
- `missing_hako_blocked_count`
- `blocked_hako_blocked_count`
- `missing_c_blocked_count`
- `blocked_c_blocked_count`

The MIMAP-455A C mimalloc result ledger diagnostic guard now asserts these
fields are exact `usize` in the typed-object plan.

## Stop Line

This row does not migrate:

- `last_reason`, because it is reason vocabulary;
- report fields and `ReportFields` mirrors, because they remain signed
  comparison payload/mirror seams until their own row;
- hako/C allocation counts, requested bytes, RSS evidence, signed deltas,
  conclusion flags, repeated benchmark execution, process allocator
  replacement, hooks, backend matcher additions, provider package generation,
  worker/TLS, threads, or `#[global_allocator]`.

## Verification

```bash
bash tools/checks/k2_wide_hako_alloc_allocator_comparison_c_mimalloc_result_ledger_diagnostics_guard.sh
bash tools/checks/current_state_pointer_guard.sh
git diff --check
```
