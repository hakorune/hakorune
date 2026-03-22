# lang/src/runtime/substrate/buf — `hako.buf` Staging

Responsibilities:
- Buffer-shape capability facade above `hako.mem`.
- Future home for:
  - len / cap
  - reserve / grow / shrink
  - set_len

Current live surface:
- `len_i64`
- `cap_i64`
- `reserve_i64`
- `grow_i64`

Rules:
- Treat `hako.buf` as shape/control vocabulary, not allocator policy.
- Keep the current implementation as a thin bridge over the existing array capacity backend.
- Keep this directory docs-first for the current phase.

Non-goals:
- No allocator policy here.
- No typed pointer dereference semantics here.
- No collection policy here.
