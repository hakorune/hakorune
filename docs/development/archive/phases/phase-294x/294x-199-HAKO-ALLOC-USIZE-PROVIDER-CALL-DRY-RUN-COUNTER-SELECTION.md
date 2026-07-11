---
Status: Landed
Date: 2026-05-24
Scope: select the next exact `usize` production field group.
Blocker: HAKO-ALLOC-USIZE-FIELD-GROUP-197
Related:
  - docs/development/current/main/design/usize-semantic-foundation-ssot.md
  - docs/development/current/main/phases/phase-294x/294x-90-usize-semantics-taskboard.md
  - lang/src/hako_alloc/memory/NUMERIC_FIELDS.md
  - lang/src/hako_alloc/memory/provider_call_dry_run_unsupported_behavior_box.hako
  - tools/checks/k2_wide_hako_alloc_provider_call_dry_run_unsupported_behavior_guard.sh
---

# 294x-199 Hako Alloc Usize Provider Call Dry Run Counter Selection

## Decision

Select the owner-local `HakoAllocProviderCallDryRunUnsupportedBehavior`
counters as `HAKO-ALLOC-USIZE-FIELD-GROUP-198`:

- `dry_run_count`
- `accepted_count`
- `reject_count`
- `missing_gate_reject_count`
- `rejected_gate_reject_count`
- `closed_execution_reject_count`

These fields are monotonic provider-call dry-run inventory/reject counters
initialized to `0`. The selected group records the unsupported dry-run model
after the provider-call capability gate only and keeps real provider calls
closed.

## Stop Line

Do not migrate:

- `last_reason`;
- `HakoAllocProviderCallDryRunUnsupportedBehaviorReportFields`;
- `HakoAllocProviderCallDryRunUnsupportedBehaviorReport`;
- capability-present / capability-valid flags, dry-run payloads, or bool-like
  inactive / would-execute flags;
- provider calls, host replacement, hooks, global allocator install, backend
  matchers, worker/TLS, atomics, provider package / DLL generation, or
  `#[global_allocator]`.

## Next Row

`HAKO-ALLOC-USIZE-FIELD-GROUP-198` should migrate only the selected owner-local
counters and update the provider-call dry-run guard to assert exact `usize`
storage while report mirrors remain signed.

## Verification

Docs-only selection row:

```bash
bash tools/checks/current_state_pointer_guard.sh
git diff --check
```
