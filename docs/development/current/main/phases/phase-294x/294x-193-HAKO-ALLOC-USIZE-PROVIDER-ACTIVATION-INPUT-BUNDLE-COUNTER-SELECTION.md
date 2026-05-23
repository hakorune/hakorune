---
Status: Landed
Date: 2026-05-24
Scope: select the next exact `usize` production field group.
Blocker: HAKO-ALLOC-USIZE-FIELD-GROUP-191
Related:
  - docs/development/current/main/design/usize-semantic-foundation-ssot.md
  - docs/development/current/main/phases/phase-294x/294x-90-usize-semantics-taskboard.md
  - lang/src/hako_alloc/memory/NUMERIC_FIELDS.md
  - lang/src/hako_alloc/memory/provider_activation_input_bundle_inventory_box.hako
  - tools/checks/k2_wide_hako_alloc_provider_activation_input_bundle_inventory_guard.sh
---

# 294x-193 Hako Alloc Usize Provider Activation Input Bundle Counter Selection

## Decision

Select the owner-local `HakoAllocProviderActivationInputBundleInventory`
counters as `HAKO-ALLOC-USIZE-FIELD-GROUP-192`:

- `bundle_count`
- `accepted_count`
- `reject_count`
- `missing_outcome_reject_count`
- `rejected_outcome_reject_count`
- `invalid_candidate_reject_count`
- `invalid_kind_reject_count`
- `invalid_request_token_reject_count`
- `invalid_mode_reject_count`
- `unsupported_evidence_reject_count`
- `closed_execution_reject_count`

These fields are monotonic inventory/reject counters initialized to `0`. The
selected group only records explicit provider activation input bundle
inventory and keeps provider activation closed.

## Stop Line

Do not migrate:

- `last_reason`;
- `HakoAllocProviderActivationInputBundleInventoryReportFields`;
- `HakoAllocProviderActivationInputBundleInventoryReport`;
- activation request token, activation mode, provider/token/kind payloads, or
  bool-like inactive / unsupported / would-execute flags;
- provider activation, provider calls, host replacement, hooks, global
  allocator install, backend matchers, worker/TLS, atomics, provider package /
  DLL generation, or `#[global_allocator]`.

## Next Row

`HAKO-ALLOC-USIZE-FIELD-GROUP-192` should migrate only the selected owner-local
counters and update the input bundle inventory guard to assert exact `usize`
storage while report mirrors remain signed.

## Verification

Docs-only selection row:

```bash
bash tools/checks/current_state_pointer_guard.sh
git diff --check
```
