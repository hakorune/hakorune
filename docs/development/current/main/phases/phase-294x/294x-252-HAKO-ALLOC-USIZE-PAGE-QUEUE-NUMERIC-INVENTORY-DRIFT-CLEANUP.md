---
Status: Landed
Date: 2026-05-24
Scope: synchronize stale page-queue numeric inventory rows with already-exact storage.
Blocker: HAKO-ALLOC-USIZE-NUMERIC-INVENTORY-PAGE-QUEUE-DRIFT-CLEANUP-001
Related:
  - docs/development/current/main/phases/phase-294x/294x-251-HAKO-ALLOC-USIZE-NEXT-FIELD-GROUP-SELECTION.md
  - lang/src/hako_alloc/memory/page_queue_box.hako
  - lang/src/hako_alloc/memory/NUMERIC_FIELDS.md
  - tools/checks/k2_wide_hako_alloc_usize_page_queue_numeric_inventory_guard.sh
---

# 294x-252 Hako Alloc Usize Page Queue Numeric Inventory Drift Cleanup

## Decision

Close:

```text
HAKO-ALLOC-USIZE-NUMERIC-INVENTORY-PAGE-QUEUE-DRIFT-CLEANUP-001
```

The detailed `NUMERIC_FIELDS.md` rows now match the existing
`HakoAllocPageQueue` storage and summary inventory.

## Implementation

No `.hako` behavior changed.

Updated detailed numeric inventory rows for:

- `bin: usize` via `HAKO-ALLOC-USIZE-FIELD-GROUP-093`;
- `add_count: usize`, `select_count: usize`, `direct_hit_count: usize`,
  `refresh_count: usize`, and `reject_count: usize` via
  `HAKO-ALLOC-USIZE-FIELD-GROUP-058`;
- `has_direct_page: i64` as a signed binary flag that stays out of the `usize`
  lane until bool/flag storage gets a dedicated row.

Added a narrow guard that checks the `.hako` storage and detailed inventory
rows stay aligned.

## Stop Line

This row does not migrate any new field. It does not change queue behavior,
page selection semantics, direct-page cache behavior, ids/indexes beyond
already-exact storage, bool/flag storage, provider/DLL seams, hooks,
worker/TLS, atomics, or `#[global_allocator]`.

## Next Row

Return to explicit field-group selection:

```text
HAKO-ALLOC-USIZE-FIELD-GROUP-NEXT-SELECTION-003
```

## Verification

```bash
bash tools/checks/k2_wide_hako_alloc_usize_page_queue_numeric_inventory_guard.sh
git diff --check
bash tools/checks/current_state_pointer_guard.sh
```
