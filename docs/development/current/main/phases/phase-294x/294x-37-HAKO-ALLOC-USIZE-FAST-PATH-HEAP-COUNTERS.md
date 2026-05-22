---
Status: Complete
Date: 2026-05-23
Scope: migrate fast-path heap event/reject counters from `i64` to exact `usize`.
Related:
  - docs/development/current/main/design/usize-semantic-foundation-ssot.md
  - docs/development/current/main/phases/phase-294x/294x-90-usize-semantics-taskboard.md
  - lang/src/hako_alloc/memory/NUMERIC_FIELDS.md
  - lang/src/hako_alloc/memory/alloc_fast_path_heap_box.hako
  - tools/checks/k2_wide_mimalloc_alloc_fast_path_guard.sh
---

# 294x-37 Hako Alloc Usize Fast Path Heap Counters

## Decision

Migrate the `HakoAllocFastPathHeap` owner-local event/reject counters:

- `alloc_count`
- `release_count`
- `fallback_count`
- `page_create_count`
- `reject_count`

These fields are monotonic fast-path heap counters and do not carry negative
sentinel values.

## Stop Line

`HakoAllocFastPathHeap.bin`, `block_size`, `page_capacity`, and `next_page_id`
remain `i64` because they are route/index/size/capacity metadata, not this
row's counter group. `HakoAllocFastPathHandle.page_id`, `block_id`, and
`requested_size` also remain `i64` until object-return API parity and
sentinel-return seams are split. This row does not migrate the OSVM-backed fast
path heap, page source, TLS/worker-local, atomic remote-free, provider
activation, host allocator replacement, hooks, or `#[global_allocator]`.

## Acceptance

```bash
bash tools/checks/k2_wide_mimalloc_alloc_fast_path_guard.sh
bash tools/checks/current_state_pointer_guard.sh
git diff --check
```
