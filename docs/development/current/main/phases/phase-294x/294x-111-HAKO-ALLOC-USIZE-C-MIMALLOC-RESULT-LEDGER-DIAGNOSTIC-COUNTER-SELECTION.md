---
Status: Landed
Date: 2026-05-23
Scope: select the next owner-local production exact `usize` field group after the C mimalloc result ledger counter migration.
Related:
  - docs/development/current/main/design/usize-semantic-foundation-ssot.md
  - docs/development/current/main/phases/phase-294x/294x-110-HAKO-ALLOC-USIZE-C-MIMALLOC-RESULT-LEDGER-COUNTERS.md
  - docs/development/current/main/phases/phase-294x/294x-90-usize-semantics-taskboard.md
  - lang/src/hako_alloc/memory/NUMERIC_FIELDS.md
  - lang/src/hako_alloc/memory/allocator_comparison_c_mimalloc_result_ledger_diagnostic_box.hako
  - tools/checks/k2_wide_hako_alloc_allocator_comparison_c_mimalloc_result_ledger_diagnostics_guard.sh
---

# 294x-111 Hako Alloc Usize C Mimalloc Result Ledger Diagnostic Counter Selection

## Decision

Select the owner-local monotonic counters in
`HakoAllocAllocatorComparisonCMimallocResultLedgerDiagnostic` as
`HAKO-ALLOC-USIZE-FIELD-GROUP-133`:

- `diagnostic_count`
- `ready_count`
- `blocked_count`
- `missing_hako_blocked_count`
- `blocked_hako_blocked_count`
- `missing_c_blocked_count`
- `blocked_c_blocked_count`

These fields count the MIMAP-455A C mimalloc result-ledger diagnostic owner's
local classifications and blocked outcomes. They do not carry comparison
payloads, signed deltas, reason vocabulary, conclusions, or provider / host
allocator state.

## Stop Line

This selection does not migrate:

- `last_reason`, because it is reason vocabulary;
- `HakoAllocAllocatorComparisonCMimallocResultLedgerDiagnosticReportFields` and
  `HakoAllocAllocatorComparisonCMimallocResultLedgerDiagnosticReport` fields,
  because report mirrors and comparison payloads stay signed until their own
  row;
- hako/C allocation counts, requested bytes, RSS evidence, signed deltas,
  conclusion flags, repeated benchmark execution, process allocator
  replacement, hooks, backend matcher additions, provider package generation,
  worker/TLS, threads, or `#[global_allocator]`.

## Verification

Selection-only row:

```bash
bash tools/checks/current_state_pointer_guard.sh
git diff --check
```
