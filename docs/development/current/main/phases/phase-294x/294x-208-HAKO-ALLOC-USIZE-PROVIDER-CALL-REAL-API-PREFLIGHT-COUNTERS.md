---
Status: Landed
Date: 2026-05-24
Scope: provider call real API execution preflight owner-local counter exact `usize` migration.
Blocker: HAKO-ALLOC-USIZE-FIELD-GROUP-206
Related:
  - docs/development/current/main/design/usize-semantic-foundation-ssot.md
  - docs/development/current/main/phases/phase-294x/294x-207-HAKO-ALLOC-USIZE-PROVIDER-CALL-REAL-API-PREFLIGHT-COUNTER-SELECTION.md
  - docs/development/current/main/phases/phase-294x/294x-90-usize-semantics-taskboard.md
  - lang/src/hako_alloc/memory/NUMERIC_FIELDS.md
  - lang/src/hako_alloc/memory/provider_call_real_api_execution_preflight_box.hako
  - tools/checks/k2_wide_hako_alloc_provider_call_real_api_execution_preflight_guard.sh
---

# 294x-208 Hako Alloc Usize Provider Call Real API Preflight Counters

## Decision

Migrate only the selected owner-local
`HakoAllocProviderCallRealApiExecutionPreflight` counters to exact `usize`
storage:

- `preflight_count`
- `accepted_count`
- `reject_count`
- `missing_noop_reject_count`
- `rejected_noop_reject_count`
- `missing_capability_reject_count`
- `invalid_capability_reject_count`
- `already_executed_reject_count`
- `closed_execution_reject_count`
- `closed_host_replacement_reject_count`
- `closed_hook_reject_count`
- `closed_backend_matcher_reject_count`

## Stop Line

This row does not migrate:

- `last_reason`;
- `HakoAllocProviderCallRealApiExecutionPreflightReportFields`;
- `HakoAllocProviderCallRealApiExecutionPreflightReport`;
- real API preflight payloads, capability flags, provider API call flags, or
  bool-like readiness / would-execute flags;
- actual provider calls, host replacement, hooks, global allocator install,
  backend matchers, worker/TLS, atomics, provider package / DLL generation, or
  `#[global_allocator]`.

## Verification

```bash
bash tools/checks/k2_wide_hako_alloc_provider_call_real_api_execution_preflight_guard.sh
bash tools/checks/current_state_pointer_guard.sh
git diff --check
```
