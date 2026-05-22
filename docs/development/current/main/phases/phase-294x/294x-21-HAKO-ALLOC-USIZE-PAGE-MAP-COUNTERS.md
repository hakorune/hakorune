---
Status: Complete
Date: 2026-05-22
Scope: migrate one owner-local page-map counter field group from `i64` to
  exact `usize`.
Related:
  - docs/development/current/main/design/usize-semantic-foundation-ssot.md
  - docs/development/current/main/phases/phase-294x/294x-90-usize-semantics-taskboard.md
  - lang/src/hako_alloc/memory/NUMERIC_FIELDS.md
  - lang/src/hako_alloc/memory/page_map_box.hako
  - tools/checks/k2_wide_mimalloc_page_map_guard.sh
---

# 294x-21 Hako Alloc Usize Page-Map Counters

## Decision

Migrate only the owner-local `HakoAllocPageMap` counter group:

- `entry_count`
- `live_count`
- `register_count`
- `lookup_count`
- `lookup_miss_count`
- `unregister_count`
- `reject_count`

These fields are non-negative counters owned by `HakoAllocPageMap`. They do not
carry signed sentinels and do not represent pointer identity, page ids, block
ids, or result status.

## Contract

- The seven page-map counters are declared as `usize`.
- Existing page-map proof output remains behaviorally identical.
- `HakoAllocPageMapEntry.ptr`, `page_id`, `block_id`, and `live` remain `i64`.
- Page-map release, realloc, huge, aligned, and provider rows are not widened
  by this card.
- Allocator-provider activation, host allocator replacement, hooks, and
  `#[global_allocator]` stay out of scope.

## Acceptance

```bash
bash tools/checks/k2_wide_mimalloc_page_map_guard.sh
bash tools/checks/current_state_pointer_guard.sh
git diff --check
```
