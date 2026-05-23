---
Status: Landed
Date: 2026-05-24
Scope: provider selection inventory owner-local counter exact `usize` migration.
Blocker: HAKO-ALLOC-USIZE-FIELD-GROUP-188
Related:
  - docs/development/current/main/design/usize-semantic-foundation-ssot.md
  - docs/development/current/main/phases/phase-294x/294x-189-HAKO-ALLOC-USIZE-PROVIDER-SELECTION-INVENTORY-COUNTER-SELECTION.md
  - docs/development/current/main/phases/phase-294x/294x-90-usize-semantics-taskboard.md
  - lang/src/hako_alloc/memory/NUMERIC_FIELDS.md
  - lang/src/hako_alloc/memory/provider_selection_inventory_box.hako
  - tools/checks/k2_wide_hako_alloc_provider_selection_inventory_guard.sh
---

# 294x-190 Hako Alloc Usize Provider Selection Inventory Counters

## Decision

Migrate only the selected owner-local `HakoAllocProviderSelectionInventory`
counters to exact `usize` storage:

- `selection_count`
- `accepted_count`
- `reject_count`
- `missing_readiness_reject_count`
- `rejected_readiness_reject_count`
- `invalid_readiness_token_reject_count`
- `invalid_candidate_token_reject_count`
- `invalid_provider_kind_reject_count`
- `closed_execution_reject_count`

## Stop Line

This row does not migrate:

- `last_reason`;
- `HakoAllocProviderSelectionInventoryReportFields`;
- `HakoAllocProviderSelectionInventoryReport`;
- readiness tokens, candidate tokens, provider kind vocabulary, or bool-like
  inactive / would-execute flags;
- provider activation, host replacement, hooks, global allocator install,
  worker/TLS, atomics, provider package / DLL generation, or
  `#[global_allocator]`.

## Verification

```bash
bash tools/checks/k2_wide_hako_alloc_provider_selection_inventory_guard.sh
bash tools/checks/current_state_pointer_guard.sh
git diff --check
```
