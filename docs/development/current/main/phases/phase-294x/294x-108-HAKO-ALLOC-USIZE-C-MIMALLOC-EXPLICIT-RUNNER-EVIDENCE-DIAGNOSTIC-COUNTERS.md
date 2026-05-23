---
Status: Landed
Date: 2026-05-23
Scope: explicit C mimalloc runner evidence diagnostic owner-local counter exact `usize` migration.
Related:
  - docs/development/current/main/design/usize-semantic-foundation-ssot.md
  - docs/development/current/main/phases/phase-294x/294x-107-HAKO-ALLOC-USIZE-C-MIMALLOC-EXPLICIT-RUNNER-EVIDENCE-DIAGNOSTIC-COUNTER-SELECTION.md
  - docs/development/current/main/phases/phase-294x/294x-90-usize-semantics-taskboard.md
  - lang/src/hako_alloc/memory/NUMERIC_FIELDS.md
  - lang/src/hako_alloc/memory/allocator_comparison_c_mimalloc_explicit_runner_evidence_diagnostic_box.hako
  - apps/hako-alloc-allocator-comparison-c-mimalloc-explicit-runner-evidence-diagnostics-proof/main.hako
  - tools/checks/k2_wide_hako_alloc_allocator_comparison_c_mimalloc_explicit_runner_evidence_diagnostics_guard.sh
---

# 294x-108 Hako Alloc Usize C Mimalloc Explicit Runner Evidence Diagnostic Counters

## Decision

Migrate only the selected
`HakoAllocAllocatorComparisonCMimallocExplicitRunnerEvidenceDiagnostic`
owner-local monotonic counters to exact `usize` storage:

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

The MIMAP-452A explicit C mimalloc runner evidence diagnostic guard now asserts
these fields are exact `usize` in the typed-object plan.

## Stop Line

This row does not migrate:

- `last_reason`, because it is reason vocabulary;
- report fields and `ReportFields` mirrors, because they remain signed
  comparison payload/mirror seams until their own row;
- runner payloads, RSS evidence, result codes, stable output contract flags,
  repeated C runner execution, process allocator replacement, hooks, backend
  matcher additions, provider package generation, worker/TLS, threads, or
  `#[global_allocator]`.

## Verification

```bash
bash tools/checks/k2_wide_hako_alloc_allocator_comparison_c_mimalloc_explicit_runner_evidence_diagnostics_guard.sh
bash tools/checks/current_state_pointer_guard.sh
git diff --check
```
