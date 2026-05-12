---
Status: Complete
Date: 2026-05-12
Scope: M178 `.hako` mimalloc aligned allocation small path.
Related:
  - docs/development/current/main/design/mimalloc-hako-port-implementation-plan-ssot.md
  - lang/src/hako_alloc/memory/alignment_policy_box.hako
  - lang/src/hako_alloc/memory/page_map_aligned_small_path_box.hako
  - lang/src/hako_alloc/memory/page_map_release_box.hako
---

# 293x-188 M178 Aligned Allocation Small Path

## Goal

Attach alignment metadata to normal page-map-backed small allocations.

M178 keeps the small execution path explicit:

```text
request size + alignment
  -> normalize/reject through M177
  -> choose a fitting normal small page
  -> acquire block
  -> register ptr through page_map
  -> expose alignment/padded-size metadata while the ptr stays live
```

This row does not start huge-page routing or native aligned allocation claims.

## Stop Line

M178 does not implement huge routing, huge-page ownership, byte-copy alignment
shims, secure-list hardening, OSVM release widening, provider activation, hook
install, process allocator replacement, or native/ABI alignment semantics.

## Proof

```bash
bash tools/checks/k2_wide_mimalloc_aligned_small_path_guard.sh
bash tools/checks/current_state_pointer_guard.sh
git diff --check
```
