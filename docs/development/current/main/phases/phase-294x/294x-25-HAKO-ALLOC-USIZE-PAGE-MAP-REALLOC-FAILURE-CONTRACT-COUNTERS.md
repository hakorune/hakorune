---
Status: Complete
Date: 2026-05-22
Scope: migrate one page-map realloc failure-contract counter field group from
  `i64` to exact `usize`.
Related:
  - docs/development/current/main/design/usize-semantic-foundation-ssot.md
  - docs/development/current/main/phases/phase-294x/294x-24-HAKO-ALLOC-USIZE-PAGE-MAP-REALLOC-ALLOC-COPY-RELEASE-COUNTERS.md
  - docs/development/current/main/phases/phase-294x/294x-90-usize-semantics-taskboard.md
  - lang/src/hako_alloc/memory/NUMERIC_FIELDS.md
  - lang/src/hako_alloc/memory/page_map_realloc_failure_contract_box.hako
  - tools/checks/k2_wide_mimalloc_realloc_failure_contract_guard.sh
---

# 294x-25 Hako Alloc Usize Page-Map Realloc Failure-Contract Counters

## Decision

Migrate only the `HakoAllocPageMapReallocFailureContract` event/reject counter
group:

- `success_count`
- `same_class_success_count`
- `move_success_count`
- `zero_reject_count`
- `oversized_reject_count`
- `alloc_fail_count`
- `lookup_miss_count`
- `stale_page_count`
- `released_block_count`
- `unexpected_reject_count`
- `reject_count`

These fields are non-negative counters owned by the failure-contract wrapper.
They classify existing M174/M175 outcomes and do not carry pointer, size, or
status-enum meaning.

## Stop Line

`last_result_ptr`, `last_failure_kind`, and `last_max_block_size` remain `i64`
in this row. They are pointer-shaped, status-enum, and size observer fields
respectively, and each needs its own contract before migration.

No page-map entry pointer/id fields, realloc path pointer/sentinel fields,
aligned allocation rows, provider activation, host allocator replacement,
hooks, or `#[global_allocator]` are opened by this row.

## Acceptance

```bash
bash tools/checks/k2_wide_mimalloc_realloc_failure_contract_guard.sh
bash tools/checks/current_state_pointer_guard.sh
git diff --check
```
