---
Status: Landed
Date: 2026-05-23
Scope: select page-queue bin exact `usize` row.
Related:
  - docs/development/current/main/design/usize-semantic-foundation-ssot.md
  - docs/development/current/main/phases/phase-294x/294x-90-usize-semantics-taskboard.md
  - lang/src/hako_alloc/memory/NUMERIC_FIELDS.md
  - lang/src/hako_alloc/memory/page_queue_box.hako
---

# 294x-70 Hako Alloc Usize Page Queue Bin Selection

## Decision

Select `HakoAllocPageQueue.bin` as `HAKO-ALLOC-USIZE-FIELD-GROUP-093`.

`bin` is the page-queue owner-local size-class index. The queue does not use it
as a failure sentinel, pointer, or mutable occupancy value; it records which
size-class page queue this owner represents. The follow-on row migrates only the
queue-local field and leaves heap-level bin mirrors and size-class policy return
values signed until their own rows.

## Stop Line

The follow-on row must not migrate:

- `HakoAllocFastPathHeap.bin`;
- `HakoAllocOsVmBackedFastPathHeap.bin`;
- `SizeClassBox.size_to_bin(...)` or `size_to_bin_usize(...)` return shape;
- `HakoAllocPageQueue.has_direct_page`;
- page identity, direct-page index, page/block identity payloads, pointer-like
  fields, or sentinel-returning methods;
- provider activation, host allocator replacement, hooks, TLS, atomics, or
  `#[global_allocator]`.

## Verification

```bash
bash tools/checks/current_state_pointer_guard.sh
git diff --check
```
