# lang/src/runtime/substrate/atomic — Atomic Capability Staging

Responsibilities:
- Docs-first reservation for `hako.atomic`.
- Future home for:
  - load/store
  - CAS
  - fetch_add
  - fence
  - pause/yield hint

Rules:
- Keep this directory docs-first for the current phase.
- `atomic` is capability substrate, not semantic owner.

Non-goals:
- No `.hako` implementation yet.
- No TLS policy here.
- No GC policy here.
- No final platform atomics fallback here.
