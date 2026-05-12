---
Status: Complete
Date: 2026-05-12
Scope: M190 nullable / failure handle contract.
Related:
  - lang/src/hako_alloc/memory/page_heap_box.hako
  - apps/mimalloc-result-contract-proof/main.hako
  - tools/checks/k2_wide_mimalloc_result_contract_guard.sh
---

# 293x-201 M190 Nullable / Failure Handle Contract

## Goal

Define an explicit result wrapper for allocator API calls that can fail, without
changing the M189 object-return/null compatibility surface.

M190 adds:

```text
HakoAllocHandleResult {
  ok: i64
  reason: i64
  handle: HakoAllocHandle
}

HakoAllocHeap.allocateResult(size) -> HakoAllocHandleResult
HakoAllocHeap.reallocResult(handle, size) -> HakoAllocHandleResult
```

Reason codes are intentionally numeric in this row so the proof remains in the
current scalar/object route lane:

```text
0 ok
1 null handle
2 invalid size
3 invalid/stale handle
4 allocation failed or unsupported size
```

## Stop Line

M190 does not add page-map API result wrappers, byte-copy payload semantics,
aligned result wrappers, huge allocation results, secure-list hardening,
provider activation, hooks, process allocator replacement, or `.inc`
allocator-name matching.

Existing `allocate(...)` / `realloc(...)` remain compatibility APIs. New callers
that need explicit failure causes use `allocateResult(...)` /
`reallocResult(...)`.

The compatibility `realloc(...)` path must reject stale handles before
replacement allocation. If old-handle release unexpectedly fails after a
replacement was allocated, it rolls back that replacement before returning
`null`.

## Proof

```bash
bash tools/checks/k2_wide_mimalloc_result_contract_guard.sh
bash tools/checks/current_state_pointer_guard.sh
git diff --check
```
