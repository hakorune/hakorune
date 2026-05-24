---
Status: Landed
Date: 2026-05-24
Scope: provider call modeled-open pilot owner-local counter exact `usize` migration.
Blocker: HAKO-ALLOC-USIZE-FIELD-GROUP-200
Related:
  - docs/development/current/main/design/usize-semantic-foundation-ssot.md
  - docs/development/current/main/phases/phase-294x/294x-201-HAKO-ALLOC-USIZE-PROVIDER-CALL-MODELED-OPEN-COUNTER-SELECTION.md
  - docs/development/current/main/phases/phase-294x/294x-90-usize-semantics-taskboard.md
  - lang/src/hako_alloc/memory/NUMERIC_FIELDS.md
  - lang/src/hako_alloc/memory/provider_call_modeled_open_pilot_box.hako
  - tools/checks/k2_wide_hako_alloc_provider_call_modeled_open_pilot_guard.sh
---

# 294x-202 Hako Alloc Usize Provider Call Modeled Open Counters

## Decision

Migrate only the selected owner-local
`HakoAllocProviderCallModeledOpenPilot` counters to exact `usize` storage:

- `modeled_open_count`
- `accepted_count`
- `reject_count`
- `missing_dry_run_reject_count`
- `rejected_dry_run_reject_count`
- `missing_capability_reject_count`
- `invalid_capability_reject_count`
- `unsupported_outcome_reject_count`
- `closed_call_reject_count`
- `closed_host_replacement_reject_count`
- `closed_hook_reject_count`
- `closed_backend_matcher_reject_count`

## Stop Line

This row does not migrate:

- `last_reason`;
- `HakoAllocProviderCallModeledOpenPilotReportFields`;
- `HakoAllocProviderCallModeledOpenPilotReport`;
- capability-present / capability-valid flags, modeled-open payloads, or
  bool-like inactive / would-execute flags;
- provider calls, host replacement, hooks, global allocator install, backend
  matchers, worker/TLS, atomics, provider package / DLL generation, or
  `#[global_allocator]`.

## Verification

```bash
bash tools/checks/k2_wide_hako_alloc_provider_call_modeled_open_pilot_guard.sh
bash tools/checks/current_state_pointer_guard.sh
git diff --check
```
