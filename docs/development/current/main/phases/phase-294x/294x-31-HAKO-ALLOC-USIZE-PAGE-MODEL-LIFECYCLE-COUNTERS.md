---
Status: Complete
Date: 2026-05-22
Scope: migrate one page-model lifecycle event/reject counter field group from `i64` to exact `usize`.
Related:
  - docs/development/current/main/design/usize-semantic-foundation-ssot.md
  - docs/development/current/main/phases/phase-294x/294x-90-usize-semantics-taskboard.md
  - lang/src/hako_alloc/memory/NUMERIC_FIELDS.md
  - lang/src/hako_alloc/memory/page_box.hako
  - tools/checks/k2_wide_mimalloc_page_model_guard.sh
  - tools/checks/k2_wide_mimalloc_local_free_retire_guard.sh
  - tools/checks/k2_wide_mimalloc_lifecycle_integration_pilot_guard.sh
  - tools/checks/k2_wide_hako_alloc_recommit_heap_integration_guard.sh
---

# 294x-31 Hako Alloc Usize Page Model Lifecycle Counters

## Decision

Migrate only the `HakoAllocPageModel` lifecycle event/reject counter group:

- `retire_count`
- `decommit_count`
- `recommit_count`
- `reuse_count`
- `lifecycle_reject_count`
- `reactivate_count`
- `reactivate_reject_count`

These fields are monotonic, non-negative counters for lifecycle transition
events or rejected lifecycle attempts. They are not the lifecycle state itself
and do not carry page identity, size, capacity, stack-top, live-count, or byte
meaning.

## Stop Line

`retired` and `decommitted` remain `i64` state flags. `used`, `free_top`,
`local_free_top`, `peak_used`, and `requested_bytes` also remain `i64`.

This row does not migrate page identity, size/capacity, stack-top/live-count,
or byte-length fields. It does not open page queue fields, page-map entry
pointer/id fields, provider activation, host allocator replacement, hooks, or
`#[global_allocator]`.

## Acceptance

```bash
bash tools/checks/k2_wide_mimalloc_page_model_guard.sh
bash tools/checks/k2_wide_mimalloc_local_free_retire_guard.sh
bash tools/checks/k2_wide_mimalloc_lifecycle_integration_pilot_guard.sh
bash tools/checks/k2_wide_hako_alloc_recommit_heap_integration_guard.sh
bash tools/checks/current_state_pointer_guard.sh
git diff --check
```
