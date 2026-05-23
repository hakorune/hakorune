---
Status: Landed
Date: 2026-05-23
Scope: select the next owner-local production exact `usize` field group after the C mimalloc result summary inventory counter migration.
Related:
  - docs/development/current/main/design/usize-semantic-foundation-ssot.md
  - docs/development/current/main/phases/phase-294x/294x-118-HAKO-ALLOC-USIZE-C-MIMALLOC-RESULT-SUMMARY-INVENTORY-COUNTERS.md
  - docs/development/current/main/phases/phase-294x/294x-90-usize-semantics-taskboard.md
  - lang/src/hako_alloc/memory/NUMERIC_FIELDS.md
  - lang/src/hako_alloc/memory/allocator_comparison_c_mimalloc_result_summary_diagnostic_box.hako
  - tools/checks/k2_wide_hako_alloc_allocator_comparison_c_mimalloc_result_summary_diagnostics_guard.sh
---

# 294x-119 Hako Alloc Usize C Mimalloc Result Summary Diagnostic Counter Selection

## Decision

Select the owner-local monotonic counters in
`HakoAllocAllocatorComparisonCMimallocResultSummaryDiagnostic` as
`HAKO-ALLOC-USIZE-FIELD-GROUP-141`:

- `diagnostic_count`
- `ready_count`
- `blocked_count`
- `missing_summary_blocked_count`
- `blocked_summary_blocked_count`

These fields count the MIMAP-458A C mimalloc result summary diagnostic owner's
local classifications and blocked outcomes. They do not carry allocation-count
payloads, requested-byte payloads, RSS evidence, deltas, readiness flags, reason
vocabulary, or provider / host allocator state.

## Stop Line

This selection does not migrate:

- comparison payloads such as allocation counts, requested bytes, RSS bytes, or
  deltas;
- `last_reason`, because it is reason vocabulary;
- `HakoAllocAllocatorComparisonCMimallocResultSummaryDiagnosticReportFields`
  and `HakoAllocAllocatorComparisonCMimallocResultSummaryDiagnosticReport`
  fields, because report mirrors stay signed until their own row;
- performance conclusions, memory conclusions, repeated benchmark execution,
  process allocator replacement, hooks, backend matcher additions, provider
  package generation, worker/TLS, threads, or `#[global_allocator]`.

## Verification

Selection-only row:

```bash
bash tools/checks/current_state_pointer_guard.sh
git diff --check
```
