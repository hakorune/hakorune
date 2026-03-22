# lang/src/runtime/substrate/tls — TLS Capability Staging

Responsibilities:
- Docs-first reservation for `hako.tls`.
- Future home for:
  - thread/task-local slot
  - cache-slot primitive
  - locality-facing substrate vocabulary

Rules:
- Keep this directory docs-first for the current phase.
- `tls` is capability substrate, not semantic owner.

Non-goals:
- No `.hako` implementation yet.
- No allocator policy here.
- No final platform TLS fallback here.
- No GC barrier logic here.
