---
Status: Landed
Date: 2026-05-24
Scope: select the next exact `usize` production field group.
Blocker: HAKO-ALLOC-USIZE-FIELD-GROUP-201
Related:
  - docs/development/current/main/design/usize-semantic-foundation-ssot.md
  - docs/development/current/main/phases/phase-294x/294x-90-usize-semantics-taskboard.md
  - lang/src/hako_alloc/memory/NUMERIC_FIELDS.md
  - lang/src/hako_alloc/memory/provider_call_execution_capability_preflight_box.hako
  - tools/checks/k2_wide_hako_alloc_provider_call_execution_capability_preflight_guard.sh
---

# 294x-203 Hako Alloc Usize Provider Call Execution Preflight Counter Selection

## Decision

Select the owner-local `HakoAllocProviderCallExecutionCapabilityPreflight`
counters as `HAKO-ALLOC-USIZE-FIELD-GROUP-202`:

- `preflight_count`
- `accepted_count`
- `reject_count`
- `missing_model_reject_count`
- `inactive_model_reject_count`
- `missing_capability_reject_count`
- `invalid_capability_reject_count`
- `closed_execution_reject_count`
- `closed_host_replacement_reject_count`
- `closed_hook_reject_count`
- `closed_backend_matcher_reject_count`

These fields are monotonic provider-call execution preflight/reject counters
initialized to `0`. The selected group records the capability preflight before a
provider-call execution seam only; real provider calls, host replacement,
hooks, and backend matcher installation remain closed.

## Stop Line

Do not migrate:

- `last_reason`;
- `HakoAllocProviderCallExecutionCapabilityPreflightReportFields`;
- `HakoAllocProviderCallExecutionCapabilityPreflightReport`;
- capability-present / capability-valid flags, preflight payloads, or bool-like
  readiness / would-execute flags;
- provider calls, host replacement, hooks, global allocator install, backend
  matchers, worker/TLS, atomics, provider package / DLL generation, or
  `#[global_allocator]`.

## Next Row

`HAKO-ALLOC-USIZE-FIELD-GROUP-202` should migrate only the selected owner-local
counters and update the provider-call execution preflight guard to assert exact
`usize` storage while report mirrors remain signed.

## Verification

Docs-only selection row:

```bash
bash tools/checks/current_state_pointer_guard.sh
git diff --check
```
