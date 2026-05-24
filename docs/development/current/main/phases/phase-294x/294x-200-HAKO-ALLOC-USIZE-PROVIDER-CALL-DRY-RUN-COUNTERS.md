---
Status: Landed
Date: 2026-05-24
Scope: provider call dry-run unsupported behavior owner-local counter exact `usize` migration.
Blocker: HAKO-ALLOC-USIZE-FIELD-GROUP-198
Related:
  - docs/development/current/main/design/usize-semantic-foundation-ssot.md
  - docs/development/current/main/phases/phase-294x/294x-199-HAKO-ALLOC-USIZE-PROVIDER-CALL-DRY-RUN-COUNTER-SELECTION.md
  - docs/development/current/main/phases/phase-294x/294x-90-usize-semantics-taskboard.md
  - lang/src/hako_alloc/memory/NUMERIC_FIELDS.md
  - lang/src/hako_alloc/memory/provider_call_dry_run_unsupported_behavior_box.hako
  - tools/checks/k2_wide_hako_alloc_provider_call_dry_run_unsupported_behavior_guard.sh
---

# 294x-200 Hako Alloc Usize Provider Call Dry Run Counters

## Decision

Migrate only the selected owner-local
`HakoAllocProviderCallDryRunUnsupportedBehavior` counters to exact `usize`
storage:

- `dry_run_count`
- `accepted_count`
- `reject_count`
- `missing_gate_reject_count`
- `rejected_gate_reject_count`
- `closed_execution_reject_count`

## Stop Line

This row does not migrate:

- `last_reason`;
- `HakoAllocProviderCallDryRunUnsupportedBehaviorReportFields`;
- `HakoAllocProviderCallDryRunUnsupportedBehaviorReport`;
- capability-present / capability-valid flags, dry-run payloads, or bool-like
  inactive / would-execute flags;
- provider calls, host replacement, hooks, global allocator install, backend
  matchers, worker/TLS, atomics, provider package / DLL generation, or
  `#[global_allocator]`.

## Verification

```bash
bash tools/checks/k2_wide_hako_alloc_provider_call_dry_run_unsupported_behavior_guard.sh
bash tools/checks/current_state_pointer_guard.sh
git diff --check
```
