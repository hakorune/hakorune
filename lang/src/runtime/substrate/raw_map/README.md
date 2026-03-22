# lang/src/runtime/substrate/raw_map — RawMap Observer Staging

Responsibilities:
- First live `RawMap` observer facade for the phase-29ct ladder.
- Current live surface:
  - `entry_count_i64(handle)`
- Future home for the `RawMap` box shape:
  - bucket/capacity shape
  - probe walk
  - tombstone handling
  - rehash trigger vocabulary
  - slot load/store for bucket entries

Rules:
- `RawMap` is not a semantic owner box.
- `RawMap` stays above capability modules and below `runtime/collections/`.
- Keep this directory focused on the RawMap observer/probe ladder; do not move collection semantics here.

Non-goals:
- No additional `.hako` `RawMap` expansion yet beyond the live observer box.
- No allocator / TLS / atomic / GC implementation here.
- No OS VM / final allocator / final ABI stubs here.
