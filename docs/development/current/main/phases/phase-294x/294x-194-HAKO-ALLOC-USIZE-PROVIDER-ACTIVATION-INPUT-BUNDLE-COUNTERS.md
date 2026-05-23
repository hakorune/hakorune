---
Status: Landed
Date: 2026-05-24
Scope: provider activation input bundle inventory owner-local counter exact `usize` migration.
Blocker: HAKO-ALLOC-USIZE-FIELD-GROUP-192
Related:
  - docs/development/current/main/design/usize-semantic-foundation-ssot.md
  - docs/development/current/main/phases/phase-294x/294x-193-HAKO-ALLOC-USIZE-PROVIDER-ACTIVATION-INPUT-BUNDLE-COUNTER-SELECTION.md
  - docs/development/current/main/phases/phase-294x/294x-90-usize-semantics-taskboard.md
  - lang/src/hako_alloc/memory/NUMERIC_FIELDS.md
  - lang/src/hako_alloc/memory/provider_activation_input_bundle_inventory_box.hako
  - tools/checks/k2_wide_hako_alloc_provider_activation_input_bundle_inventory_guard.sh
---

# 294x-194 Hako Alloc Usize Provider Activation Input Bundle Counters

## Decision

Migrate only the selected owner-local
`HakoAllocProviderActivationInputBundleInventory` counters to exact `usize`
storage:

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

## Stop Line

This row does not migrate:

- `last_reason`;
- `HakoAllocProviderActivationInputBundleInventoryReportFields`;
- `HakoAllocProviderActivationInputBundleInventoryReport`;
- activation request token, activation mode, provider/token/kind payloads, or
  bool-like inactive / unsupported / would-execute flags;
- provider activation, provider calls, host replacement, hooks, global
  allocator install, backend matchers, worker/TLS, atomics, provider package /
  DLL generation, or `#[global_allocator]`.

## Verification

```bash
bash tools/checks/k2_wide_hako_alloc_provider_activation_input_bundle_inventory_guard.sh
bash tools/checks/current_state_pointer_guard.sh
git diff --check
```
