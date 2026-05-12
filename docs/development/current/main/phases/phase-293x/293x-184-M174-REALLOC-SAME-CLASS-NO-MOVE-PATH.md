---
Status: Complete
Date: 2026-05-12
Scope: M174 `.hako` mimalloc realloc same-class/no-move path.
Related:
  - docs/development/current/main/design/mimalloc-hako-port-implementation-plan-ssot.md
  - lang/src/hako_alloc/memory/page_map_release_box.hako
  - lang/src/hako_alloc/memory/page_map_realloc_same_class_box.hako
---

# 293x-184 M174 Realloc Same-Class No-Move Path

## Goal

Keep the current live handle when the new request still fits the current page
block.

M174 adds a narrow path owner over the existing page-map release seam:

```text
HakoAllocPageMap.lookup(ptr)
  -> resolve page/block identity
  -> reject stale/released/grow cases without release or unregister
  -> return the same ptr when requested_size <= page.block_size
```

This row freezes the no-move branch only. It does not allocate a replacement,
copy bytes, or release the old handle on grow failures.

## Stop Line

M174 does not implement alloc-copy-release fallback, byte copy, alignment,
huge allocation, secure-list hardening, OSVM release, provider activation, hook
install, process allocator replacement, `.inc` name matching, or production
`usize` field migration.

The new owner must not take over `register(...)`, `releasePtr(...)`,
`releaseLocal(...)`, or `unregister(...)`. Those owners stay in M171-M173.

## Proof

```bash
bash tools/checks/k2_wide_mimalloc_realloc_same_class_guard.sh
bash tools/checks/current_state_pointer_guard.sh
git diff --check
```
