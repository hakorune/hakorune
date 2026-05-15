# mimalloc facade small allocation proof

Decision: accepted for `MIMAP-014A`.

This app proves that `HakoAllocObjectLifecycleFacade` can select one reusable
page through its object-backed lifecycle queue and allocate one small block via
`HakoAllocPageModel.acquire(size)`.

Acceptance backend: LLVM/EXE primary.

The proof stays scalar at the facade boundary: selected page id, allocated block
id, reason code, and summary. This row does not expose a selected page object,
add release/free, realloc, alignment, OSVM/page-source activation, provider
hooks, remote-free execution, or host allocator replacement.
