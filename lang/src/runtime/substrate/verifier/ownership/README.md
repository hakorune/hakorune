# lang/src/runtime/substrate/verifier/ownership — Ownership Verifier Staging

Responsibilities:
- Current home for the third `minimum verifier` box in the phase-29ct ladder.
- Answer only carrier-liveness ownership questions for the current slice:
  - is a receiver handle still live for read?
  - is a receiver handle still live for write?
  - is a positive `any` carrier still backed by a live handle?

Rules:
- Keep carrier validity separate from bounds math and initialized-range reasoning.
- Do not duplicate borrowed-string alias expiry logic here.
- Treat non-positive `any` carriers as immediate values, not handles.

Current live subset:
- `ensure_handle_readable_i64(handle)`
- `ensure_handle_writable_i64(handle)`
- `ensure_any_readable_i64(value_any)`

Current reading:
- Borrowed alias expiry remains governed by `value_codec` and its conservative fallback.
- This box only prevents silent reuse of invalid positive handles on current raw substrate routes.

Non-goals:
- No ownership transfer engine here.
- No borrow checker or move semantics here.
- No allocator / TLS / atomic / GC policy here.
