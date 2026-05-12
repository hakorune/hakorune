---
Status: Complete
Date: 2026-05-12
Scope: M188 exact usize for allocation request path.
Related:
  - lang/src/hako_alloc/memory/alignment_policy_box.hako
  - lang/src/hako_alloc/memory/page_box.hako
  - lang/src/hako_alloc/memory/page_map_aligned_small_path_box.hako
  - lang/src/hako_alloc/memory/huge_threshold_router_box.hako
---

# 293x-199 M188 Request Path usize

## Goal

Add typed `usize` input facades across the current allocation request path.

M188 adds:

```text
HakoAllocAlignmentPolicy.*_usize(...)
HakoAllocPageModel.acquire_usize(...)
HakoAllocPageMapAlignedSmallPath.allocateAlignedSmallUsize(...)
HakoAllocHugeThresholdRouter.classifyAlignedRequestUsize(...)
HakoAllocHugeThresholdRouter.allocateUsize(...)
HakoAllocHugeThresholdRouter.allocateAlignedUsize(...)
```

## Stop Line

M188 does not migrate stored fields, page ids, block ids, modeled pointers,
page-map entries, result handles, or failure sentinels to `usize`.

The facades only give request sizes and alignments exact input metadata. All
existing result and reject lanes keep their current signed behavior.

## Proof

```bash
bash tools/checks/k2_wide_mimalloc_request_path_usize_guard.sh
bash tools/checks/k2_wide_mimalloc_aligned_small_path_guard.sh
bash tools/checks/k2_wide_mimalloc_huge_threshold_routing_guard.sh
bash tools/checks/current_state_pointer_guard.sh
git diff --check
```
