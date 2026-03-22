# lang/src/runtime/substrate/raw_array — RawArray Staging

Responsibilities:
- Docs-first reservation for the first algorithm-substrate consumer above `mem` / `buf` / `ptr` / minimum verifier.
- Future home for the `RawArray` box shape:
  - `ptr/cap/len`
  - reserve/grow
  - slot load/store
  - append-at-end policy
- Current widened substrate path also includes the reserve/grow verb pair above the concrete array owner, routed through `PtrCoreBox`.

Rules:
- `RawArray` is not a semantic owner box.
- `RawArray` stays above capability modules and below `runtime/collections/`.
- Keep this directory docs-first for the current phase.

Non-goals:
- No `.hako` `RawArray` implementation yet.
- No `RawMap` logic here.
- No allocator / TLS / atomic / GC implementation here.
- No OS VM / final allocator / final ABI stubs here.
