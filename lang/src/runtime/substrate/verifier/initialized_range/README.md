# lang/src/runtime/substrate/verifier/initialized_range — Initialized-Range Verifier Staging

Responsibilities:
- Live `initialized-range` verifier facade for the current `phase-29ct` slice.
- Answer one question only: is this index inside the currently readable initialized range?

Rules:
- Keep initialized-range reasoning separate from bounds math and ownership transfer.
- Keep this directory focused on readable-range checks only.

Current live surface:
- `ensure_initialized_index_i64(handle, idx)` gate for the RawArray slot read path.
- Current readable range is intentionally locked to `BufCoreBox.len_i64(handle)` until `set_len/shrink` widening lands.

Non-goals:
- No bounds verifier here.
- No ownership verifier here.
- No `set_len` / `shrink` implementation here.
- No allocator / TLS / atomic / GC policy here.
