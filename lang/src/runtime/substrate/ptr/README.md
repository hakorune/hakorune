# lang/src/runtime/substrate/ptr — `hako.ptr` Staging

Responsibilities:
- Typed pointer/span facade above `hako.mem` and `hako.buf`.
- Future home for:
  - typed pointer vocabulary
  - span/view vocabulary
  - inbounds/raw read-write entry vocabulary

Rules:
- Keep pointer power restricted and capability-shaped.
- Do not introduce unrestricted raw pointer semantics here.
- Keep this directory docs-first for the current phase.

Non-goals:
- No `.hako` implementation yet.
- No allocator policy here.
- No TLS/atomic/GC policy here.
