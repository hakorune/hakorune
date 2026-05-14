# 293x-353 MIMAP-012 Object Lifecycle Queue Pilot

Status: landed
Date: 2026-05-15

## Decision

`MIMAP-012` adds an object-backed lifecycle queue owner. The queue retains
`HakoAllocPageModel` objects in `ArrayBox`, selects through page lifecycle
methods, and returns the selected page object to the caller.

## Scope

- Add `HakoAllocObjectLifecyclePageQueue`.
- Add LLVM/EXE-primary proof app and guard for object-backed queue selection.
- Keep VM diagnostic-only for this object-heavy route.
- Keep provider activation, hooks, OSVM execution, host allocator replacement,
  remote-free, atomics, TLS, and segment ownership out of scope.

## Acceptance

- Proof app path: `apps/mimalloc-object-lifecycle-queue-proof/main.hako`.
- Guard path: `tools/checks/k2_wide_mimalloc_object_lifecycle_queue_exe_guard.sh`.
- Expected proof output includes:
  - `pages=20,30,-1`
  - `kinds=1,2,0`
  - `queue=3,2,1,1,3,0,1,3`
  - `shape=18`
  - `summary=ok`

## Follow-up

Next mimalloc row should decide whether to route the allocator facade through the
object-backed queue owner, while keeping LLVM/EXE primary and VM non-blocking.
