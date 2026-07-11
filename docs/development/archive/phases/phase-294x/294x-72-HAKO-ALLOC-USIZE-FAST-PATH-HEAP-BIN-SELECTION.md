---
Status: Landed
Date: 2026-05-23
Scope: select fast-path heap bin exact `usize` row.
Related:
  - docs/development/current/main/design/usize-semantic-foundation-ssot.md
  - docs/development/current/main/phases/phase-294x/294x-90-usize-semantics-taskboard.md
  - lang/src/hako_alloc/memory/NUMERIC_FIELDS.md
  - lang/src/hako_alloc/memory/alloc_fast_path_heap_box.hako
---

# 294x-72 Hako Alloc Usize Fast Path Heap Bin Selection

## Decision

Select `HakoAllocFastPathHeap.bin` as `HAKO-ALLOC-USIZE-FIELD-GROUP-095`.

`bin` is the non-OSVM fast-path heap's owner-local size-class index. The heap
copies it into the already-exact `HakoAllocPageQueue.bin` field and does not use
it as a failure sentinel, pointer-like payload, or mutable occupancy value. The
follow-on row can therefore migrate only this heap-local field while leaving the
OSVM-backed mirror and size-class policy return shapes as separate seams.

## Stop Line

The follow-on row must not migrate:

- `HakoAllocOsVmBackedFastPathHeap.bin`;
- `SizeClassBox.size_to_bin(...)` or `size_to_bin_usize(...)` return shape;
- `HakoAllocFastPathHandle.page_id` or `block_id`;
- `HakoAllocPageModel.page_id`;
- page/block identity payloads, pointer-like fields, or sentinel-returning
  methods;
- provider activation, host allocator replacement, hooks, TLS, atomics, or
  `#[global_allocator]`.

## Verification

```bash
bash tools/checks/current_state_pointer_guard.sh
git diff --check
```
