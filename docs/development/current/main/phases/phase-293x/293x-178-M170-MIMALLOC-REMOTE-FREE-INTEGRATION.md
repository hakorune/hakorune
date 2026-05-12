---
Status: Complete
Date: 2026-05-12
Scope: M170 `.hako` mimalloc remote-free page integration.
Related:
  - docs/development/current/main/design/mimalloc-hako-port-implementation-plan-ssot.md
  - lang/src/hako_alloc/memory/remote_free_page_integration_box.hako
  - lang/src/hako_alloc/memory/remote_free_policy_box.hako
  - lang/src/hako_alloc/memory/page_box.hako
  - apps/mimalloc-remote-free-page-integration-proof/
---

# 293x-178 M170 Mimalloc Remote-Free Integration

## Goal

Compose the existing bounded pointer remote-free retry policy with page-owned
state.

`HakoAllocRemoteFreePageInbox` now owns the narrow M170 integration seam:

- `publish(block_ptr, block_id, interferer_ptr)` pushes `block_ptr` through
  `HakoAllocRemoteFreePolicy.pushRetry(...)`, then records the caller-provided
  `block_id` in a page-owned pending list;
- `collectOne()` moves one pending `block_id` into
  `HakoAllocPageModel.releaseLocal(...)`;
- `peekHead()` and `peekNext(...)` expose the existing pointer-list proof
  surface without becoming a new pointer atomic owner.

## Stop Line

M170 only composes already-landed pointer load/store/CAS route facts with the
page model. It does not add pointer `fetch_add`, page-map lookup, arbitrary
pointer free, abandoned-page reclaim, OSVM unreserve/release, provider
activation, hook install, process allocator replacement, `.inc` name matching,
or production `usize` field migration.

The proof uses caller-provided block identity. Resolving arbitrary remote
pointers to page/block ownership remains a future page-map row.

## Proof

```bash
bash tools/checks/k2_wide_mimalloc_remote_free_page_integration_guard.sh
bash tools/checks/current_state_pointer_guard.sh
git diff --check
```
