# lang/src/runtime/substrate/ptr — `hako.ptr` Staging

Responsibilities:
- Typed pointer/span facade near `hako.mem` and `hako.buf`.
- Future home for:
  - typed pointer vocabulary
  - span/view vocabulary
  - inbounds/raw read-write entry vocabulary

Current live surface:
- `slot_load_i64`
- `slot_store_i64`
- `slot_store_string_handle`
- `slot_len_i64`
- `slot_cap_i64`
- `slot_append_any`
- `slot_pop_any`
- `slot_remove_any`
- `slot_insert_any`
- `slot_slice_any`
- `slot_reserve_i64`
- `slot_grow_i64`

Rules:
- Keep pointer power restricted and capability-shaped.
- Do not introduce unrestricted raw pointer semantics here.
- Keep direct array-slot backend symbol names here, below owner facades.
- Keep buffer-facing shape vocabulary in `hako.buf`; this module only owns the
  lower slot route names for the current live row.

Non-goals:
- No allocator policy here.
- No TLS/atomic/GC policy here.
