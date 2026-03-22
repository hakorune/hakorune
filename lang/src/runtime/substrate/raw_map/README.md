# lang/src/runtime/substrate/raw_map — RawMap Staging

Responsibilities:
- Docs-first reservation for the algorithm-substrate consumer after `RawArray`.
- Future home for the `RawMap` box shape:
  - bucket/capacity shape
  - probe walk
  - tombstone handling
  - rehash trigger vocabulary
  - slot load/store for bucket entries

Rules:
- `RawMap` is not a semantic owner box.
- `RawMap` stays above capability modules and below `runtime/collections/`.
- Keep this directory docs-first for the current phase.

Non-goals:
- No `.hako` `RawMap` implementation yet.
- No allocator / TLS / atomic / GC implementation here.
- No OS VM / final allocator / final ABI stubs here.
