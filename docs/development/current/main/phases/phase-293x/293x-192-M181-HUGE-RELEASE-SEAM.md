---
Status: Complete
Date: 2026-05-12
Scope: M181 `.hako` mimalloc huge release seam.
Related:
  - docs/development/current/main/design/mimalloc-hako-port-implementation-plan-ssot.md
  - lang/src/hako_alloc/memory/huge_release_seam_box.hako
  - lang/src/hako_alloc/memory/huge_page_model_box.hako
  - apps/mimalloc-huge-release-seam-proof/main.hako
---

# 293x-192 M181 Huge Release Seam

## Goal

Release huge handles without routing them through small page free lists.

M181 owns only this composition:

```text
HakoAllocPageMap.lookup(ptr)
  -> confirm ptr is live in HakoAllocHugePageModel
  -> mark huge model entry released
  -> HakoAllocPageMap.unregister(ptr)
```

## Stop Line

M181 does not implement OS unreserve/release, decommit, small-page
`releaseLocal(...)`, secure-list hardening, byte copy, provider activation,
hook install, process allocator replacement, native allocator replacement, or
`.inc` allocator-name matching.

M182 owns secure free-list policy inventory.

## Proof

```bash
bash tools/checks/k2_wide_mimalloc_huge_release_seam_guard.sh
bash tools/checks/current_state_pointer_guard.sh
git diff --check
```
