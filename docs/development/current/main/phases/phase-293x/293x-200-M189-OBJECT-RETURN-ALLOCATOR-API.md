---
Status: Complete
Date: 2026-05-12
Scope: M189 object-return allocate/realloc EXE parity.
Related:
  - lang/src/hako_alloc/memory/page_heap_box.hako
  - apps/mimalloc-object-return-api-proof/main.hako
  - tools/checks/k2_wide_mimalloc_object_return_api_guard.sh
---

# 293x-200 M189 Object-Return Allocator API

## Goal

Prove the semantic allocator API can return handle objects without replacing it
with scalar observers.

M189 covers:

```text
HakoAllocHeap.allocate(size) -> HakoAllocHandle
HakoAllocHeap.realloc(handle, size) -> HakoAllocHandle
HakoAllocHeap.release(handle) cleanup side effect
```

The proof compares the VM and pure-first EXE proof lines for the same
handle-field observations, release side effect, and final heap state. Harness
noise such as plugin warnings or process result trailers is filtered out of the
parity comparison.

## Stop Line

M189 does not add nullable result wrappers, failure reason objects, page-map API
object handles, byte-copy payload semantics, provider activation, hooks, process
allocator replacement, or `.inc` allocator-name matching.

M190 owns the explicit nullable/failure handle contract.

## Proof

```bash
bash tools/checks/k2_wide_mimalloc_object_return_api_guard.sh
bash tools/checks/current_state_pointer_guard.sh
git diff --check
```
