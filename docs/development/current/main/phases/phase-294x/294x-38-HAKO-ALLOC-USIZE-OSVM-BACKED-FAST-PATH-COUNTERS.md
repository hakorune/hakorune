---
Status: Complete
Date: 2026-05-23
Scope: migrate OSVM-backed fast-path heap event/source counters from `i64` to exact `usize`.
Related:
  - docs/development/current/main/design/usize-semantic-foundation-ssot.md
  - docs/development/current/main/phases/phase-294x/294x-90-usize-semantics-taskboard.md
  - lang/src/hako_alloc/memory/NUMERIC_FIELDS.md
  - lang/src/hako_alloc/memory/osvm_backed_fast_path_heap_box.hako
  - tools/checks/k2_wide_mimalloc_osvm_page_source_composition_guard.sh
---

# 294x-38 Hako Alloc Usize OSVM-Backed Fast Path Counters

## Decision

Migrate the `HakoAllocOsVmBackedFastPathHeap` owner-local event/source
counters:

- `alloc_count`
- `release_count`
- `fallback_count`
- `page_create_count`
- `reject_count`
- `reserve_count`
- `commit_count`
- `decommit_count`
- `source_reject_count`

These fields are monotonic counters attached to the OSVM-backed fast-path owner
and do not carry negative sentinel values.

## Stop Line

`bin`, `block_size`, `page_capacity`, `next_page_id`, and `backing_count` remain
`i64`. `backing_count` is intentionally kept signed in this row because it is
still compared with signed `page_id` inputs. `HakoAllocOsVmPageBacking` and
`HakoAllocOsVmBackedHandle` payload fields also remain `i64` until pointer,
byte-length, and object-return API contracts are split. This row does not open
OSVM unreserve/release ownership in the M168 heap/app, TLS/worker-local, atomic
remote-free, provider activation, host allocator replacement, hooks, or
`#[global_allocator]`.

## Acceptance

```bash
bash tools/checks/k2_wide_mimalloc_osvm_page_source_composition_guard.sh
bash tools/checks/current_state_pointer_guard.sh
git diff --check
```
