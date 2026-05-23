---
Status: Landed
Date: 2026-05-24
Scope: select the next exact `usize` production field group.
Blocker: HAKO-ALLOC-USIZE-FIELD-GROUP-193
Related:
  - docs/development/current/main/design/usize-semantic-foundation-ssot.md
  - docs/development/current/main/phases/phase-294x/294x-90-usize-semantics-taskboard.md
  - lang/src/hako_alloc/memory/NUMERIC_FIELDS.md
  - lang/src/hako_alloc/memory/provider_activation_modeled_open_pilot_box.hako
  - tools/checks/k2_wide_hako_alloc_provider_activation_modeled_open_pilot_guard.sh
---

# 294x-195 Hako Alloc Usize Provider Activation Modeled-Open Counter Selection

## Decision

Select the owner-local `HakoAllocProviderActivationModeledOpenPilot` counters
as `HAKO-ALLOC-USIZE-FIELD-GROUP-194`:

- `modeled_open_count`
- `accepted_count`
- `reject_count`
- `missing_dry_run_reject_count`
- `rejected_dry_run_reject_count`
- `invalid_request_token_reject_count`
- `invalid_mode_reject_count`
- `closed_call_reject_count`
- `closed_host_replacement_reject_count`
- `closed_hook_reject_count`
- `closed_backend_matcher_reject_count`

These fields are monotonic modeled-open/reject counters initialized to `0`.
The selected group records modeled provider activation-open state only; it does
not open provider calls, host replacement, hooks, backend matchers, or process
allocator installation.

## Stop Line

Do not migrate:

- `last_reason`;
- `HakoAllocProviderActivationModeledOpenPilotReportFields`;
- `HakoAllocProviderActivationModeledOpenPilotReport`;
- activation request token, activation mode, provider/token/kind payloads, or
  bool-like inactive / modeled-open / would-execute flags;
- provider calls, host replacement, hooks, global allocator install, backend
  matchers, worker/TLS, atomics, provider package / DLL generation, or
  `#[global_allocator]`.

## Next Row

`HAKO-ALLOC-USIZE-FIELD-GROUP-194` should migrate only the selected owner-local
counters and update the modeled-open guard to assert exact `usize` storage
while report mirrors remain signed.

## Verification

Docs-only selection row:

```bash
bash tools/checks/current_state_pointer_guard.sh
git diff --check
```
