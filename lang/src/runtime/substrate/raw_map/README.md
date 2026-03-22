# lang/src/runtime/substrate/raw_map — RawMap Substrate Staging

Responsibilities:
- First live `RawMap` substrate facade for the phase-29ct ladder.
- Current live surface:
  - `entry_count_i64(handle)`
  - `cap_i64(handle)`
  - `probe_i64(handle, key_i64)` / `probe_any(handle, key_any)`
  - `slot_load_i64(handle, key_i64)` / `slot_load_any(handle, key_any)`
  - `slot_store_i64_any(handle, key_i64, val_any)` / `slot_store_any(handle, key_any, val_any)`
- Future home for the `RawMap` box shape:
  - bucket/capacity shape
  - probe walk
  - tombstone handling
  - rehash trigger vocabulary
  - slot load/store for bucket entries
- Truthful widening gate:
  - `docs/development/current/main/design/raw-map-truthful-native-seam-inventory.md`

Rules:
- `RawMap` is not a semantic owner box.
- `RawMap` stays above capability modules and below `runtime/collections/`.
- Keep this directory focused on the RawMap observer/probe ladder; do not move collection semantics here.

Non-goals:
- No `rehash` / `tombstone` policy implementation yet.
- No allocator / TLS / atomic / GC implementation here.
- No OS VM / final allocator / final ABI stubs here.
