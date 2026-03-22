# lang/src/runtime/substrate/raw_array — RawArray Staging

Responsibilities:
- Staging home for the first algorithm-substrate consumer above `mem` / `buf` / `ptr` / minimum verifier.
- Future home for the `RawArray` box shape:
  - `ptr/cap/len`
  - reserve/grow
  - slot load/store
  - append-at-end policy
- Current widened substrate path includes:
  - `slot_load_i64` via `OwnershipCoreBox` + `BoundsCoreBox` + `InitializedRangeCoreBox` + `PtrCoreBox`
  - `slot_store_i64` via `OwnershipCoreBox` + `BoundsCoreBox` + `PtrCoreBox`
  - `slot_len_i64` / `slot_append_any` via `OwnershipCoreBox` + `PtrCoreBox`
  - `slot_reserve_i64` / `slot_grow_i64` via `BufCoreBox`, which remains a thin shape facade over the current capacity backend

Rules:
- `RawArray` is not a semantic owner box.
- `RawArray` stays above capability modules and below `runtime/collections/`.
- Keep this directory focused on the RawArray substrate ladder; do not move owner semantics here.

Non-goals:
- No `set_len` / `shrink` implementation yet.
- No `RawMap` logic here.
- No allocator / TLS / atomic / GC implementation here.
- No OS VM / final allocator / final ABI stubs here.
