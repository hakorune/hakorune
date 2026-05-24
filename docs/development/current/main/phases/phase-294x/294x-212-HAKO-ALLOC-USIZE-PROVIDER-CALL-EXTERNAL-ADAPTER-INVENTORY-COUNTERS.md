---
Status: Landed
Date: 2026-05-24
Scope: provider call external API adapter inventory owner-local counter exact `usize` migration.
Blocker: HAKO-ALLOC-USIZE-FIELD-GROUP-210
Related:
  - docs/development/current/main/design/usize-semantic-foundation-ssot.md
  - docs/development/current/main/phases/phase-294x/294x-211-HAKO-ALLOC-USIZE-PROVIDER-CALL-EXTERNAL-ADAPTER-INVENTORY-COUNTER-SELECTION.md
  - docs/development/current/main/phases/phase-294x/294x-90-usize-semantics-taskboard.md
  - lang/src/hako_alloc/memory/NUMERIC_FIELDS.md
  - lang/src/hako_alloc/memory/provider_call_external_api_adapter_inventory_box.hako
  - tools/checks/k2_wide_hako_alloc_provider_call_external_api_adapter_inventory_guard.sh
---

# 294x-212 Hako Alloc Usize Provider Call External Adapter Inventory Counters

## Decision

Migrate only the selected owner-local
`HakoAllocProviderCallExternalApiAdapterInventory` counters to exact `usize`
storage:

- `inventory_count`
- `accepted_count`
- `reject_count`
- `missing_stub_reject_count`
- `rejected_stub_reject_count`
- `missing_adapter_reject_count`
- `invalid_adapter_reject_count`
- `already_executed_reject_count`
- `closed_execution_reject_count`
- `closed_host_replacement_reject_count`
- `closed_hook_reject_count`
- `closed_backend_matcher_reject_count`

## Stop Line

This row does not migrate:

- `last_reason`;
- `HakoAllocProviderCallExternalApiAdapterInventoryReportFields`;
- `HakoAllocProviderCallExternalApiAdapterInventoryReport`;
- adapter payloads, external API readiness / executed flags, or bool-like
  would-execute flags;
- actual provider calls, host replacement, hooks, global allocator install,
  backend matchers, worker/TLS, atomics, provider package / DLL generation, or
  `#[global_allocator]`.

## Verification

```bash
bash tools/checks/k2_wide_hako_alloc_provider_call_external_api_adapter_inventory_guard.sh
bash tools/checks/current_state_pointer_guard.sh
git diff --check
```
