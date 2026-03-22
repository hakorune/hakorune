# lang/src/runtime/substrate/gc — GC Capability Staging

Responsibilities:
- Docs-first reservation for `hako.gc`.
- Future home for:
  - write_barrier
  - root_scope
  - pin/unpin
  - GC-facing hook vocabulary

Rules:
- Keep this directory docs-first for the current phase.
- `gc` is capability substrate, not semantic owner.

Non-goals:
- No `.hako` implementation yet.
- No allocator state machine here.
- No collector backend here.
- No final native GC integration here.
