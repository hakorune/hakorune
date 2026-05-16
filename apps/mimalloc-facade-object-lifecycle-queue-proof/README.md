# mimalloc facade object lifecycle queue proof

Decision: accepted for `MIMAP-013`.

This app proves that the thin `HakoAllocObjectLifecycleFacade` can compose the
object-backed `HakoAllocObjectLifecyclePageQueue` from MIMAP-012. The facade
stores the queue, adds real `HakoAllocPageModel` objects, calls queue selection,
and returns selected-page identity through read-only scalar observers at the
facade boundary. The underlying queue proof now exercises fourth-slot selection
through the queue-length loop. Returned selected page objects are intentionally
not exposed, bound, or mutated at the facade boundary.

Acceptance backend: LLVM/EXE primary.

VM remains diagnostic only for this object-heavy route. This row does not
activate OSVM, provider hooks, remote-free execution, or host allocator
replacement.
