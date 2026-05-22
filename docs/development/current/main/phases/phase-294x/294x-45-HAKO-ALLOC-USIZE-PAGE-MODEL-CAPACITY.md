---
Status: Landed
Date: 2026-05-23
Scope: production page-model capacity/reserved exact `usize` migration.
Related:
  - lang/src/hako_alloc/memory/page_box.hako
  - tools/checks/k2_wide_mimalloc_page_model_guard.sh
  - lang/src/hako_alloc/memory/NUMERIC_FIELDS.md
  - docs/development/current/main/design/usize-semantic-foundation-ssot.md
---

# 294x-45 Hako Alloc Usize Page Model Capacity

## Decision

Migrate only the `HakoAllocPageModel` capacity field group to exact `usize`
storage:

- `capacity`;
- `reserved`.

This follows the proof-only `294x-44` capacity-bound row, which fixed
current-lane signed index guards and `loop(i < capacity)` checks before
production capacity migration.

## Stop Line

This row does not migrate page identity, block size, requested byte accounting,
queue indexes, remote-free mailbox fields, provider activation, host allocator
replacement, hooks, or global allocator integration.

`HakoAllocPageModel.page_id`, `block_size`, `requested_bytes`, `retired`, and
`decommitted` remain `i64` in this row.

## Verification

```bash
bash tools/checks/k2_wide_mimalloc_page_model_guard.sh
bash tools/checks/k2_wide_mimalloc_local_free_retire_guard.sh
bash tools/checks/k2_wide_mimalloc_remote_free_page_integration_guard.sh
bash tools/checks/k2_wide_mimalloc_pre_realloc_release_invariant_guard.sh
bash tools/checks/current_state_pointer_guard.sh
git diff --check
```
