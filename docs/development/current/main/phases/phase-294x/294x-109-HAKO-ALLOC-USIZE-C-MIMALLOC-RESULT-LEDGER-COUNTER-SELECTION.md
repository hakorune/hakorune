---
Status: Landed
Date: 2026-05-23
Scope: select the next owner-local production exact `usize` field group after the explicit C mimalloc runner evidence diagnostic counter migration.
Related:
  - docs/development/current/main/design/usize-semantic-foundation-ssot.md
  - docs/development/current/main/phases/phase-294x/294x-108-HAKO-ALLOC-USIZE-C-MIMALLOC-EXPLICIT-RUNNER-EVIDENCE-DIAGNOSTIC-COUNTERS.md
  - docs/development/current/main/phases/phase-294x/294x-90-usize-semantics-taskboard.md
  - lang/src/hako_alloc/memory/NUMERIC_FIELDS.md
  - lang/src/hako_alloc/memory/allocator_comparison_c_mimalloc_result_ledger_box.hako
  - tools/checks/k2_wide_hako_alloc_allocator_comparison_c_mimalloc_result_ledger_guard.sh
---

# 294x-109 Hako Alloc Usize C Mimalloc Result Ledger Counter Selection

## Decision

Select the owner-local monotonic counters in
`HakoAllocAllocatorComparisonCMimallocResultLedger` as
`HAKO-ALLOC-USIZE-FIELD-GROUP-131`:

- `ledger_count`
- `accepted_count`
- `reject_count`
- `missing_hako_diagnostic_reject_count`
- `blocked_hako_diagnostic_reject_count`
- `missing_c_diagnostic_reject_count`
- `blocked_c_diagnostic_reject_count`

These fields count the MIMAP-454A scalar comparison-result ledger owner's local
ledger attempts and reject outcomes. They do not carry comparison payloads,
signed deltas, runner RSS evidence, reason vocabulary, conclusions, or
provider / host allocator state.

## Stop Line

This selection does not migrate:

- `last_reason`, because it is reason vocabulary;
- `HakoAllocAllocatorComparisonCMimallocResultLedgerReportFields` and
  `HakoAllocAllocatorComparisonCMimallocResultLedgerReport` fields, because
  report mirrors and comparison payloads stay signed until their own row;
- hako/C allocation counts, requested bytes, RSS evidence, deltas, conclusion
  flags, repeated benchmark execution, process allocator replacement, hooks,
  backend matcher additions, provider package generation, worker/TLS, threads,
  or `#[global_allocator]`.

## Verification

Selection-only row:

```bash
bash tools/checks/current_state_pointer_guard.sh
git diff --check
```
