---
Status: Landed
Date: 2026-05-23
Scope: C mimalloc result summary diagnostic owner-local counter exact `usize` migration.
Related:
  - docs/development/current/main/design/usize-semantic-foundation-ssot.md
  - docs/development/current/main/phases/phase-294x/294x-119-HAKO-ALLOC-USIZE-C-MIMALLOC-RESULT-SUMMARY-DIAGNOSTIC-COUNTER-SELECTION.md
  - docs/development/current/main/phases/phase-294x/294x-90-usize-semantics-taskboard.md
  - lang/src/hako_alloc/memory/NUMERIC_FIELDS.md
  - lang/src/hako_alloc/memory/allocator_comparison_c_mimalloc_result_summary_diagnostic_box.hako
  - apps/hako-alloc-allocator-comparison-c-mimalloc-result-summary-diagnostics-proof/main.hako
  - tools/checks/k2_wide_hako_alloc_allocator_comparison_c_mimalloc_result_summary_diagnostics_guard.sh
---

# 294x-120 Hako Alloc Usize C Mimalloc Result Summary Diagnostic Counters

## Decision

Migrate only the selected
`HakoAllocAllocatorComparisonCMimallocResultSummaryDiagnostic` owner-local
monotonic counters to exact `usize` storage:

- `diagnostic_count`
- `ready_count`
- `blocked_count`
- `missing_summary_blocked_count`
- `blocked_summary_blocked_count`

The MIMAP-458A C mimalloc result summary diagnostics guard now asserts these
fields are exact `usize` in the typed-object plan.

## Stop Line

This row does not migrate:

- comparison payloads such as allocation counts, requested bytes, RSS bytes, or
  deltas;
- `last_reason`, because it is reason vocabulary;
- report fields and `ReportFields` mirrors, because they remain signed
  comparison payload/mirror seams until their own row;
- performance conclusions, memory conclusions, repeated benchmark execution,
  process allocator replacement, hooks, backend matcher additions, provider
  package generation, worker/TLS, threads, or `#[global_allocator]`.

## Verification

```bash
bash tools/checks/k2_wide_hako_alloc_allocator_comparison_c_mimalloc_result_summary_diagnostics_guard.sh
bash tools/checks/current_state_pointer_guard.sh
git diff --check
```
