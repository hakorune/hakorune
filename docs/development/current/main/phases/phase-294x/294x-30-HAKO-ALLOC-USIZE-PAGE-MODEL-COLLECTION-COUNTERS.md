---
Status: Complete
Date: 2026-05-22
Scope: migrate one page-model local-free collection counter field group from `i64` to exact `usize`.
Related:
  - docs/development/current/main/design/usize-semantic-foundation-ssot.md
  - docs/development/current/main/phases/phase-294x/294x-90-usize-semantics-taskboard.md
  - lang/src/hako_alloc/memory/NUMERIC_FIELDS.md
  - lang/src/hako_alloc/memory/page_box.hako
  - tools/checks/k2_wide_mimalloc_page_model_guard.sh
  - tools/checks/k2_wide_mimalloc_local_free_retire_guard.sh
---

# 294x-30 Hako Alloc Usize Page Model Collection Counters

## Decision

Migrate only the `HakoAllocPageModel` local-free collection counter group:

- `local_free_collect_count`
- `local_free_collected_blocks`

These fields are monotonic, non-negative counters that record local-free
collection events and collected block counts. They do not carry stack-top,
live-count, lifecycle state, page identity, size, capacity, or sentinel
meaning.

## Stop Line

`used`, `free_top`, `local_free_top`, `retired`, `decommitted`,
`retire_count`, `decommit_count`, `recommit_count`, `reuse_count`,
`lifecycle_reject_count`, `reactivate_count`, `reactivate_reject_count`,
`peak_used`, and `requested_bytes` remain `i64` in this row.

The local-free collection operation itself remains the existing page-local
behavior. This row changes only the stored counter exactness and updates guards
that previously asserted the old all-`i64` production-state contract.

No page queue fields, page-map entry pointer/id fields, provider activation,
host allocator replacement, hooks, or `#[global_allocator]` are opened by this
row.

## Acceptance

```bash
bash tools/checks/k2_wide_mimalloc_page_model_guard.sh
bash tools/checks/k2_wide_mimalloc_local_free_retire_guard.sh
bash tools/checks/current_state_pointer_guard.sh
git diff --check
```
