---
Status: Complete
Date: 2026-05-23
Scope: migrate huge-page model metadata mirror counters from `i64` to exact `usize`.
Related:
  - docs/development/current/main/design/usize-semantic-foundation-ssot.md
  - docs/development/current/main/phases/phase-294x/294x-90-usize-semantics-taskboard.md
  - docs/development/current/main/phases/phase-294x/294x-33-HAKO-ALLOC-USIZE-HUGE-META-STORE-COUNTERS.md
  - lang/src/hako_alloc/memory/NUMERIC_FIELDS.md
  - lang/src/hako_alloc/memory/huge_page_model_box.hako
  - tools/checks/k2_wide_mimalloc_huge_page_model_guard.sh
  - tools/checks/k2_wide_mimalloc_facade_huge_page_model_exe_guard.sh
---

# 294x-34 Hako Alloc Usize Huge Model Meta Mirrors

## Decision

Migrate the huge-page model metadata mirrors:

- `HakoAllocHugePageModel.huge_count`
- `HakoAllocHugePageModel.live_count`

These fields mirror the C205d `HakoAllocHugePageMetaStore.count` /
`live_count` owner truth, which became exact `usize` in `294x-33`. Migrating
the mirrors removes the exact-to-signed owner seam while keeping route/report
presentation fields unchanged.

## Stop Line

`allocate_count`, `release_count`, `release_reject_count`,
`zero_reject_count`, `commit_reject_count`, `register_fail_count`, and
`reject_count` remain `i64` until the huge-model event/reject counter row.
`next_page_id`, `next_ptr`, `last_result_ptr`, `last_page_id`,
`last_requested_size`, `last_committed_size`, and `last_failure_kind` remain
signed pointer/id/size/status observers. This row does not migrate facade
report fields, huge release seam counters, OSVM, provider activation, host
allocator replacement, hooks, or `#[global_allocator]`.

## Acceptance

```bash
bash tools/checks/k2_wide_mimalloc_huge_page_model_guard.sh
bash tools/checks/k2_wide_mimalloc_facade_huge_page_model_exe_guard.sh
bash tools/checks/current_state_pointer_guard.sh
git diff --check
```
