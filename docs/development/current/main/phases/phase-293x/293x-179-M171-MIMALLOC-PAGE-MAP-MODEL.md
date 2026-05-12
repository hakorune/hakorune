---
Status: Complete
Date: 2026-05-12
Scope: M171 `.hako` mimalloc pointer-to-page map model.
Related:
  - docs/development/current/main/design/mimalloc-hako-port-implementation-plan-ssot.md
  - lang/src/hako_alloc/memory/page_map_box.hako
  - apps/mimalloc-page-map-proof/
---

# 293x-179 M171 Mimalloc Page-Map Model

## Goal

Add the first page-map owner after M170.

`HakoAllocPageMap` now owns a small model for caller-visible block pointer
ownership:

- `register(ptr, page_id, block_id)` records a live pointer ownership entry;
- `lookup(ptr)` resolves a live entry to page/block identity;
- `unregister(ptr)` marks an entry dead so duplicate old frees can fail fast.

This closes the M170 caller-provided block-id proof seam at the data-model
level. Later rows can compose lookup with page-local release and realloc policy.

## Stop Line

M171 is a pure `.hako` ownership map model. It does not implement arbitrary
free, realloc, pointer arithmetic, OSVM release, remote-free atomics, provider
activation, hook install, process allocator replacement, `.inc` name matching,
or production `usize` field migration.

## Proof

```bash
bash tools/checks/k2_wide_mimalloc_page_map_guard.sh
bash tools/checks/current_state_pointer_guard.sh
git diff --check
```
