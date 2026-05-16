# MIMAP-013 facade object lifecycle queue SSOT

Decision: accepted.

`MIMAP-013` composes the MIMAP-012 object-backed lifecycle queue through a thin
object lifecycle facade. This keeps the row small and avoids broadening the
existing production allocator facade in the same step.

## Backend acceptance

Acceptance backend: LLVM/EXE primary.

VM remains diagnostic only for this object-heavy route. VM green is useful but
not required for MIMAP-013 completion.

## Owner boundary

Facade owner:

```text
lang/src/hako_alloc/memory/object_lifecycle_facade_box.hako
```

Object queue owner:

```text
lang/src/hako_alloc/memory/object_lifecycle_page_queue_box.hako
```

The facade may:

- store one `HakoAllocObjectLifecyclePageQueue`
- forward page-object `addPage` operations
- call queue selection and return selected-page identity as a scalar observer
- expose read-only observer counters for the object queue
- rely on the queue owner's queue-length selection loop without exposing the
  returned page object at the facade boundary

The facade must not:

- call OSVM/page-source APIs as part of object lifecycle selection
- own segment, TLS, atomic, remote-free, abandoned reclaim, or page-map policy
- activate provider/hook/global allocator behavior
- add backend-specific matcher shortcuts

## Proof and guard

```text
apps/mimalloc-facade-object-lifecycle-queue-proof/main.hako
tools/checks/k2_wide_mimalloc_facade_object_lifecycle_queue_exe_guard.sh
```

## Follow-up

`MIMAP-040A` updates the underlying queue selection to a queue-length loop while
keeping the facade observer-only. Binding or mutating a facade-returned selected
page object remains deferred. Do not combine helper-call object loops, facade
selected-object exposure, dense proof reads, and allocator execution in one row.
