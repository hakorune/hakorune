---
Status: Landed
Date: 2026-05-23
Scope: production fast-path handle requested-size exact `usize` migration.
Related:
  - lang/src/hako_alloc/memory/alloc_fast_path_heap_box.hako
  - tools/checks/k2_wide_mimalloc_alloc_fast_path_guard.sh
  - lang/src/hako_alloc/memory/NUMERIC_FIELDS.md
---

# 294x-51 Hako Alloc Usize Fast Path Handle Requested Size

## Decision

Migrate only `HakoAllocFastPathHandle.requested_size` to exact `usize`
storage.

The fast-path heap now carries exact size/capacity metadata, and page-local
`acquire(requested_size: usize)` already treats requested size as a non-negative
size value. The returned handle can therefore preserve the requested-size
payload as exact `usize` without changing page or block identity semantics.

## Stop Line

This row does not migrate `HakoAllocFastPathHandle.page_id` or `block_id`.
Those remain signed id/index payloads until object-return id/index contracts and
sentinel-return seams are split.

It does not migrate OSVM-backed handles, page-map handles, provider activation,
host allocator replacement, hooks, or global allocator integration.

## Verification

```bash
bash tools/checks/k2_wide_mimalloc_alloc_fast_path_guard.sh
bash tools/checks/k2_wide_mimalloc_osvm_page_source_composition_guard.sh
bash tools/checks/current_state_pointer_guard.sh
git diff --check
```
