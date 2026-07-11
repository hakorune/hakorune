---
Status: Landed
Date: 2026-05-24
Scope: select the next exact `usize` production field group.
Blocker: HAKO-ALLOC-USIZE-FIELD-GROUP-205
Related:
  - docs/development/current/main/design/usize-semantic-foundation-ssot.md
  - docs/development/current/main/phases/phase-294x/294x-90-usize-semantics-taskboard.md
  - lang/src/hako_alloc/memory/NUMERIC_FIELDS.md
  - lang/src/hako_alloc/memory/provider_call_real_api_execution_preflight_box.hako
  - tools/checks/k2_wide_hako_alloc_provider_call_real_api_execution_preflight_guard.sh
---

# 294x-207 Hako Alloc Usize Provider Call Real API Preflight Counter Selection

## Decision

Select the owner-local `HakoAllocProviderCallRealApiExecutionPreflight`
counters as `HAKO-ALLOC-USIZE-FIELD-GROUP-206`:

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

These fields are monotonic real-provider-API preflight/reject counters
initialized to `0`. The selected group records readiness for a future real
provider API call only; actual provider API calls, host replacement, hooks,
backend matcher installation, worker/TLS, and global allocator install remain
closed.

## Stop Line

Do not migrate:

- `last_reason`;
- `HakoAllocProviderCallRealApiExecutionPreflightReportFields`;
- `HakoAllocProviderCallRealApiExecutionPreflightReport`;
- real API preflight payloads, capability flags, provider API call flags, or
  bool-like readiness / would-execute flags;
- actual provider calls, host replacement, hooks, global allocator install,
  backend matchers, worker/TLS, atomics, provider package / DLL generation, or
  `#[global_allocator]`.

## Next Row

`HAKO-ALLOC-USIZE-FIELD-GROUP-206` should migrate only the selected owner-local
counters and update the provider-call real API execution preflight guard to
assert exact `usize` storage while report mirrors remain signed.

## Verification

Docs-only selection row:

```bash
bash tools/checks/current_state_pointer_guard.sh
git diff --check
```
