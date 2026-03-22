# lang/src/runtime/substrate/buf — `hako.buf` Staging

Responsibilities:
- Buffer-shape capability facade above `hako.mem`.
- Future home for:
  - len / cap
  - reserve / grow / shrink
  - set_len

Rules:
- Treat `hako.buf` as shape/control vocabulary, not allocator policy.
- Depend on `hako.mem` when implementation begins.
- Keep this directory docs-first for the current phase.

Non-goals:
- No `.hako` implementation yet.
- No typed pointer dereference semantics here.
- No collection policy here.
