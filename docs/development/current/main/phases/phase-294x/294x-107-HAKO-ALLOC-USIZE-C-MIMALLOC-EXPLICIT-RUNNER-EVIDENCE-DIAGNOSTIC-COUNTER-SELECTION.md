---
Status: Landed
Date: 2026-05-23
Scope: select the next owner-local production exact `usize` field group after the explicit C mimalloc runner execution pilot counter migration.
Related:
  - docs/development/current/main/design/usize-semantic-foundation-ssot.md
  - docs/development/current/main/phases/phase-294x/294x-106-HAKO-ALLOC-USIZE-C-MIMALLOC-EXPLICIT-RUNNER-EXECUTION-PILOT-COUNTERS.md
  - docs/development/current/main/phases/phase-294x/294x-90-usize-semantics-taskboard.md
  - lang/src/hako_alloc/memory/NUMERIC_FIELDS.md
  - lang/src/hako_alloc/memory/allocator_comparison_c_mimalloc_explicit_runner_evidence_diagnostic_box.hako
  - tools/checks/k2_wide_hako_alloc_allocator_comparison_c_mimalloc_explicit_runner_evidence_diagnostics_guard.sh
---

# 294x-107 Hako Alloc Usize C Mimalloc Explicit Runner Evidence Diagnostic Counter Selection

## Decision

Select the owner-local monotonic counters in
`HakoAllocAllocatorComparisonCMimallocExplicitRunnerEvidenceDiagnostic` as
`HAKO-ALLOC-USIZE-FIELD-GROUP-129`:

- `diagnostic_count`
- `ready_count`
- `blocked_count`
- `missing_diagnostic_blocked_count`
- `rejected_diagnostic_blocked_count`
- `missing_runner_blocked_count`
- `missing_output_blocked_count`
- `missing_memory_evidence_blocked_count`
- `missing_output_contract_blocked_count`
- `failed_runner_blocked_count`
- `invalid_run_count_blocked_count`

These fields count the MIMAP-452A explicit C mimalloc runner evidence
diagnostic owner's local classifications and blocked outcomes. They do not
carry runner payloads, RSS evidence, result codes, reason vocabulary,
stop-line flags, or provider / host allocator state.

## Stop Line

This selection does not migrate:

- `last_reason`, because it is reason vocabulary;
- `HakoAllocAllocatorComparisonCMimallocExplicitRunnerEvidenceDiagnosticReportFields`
  and `HakoAllocAllocatorComparisonCMimallocExplicitRunnerEvidenceDiagnosticReport`
  fields, because report mirrors stay signed until their own row;
- runner payloads, RSS evidence, result codes, stable output contract flags,
  stop-line flags, repeated C runner execution, process allocator replacement,
  hooks, backend matcher additions, provider package generation, worker/TLS,
  threads, or `#[global_allocator]`.

## Verification

Selection-only row:

```bash
bash tools/checks/current_state_pointer_guard.sh
git diff --check
```
