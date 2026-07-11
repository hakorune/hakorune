---
Status: Landed
Date: 2026-05-24
Scope: select the next exact `usize` production field group.
Blocker: HAKO-ALLOC-USIZE-FIELD-GROUP-187
Related:
  - docs/development/current/main/design/usize-semantic-foundation-ssot.md
  - docs/development/current/main/phases/phase-294x/294x-90-usize-semantics-taskboard.md
  - lang/src/hako_alloc/memory/NUMERIC_FIELDS.md
  - lang/src/hako_alloc/memory/provider_selection_inventory_box.hako
  - tools/checks/k2_wide_hako_alloc_provider_selection_inventory_guard.sh
---

# 294x-189 Hako Alloc Usize Provider Selection Inventory Counter Selection

## Decision

Select the owner-local `HakoAllocProviderSelectionInventory` counters as
`HAKO-ALLOC-USIZE-FIELD-GROUP-188`:

- `selection_count`
- `accepted_count`
- `reject_count`
- `missing_readiness_reject_count`
- `rejected_readiness_reject_count`
- `invalid_readiness_token_reject_count`
- `invalid_candidate_token_reject_count`
- `invalid_provider_kind_reject_count`
- `closed_execution_reject_count`

These fields are monotonic counters initialized to `0` and incremented only by
the inventory/reject paths. They prepare the provider-facing ladder without
opening provider activation.

## Stop Line

Do not migrate:

- `HakoAllocProviderSelectionInventory.last_reason`;
- `HakoAllocProviderSelectionInventoryReportFields`;
- `HakoAllocProviderSelectionInventoryReport`;
- readiness tokens, candidate tokens, provider kind vocabulary, or bool-like
  inactive / would-execute flags;
- provider activation, host replacement, hooks, global allocator install,
  worker/TLS, atomics, provider package / DLL generation, or
  `#[global_allocator]`.

## Next Row

`HAKO-ALLOC-USIZE-FIELD-GROUP-188` should migrate only the selected owner-local
counters and update the provider-selection inventory guard to assert exact
`usize` storage while report mirrors remain signed.

## Verification

Docs-only selection row:

```bash
bash tools/checks/current_state_pointer_guard.sh
git diff --check
```
