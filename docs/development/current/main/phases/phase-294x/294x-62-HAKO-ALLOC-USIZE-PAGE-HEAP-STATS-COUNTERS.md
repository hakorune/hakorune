---
Status: Landed
Date: 2026-05-23
Scope: legacy page-heap stats counter exact `usize` migration.
Related:
  - docs/development/current/main/design/usize-semantic-foundation-ssot.md
  - docs/development/current/main/phases/phase-294x/294x-90-usize-semantics-taskboard.md
  - lang/src/hako_alloc/memory/NUMERIC_FIELDS.md
  - lang/src/hako_alloc/memory/page_heap_box.hako
  - tools/checks/k2_wide_hako_alloc_usize_page_heap_stats_counters_guard.sh
---

# 294x-62 Hako Alloc Usize Page Heap Stats Counters

## Decision

Return from the comparison evidence row to a narrow production `usize`
field-group and migrate only the live `HakoAllocPage` stats counters:

- `alloc_count`
- `free_count`
- `reuse_count`

These fields are monotonic owner-local counters in the legacy `page_heap_box`
policy-state owner. They are still exercised by `mimalloc-lite`,
`allocator-stress`, and production-facade smoke paths, but they do not carry
negative sentinels and are not page/block identity values.

## Stop Line

This row does not migrate:

- `HakoAllocHandle.page_id`, `block_id`, or `requested_size`;
- `HakoAllocPage.page_id`, `block_size`, `capacity`, `free_top`,
  `current_used`, `peak_used`, or `requested_bytes`;
- page-map ids, OSVM-backed heap `backing_count`, pointer-like payloads, TLS,
  atomics, provider activation, host allocator replacement, hooks, or
  `#[global_allocator]`.

`HakoAllocOsVmBackedFastPathHeap.backing_count` remains signed because existing
guards still require it to compare against signed `page_id` inputs until an
id/index split row opens that seam.

## Verification

```bash
bash tools/checks/k2_wide_hako_alloc_usize_page_heap_stats_counters_guard.sh
bash tools/checks/current_state_pointer_guard.sh
git diff --check
```
