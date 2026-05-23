---
Status: Landed
Date: 2026-05-23
Scope: fast-path heap bin exact `usize` migration.
Related:
  - docs/development/current/main/design/usize-semantic-foundation-ssot.md
  - docs/development/current/main/phases/phase-294x/294x-72-HAKO-ALLOC-USIZE-FAST-PATH-HEAP-BIN-SELECTION.md
  - docs/development/current/main/phases/phase-294x/294x-90-usize-semantics-taskboard.md
  - lang/src/hako_alloc/memory/NUMERIC_FIELDS.md
  - lang/src/hako_alloc/memory/alloc_fast_path_heap_box.hako
  - apps/mimalloc-alloc-fast-path-proof/main.hako
  - tools/checks/k2_wide_mimalloc_alloc_fast_path_guard.sh
---

# 294x-73 Hako Alloc Usize Fast Path Heap Bin

## Decision

Migrate only `HakoAllocFastPathHeap.bin` to exact `usize` storage.

`bin` is the non-OSVM fast-path heap's owner-local size-class index. This row
tightens `HakoAllocFastPathHeap.birth` to accept `bin: usize`, preserves the
existing exact `usize` queue constructor surface, and updates the focused proof
app to exercise exact `usize` heap construction without widening page/block
identity or size-class return seams.

## Stop Line

This row does not migrate:

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
bash tools/checks/k2_wide_mimalloc_alloc_fast_path_guard.sh
bash tools/checks/k2_wide_mimalloc_page_queue_guard.sh
bash tools/checks/k2_wide_mimalloc_osvm_page_source_composition_guard.sh
bash tools/checks/current_state_pointer_guard.sh
git diff --check
```
