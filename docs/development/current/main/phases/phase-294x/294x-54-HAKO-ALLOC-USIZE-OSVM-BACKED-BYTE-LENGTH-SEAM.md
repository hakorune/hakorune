---
Status: Landed
Date: 2026-05-23
Scope: comparison-required OSVM-backed fast-path byte-length exact `usize` seam.
Related:
  - docs/development/current/main/phases/phase-294x/294x-53-MIMALLOC-COMPARISON-VERTICAL-SLICE-WORKLOAD-PACK.md
  - lang/src/hako_alloc/memory/osvm_backed_fast_path_heap_box.hako
  - lang/src/hako_alloc/memory/page_source_policy_box.hako
  - tools/checks/k2_wide_mimalloc_osvm_page_source_composition_guard.sh
---

# 294x-54 Hako Alloc Usize OSVM Backed Byte-Length Seam

## Decision

Migrate only the OSVM-backed fast-path byte-length seam needed by the
comparison vertical slice.

This row makes the following exact `usize`:

```text
HakoAllocOsVmBackedFastPathHeap.block_size
HakoAllocOsVmBackedFastPathHeap.page_capacity
HakoAllocOsVmPageBacking.bytes
HakoAllocPageSourcePolicy reserve/commit/decommit/unreserve byte length params
```

The page-source policy now calls the existing `OsVmCoreBox.*_bytes_usize`
facades, which still route through the current-lane non-negative i64 subset
before the underlying OSVM ABI call.

## Stop Line

This row does not migrate:

- `HakoAllocOsVmBackedFastPathHeap.bin`;
- `next_page_id`;
- `backing_count`;
- `HakoAllocOsVmBackedHandle.page_id` or `block_id`;
- `HakoAllocOsVmPageBacking.page_id` or `base`;
- OSVM native pointer representation;
- provider activation, host allocator replacement, hooks, or
  `#[global_allocator]`.

It does not add new OSVM native leaves. The underlying substrate calls remain
the existing reserve/commit/decommit/unreserve ABI.

## Verification

```bash
bash tools/checks/k2_wide_mimalloc_osvm_page_source_composition_guard.sh
bash tools/checks/current_state_pointer_guard.sh
git diff --check
```
