# 293x-356 MIMAP-013 Facade Object Lifecycle Queue

Status: landed
Date: 2026-05-15

## Decision

`MIMAP-013` composes the object-backed lifecycle queue from `MIMAP-012` through
a thin `HakoAllocObjectLifecycleFacade`.

## Scope

- Add a thin facade with a `HakoAllocObjectLifecyclePageQueue` field.
- Add narrow facade methods for adding page objects, invoking queue selection,
  returning selected page identity as a scalar observer, and reading queue
  observer counters.
- Add a proof app that selects reusable and active pages through the facade and
  reads selected-page identity through facade observers.
- Keep LLVM/EXE as primary acceptance and VM diagnostic-only.

## Non-goals

- No OSVM/page-source execution in the object queue route.
- No provider activation, hooks, host allocator replacement, remote-free
  execution, TLS, atomics, segment ownership, or page-map lookup.
- No backend-specific matcher shortcuts.
- No production allocator facade widening, binding or direct mutation of
  facade-returned selected page objects,
  dynamic object-loop helper-call, or nullable selected-object field broadening
  beyond the already-landed MIMAP-012 queue shape.

## Acceptance

- Proof app path:
  `apps/mimalloc-facade-object-lifecycle-queue-proof/main.hako`
- Guard path:
  `tools/checks/k2_wide_mimalloc_facade_object_lifecycle_queue_exe_guard.sh`
- Expected proof output includes:
  - `adds=0,1,2`
  - `pages=20,30,-1`
  - `kinds=1,2,0`
  - `queue=3,3,2,1,1,3,0,1`
  - `shape=18`
  - `summary=ok`
- Landed guard output:
  - `[mimap013-mir-json] ok`
  - `[k2-wide-mimalloc-facade-object-lifecycle-queue-exe] ok`

## Follow-up

After this row lands, the primary next row should compose a small allocation
fast-path over the facade-owned object queue, unless `MIR-ROW-B` is selected as
the next compiler acceptance sidecar.
