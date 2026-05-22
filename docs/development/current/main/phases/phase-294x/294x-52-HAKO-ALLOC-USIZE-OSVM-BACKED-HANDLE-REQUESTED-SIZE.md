---
Status: Landed
Date: 2026-05-23
Scope: production OSVM-backed fast-path handle requested-size exact `usize` migration.
Related:
  - lang/src/hako_alloc/memory/osvm_backed_fast_path_heap_box.hako
  - tools/checks/k2_wide_mimalloc_osvm_page_source_composition_guard.sh
  - lang/src/hako_alloc/memory/NUMERIC_FIELDS.md
---

# 294x-52 Hako Alloc Usize OSVM Backed Handle Requested Size

## Decision

Migrate only `HakoAllocOsVmBackedHandle.requested_size` to exact `usize`
storage.

This mirrors the non-OSVM fast-path handle requested-size migration while
leaving the OSVM page-source byte-length and backing payload seams untouched.

## Stop Line

This row does not migrate `HakoAllocOsVmBackedHandle.page_id` or `block_id`.
It also does not migrate `HakoAllocOsVmPageBacking.base`, `bytes`,
`HakoAllocOsVmBackedFastPathHeap.block_size`, `page_capacity`, `backing_count`,
or OSVM substrate call signatures.

It does not open provider activation, host allocator replacement, hooks, or
global allocator integration.

## Verification

```bash
bash tools/checks/k2_wide_mimalloc_osvm_page_source_composition_guard.sh
bash tools/checks/current_state_pointer_guard.sh
git diff --check
```
