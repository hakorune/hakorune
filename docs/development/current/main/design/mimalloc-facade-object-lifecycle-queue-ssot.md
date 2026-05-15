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

Next rows may either continue the dynamic object-loop sidecar (`MIR-ROW-B`) or
compose allocation fast-path policy over the facade-owned object queue. Binding
or mutating a facade-returned selected page object is deliberately deferred to a
separate dynamic receiver row. `MIMAP-013` keeps facade selection observer-only
because returning the selected object through the facade re-enters the
object-heavy MIR route covered by `VM-LIM-001`. Do not combine helper-call
object loops, nullable selected-object fields, dense proof reads, and allocator
execution in one row.
