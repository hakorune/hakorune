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
  - `slot_load_usize` via the same gates, using the non-negative current-lane
    i64 `usize` subset
  - `slot_store_i64` via `OwnershipCoreBox` + `BoundsCoreBox` + `PtrCoreBox`
  - `slot_store_usize` and `slot_store_string_handle_usize` via the same
    gates, using the non-negative current-lane i64 `usize` subset
  - `slot_remove_any` via `OwnershipCoreBox` + `BoundsCoreBox` + `InitializedRangeCoreBox` + `PtrCoreBox`
  - `slot_remove_any_usize` via the same gates, using the non-negative
    current-lane i64 `usize` subset
  - `slot_insert_any` via `OwnershipCoreBox` + `BoundsCoreBox.insert` + `OwnershipCoreBox.any` + `PtrCoreBox`
  - `slot_insert_any_usize` via the same gates, using the non-negative
    current-lane i64 `usize` subset
  - `slot_len_i64` / `slot_append_any` via `OwnershipCoreBox` + `PtrCoreBox`
  - `slot_len_usize` via `OwnershipCoreBox` + `BufCoreBox`
  - `slot_cap_i64` via `OwnershipCoreBox` + `BufCoreBox`
  - `slot_cap_usize` via `OwnershipCoreBox` + `BufCoreBox`
  - `slot_reserve_i64` / `slot_grow_i64` via `BufCoreBox`, which remains a thin shape facade over `PtrCoreBox` slot routes
  - `slot_reserve_usize` / `slot_grow_usize` via `BufCoreBox`, using the
    non-negative current-lane i64 `usize` subset

Rules:
- `RawArray` is not a semantic owner box.
- `RawArray` stays above capability modules and below `runtime/collections/`.
- Keep this directory focused on the RawArray substrate ladder; do not move owner semantics here.

Non-goals:
- No `set_len` / `shrink` implementation yet.
- No `RawMap` logic here.
- No allocator / TLS / atomic / GC implementation here.
- No OS VM / final allocator / final ABI stubs here.
