---
Status: Landed
Date: 2026-05-23
Scope: C mimalloc result ledger owner-local counter exact `usize` migration.
Related:
  - docs/development/current/main/design/usize-semantic-foundation-ssot.md
  - docs/development/current/main/phases/phase-294x/294x-109-HAKO-ALLOC-USIZE-C-MIMALLOC-RESULT-LEDGER-COUNTER-SELECTION.md
  - docs/development/current/main/phases/phase-294x/294x-90-usize-semantics-taskboard.md
  - lang/src/hako_alloc/memory/NUMERIC_FIELDS.md
  - lang/src/hako_alloc/memory/allocator_comparison_c_mimalloc_result_ledger_box.hako
  - apps/hako-alloc-allocator-comparison-c-mimalloc-result-ledger-proof/main.hako
  - tools/checks/k2_wide_hako_alloc_allocator_comparison_c_mimalloc_result_ledger_guard.sh
---

# 294x-110 Hako Alloc Usize C Mimalloc Result Ledger Counters

## Decision

Migrate only the selected
`HakoAllocAllocatorComparisonCMimallocResultLedger` owner-local monotonic
counters to exact `usize` storage:

- `ledger_count`
- `accepted_count`
- `reject_count`
- `missing_hako_diagnostic_reject_count`
- `blocked_hako_diagnostic_reject_count`
- `missing_c_diagnostic_reject_count`
- `blocked_c_diagnostic_reject_count`

The MIMAP-454A C mimalloc result ledger guard now asserts these fields are
exact `usize` in the typed-object plan.

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
bash tools/checks/k2_wide_hako_alloc_allocator_comparison_c_mimalloc_result_ledger_guard.sh
bash tools/checks/current_state_pointer_guard.sh
git diff --check
```
