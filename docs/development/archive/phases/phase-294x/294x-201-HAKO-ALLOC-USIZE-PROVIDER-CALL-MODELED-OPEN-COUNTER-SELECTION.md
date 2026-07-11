---
Status: Landed
Date: 2026-05-24
Scope: select the next exact `usize` production field group.
Blocker: HAKO-ALLOC-USIZE-FIELD-GROUP-199
Related:
  - docs/development/current/main/design/usize-semantic-foundation-ssot.md
  - docs/development/current/main/phases/phase-294x/294x-90-usize-semantics-taskboard.md
  - lang/src/hako_alloc/memory/NUMERIC_FIELDS.md
  - lang/src/hako_alloc/memory/provider_call_modeled_open_pilot_box.hako
  - tools/checks/k2_wide_hako_alloc_provider_call_modeled_open_pilot_guard.sh
---

# 294x-201 Hako Alloc Usize Provider Call Modeled Open Counter Selection

## Decision

Select the owner-local `HakoAllocProviderCallModeledOpenPilot` counters as
`HAKO-ALLOC-USIZE-FIELD-GROUP-200`:

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

These fields are monotonic modeled-open inventory/reject counters initialized to
`0`. The selected group records the model-open provider-call seam only; actual
provider calls, host replacement, hooks, and backend matcher installation stay
closed.

## Stop Line

Do not migrate:

- `last_reason`;
- `HakoAllocProviderCallModeledOpenPilotReportFields`;
- `HakoAllocProviderCallModeledOpenPilotReport`;
- capability-present / capability-valid flags, modeled-open payloads, or
  bool-like inactive / would-execute flags;
- provider calls, host replacement, hooks, global allocator install, backend
  matchers, worker/TLS, atomics, provider package / DLL generation, or
  `#[global_allocator]`.

## Next Row

`HAKO-ALLOC-USIZE-FIELD-GROUP-200` should migrate only the selected owner-local
counters and update the provider-call modeled-open guard to assert exact
`usize` storage while report mirrors remain signed.

## Verification

Docs-only selection row:

```bash
bash tools/checks/current_state_pointer_guard.sh
git diff --check
```
