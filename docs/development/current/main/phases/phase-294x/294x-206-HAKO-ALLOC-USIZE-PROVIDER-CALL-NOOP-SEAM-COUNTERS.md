---
Status: Landed
Date: 2026-05-24
Scope: provider call no-op execution seam owner-local counter exact `usize` migration.
Blocker: HAKO-ALLOC-USIZE-FIELD-GROUP-204
Related:
  - docs/development/current/main/design/usize-semantic-foundation-ssot.md
  - docs/development/current/main/phases/phase-294x/294x-205-HAKO-ALLOC-USIZE-PROVIDER-CALL-NOOP-SEAM-COUNTER-SELECTION.md
  - docs/development/current/main/phases/phase-294x/294x-90-usize-semantics-taskboard.md
  - lang/src/hako_alloc/memory/NUMERIC_FIELDS.md
  - lang/src/hako_alloc/memory/provider_call_noop_execution_seam_pilot_box.hako
  - tools/checks/k2_wide_hako_alloc_provider_call_noop_execution_seam_pilot_guard.sh
---

# 294x-206 Hako Alloc Usize Provider Call No-Op Seam Counters

## Decision

Migrate only the selected owner-local
`HakoAllocProviderCallNoopExecutionSeamPilot` counters to exact `usize`
storage:

- `seam_count`
- `accepted_count`
- `reject_count`
- `missing_preflight_reject_count`
- `rejected_preflight_reject_count`
- `not_ready_reject_count`
- `closed_execution_reject_count`
- `closed_host_replacement_reject_count`
- `closed_hook_reject_count`
- `closed_backend_matcher_reject_count`

## Stop Line

This row does not migrate:

- `last_reason`;
- `HakoAllocProviderCallNoopExecutionSeamPilotReportFields`;
- `HakoAllocProviderCallNoopExecutionSeamPilotReport`;
- no-op/open/executed flags, preflight payloads, provider API call flags, or
  bool-like readiness / would-execute flags;
- actual provider calls, host replacement, hooks, global allocator install,
  backend matchers, worker/TLS, atomics, provider package / DLL generation, or
  `#[global_allocator]`.

## Verification

```bash
bash tools/checks/k2_wide_hako_alloc_provider_call_noop_execution_seam_pilot_guard.sh
bash tools/checks/current_state_pointer_guard.sh
git diff --check
```
