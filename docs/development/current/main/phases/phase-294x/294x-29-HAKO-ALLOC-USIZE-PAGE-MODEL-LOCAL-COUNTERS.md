---
Status: Complete
Date: 2026-05-22
Scope: migrate one page-model local counter field group from `i64` to exact `usize`.
Related:
  - docs/development/current/main/design/usize-semantic-foundation-ssot.md
  - docs/development/current/main/phases/phase-294x/294x-90-usize-semantics-taskboard.md
  - lang/src/hako_alloc/memory/NUMERIC_FIELDS.md
  - lang/src/hako_alloc/memory/page_box.hako
  - tools/checks/k2_wide_mimalloc_page_model_guard.sh
---

# 294x-29 Hako Alloc Usize Page Model Local Counters

## Decision

Migrate only the low-risk `HakoAllocPageModel` page-local stats counter group:

- `alloc_count`
- `local_free_count`
- `reject_count`

These fields are monotonic, non-negative page-local counters. They do not carry
page identity, capacity, stack-top, live-count, lifecycle state, byte-size, or
sentinel meaning.

## Stop Line

`page_id`, `block_size`, `capacity`, `reserved`, `used`, `free_top`,
`local_free_top`, `peak_used`, and `requested_bytes` remain `i64` in this row.
They require index/capacity/decrement/byte-length contracts before exact
migration.

`local_free_collect_count`, `local_free_collected_blocks`, `retired`,
`decommitted`, `retire_count`, `decommit_count`, `recommit_count`,
`reuse_count`, `lifecycle_reject_count`, `reactivate_count`, and
`reactivate_reject_count` also remain `i64`. Collection and lifecycle counters
are split into their own rows because existing lifecycle/local-free guards still
own their signed contracts.

No page queue fields, page-map entry pointer/id fields, provider activation,
host allocator replacement, hooks, or `#[global_allocator]` are opened by this
row.

## Acceptance

```bash
bash tools/checks/k2_wide_mimalloc_page_model_guard.sh
bash tools/checks/current_state_pointer_guard.sh
git diff --check
```
