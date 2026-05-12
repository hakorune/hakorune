---
Status: Complete
Date: 2026-05-12
Scope: M175 `.hako` mimalloc realloc alloc-copy-release fallback.
Related:
  - docs/development/current/main/design/mimalloc-hako-port-implementation-plan-ssot.md
  - lang/src/hako_alloc/memory/page_map_realloc_same_class_box.hako
  - lang/src/hako_alloc/memory/page_map_realloc_alloc_copy_release_box.hako
---

# 293x-185 M175 Realloc Alloc-Copy-Release Fallback

## Goal

Add the grow fallback after M174 rejects the no-move branch.

M175 keeps the ordering explicit:

```text
lookup old ptr
  -> reject same-class/released/stale/unknown early
  -> allocate replacement block
  -> register new ptr
  -> model one copy
  -> release old ptr through M172 seam
```

This row models copy count only. It does not perform byte copy, aligned
allocation, or huge-page routing.

## Stop Line

M175 does not implement byte copy, alignment, huge allocation, secure-list
hardening, OSVM release, provider activation, hook install, process allocator
replacement, `.inc` name matching, or production `usize` field migration.

The fallback owner may orchestrate `page_map.register(...)` and
`releasePtr(...)`, but it must not take over raw `unregister(...)`,
`releaseLocal(...)`, or same-class/no-move ownership from M174.

## Proof

```bash
bash tools/checks/k2_wide_mimalloc_realloc_alloc_copy_release_guard.sh
bash tools/checks/current_state_pointer_guard.sh
git diff --check
```
