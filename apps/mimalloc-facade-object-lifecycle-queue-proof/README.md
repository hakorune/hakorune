# mimalloc facade object lifecycle queue proof

Decision: accepted for `MIMAP-013`.

This app proves that the thin `HakoAllocObjectLifecycleFacade` can compose the
object-backed `HakoAllocObjectLifecyclePageQueue` from MIMAP-012. The facade
stores the queue, adds real `HakoAllocPageModel` objects, calls queue selection,
and returns selected-page identity through read-only scalar observers at the
facade boundary. Returned selected page objects are intentionally not exposed,
bound, or mutated in this row; that heavier dynamic receiver shape stays in the
MIR object-loop sidecar series.

Acceptance backend: LLVM/EXE primary.

VM remains diagnostic only for this object-heavy route. This row does not
activate OSVM, provider hooks, remote-free execution, or host allocator
replacement.
