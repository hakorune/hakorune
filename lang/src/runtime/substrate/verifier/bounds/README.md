# lang/src/runtime/substrate/verifier/bounds — Bounds Verifier Staging

Responsibilities:
- Live `bounds` verifier facade for the current `phase-29ct` slice.
- Answer one question only: is this index within the current logical length?

Rules:
- Keep bounds math separate from initialized-range and ownership reasoning.
- Keep this directory focused on fail-fast index checks only.

Current live surface:
- `ensure_index_i64(handle, idx)` gate for RawArray slot read/write paths.

Non-goals:
- No initialized-range verifier here.
- No ownership verifier here.
- No allocator / TLS / atomic / GC policy here.
