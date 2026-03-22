# lang/src/runtime/substrate/gc — GC Capability Staging

Responsibilities:
- First live `hako.gc` capability facade for the current phase.
- Current live surface:
  - `write_barrier_i64(handle_or_ptr)`
- Future home for:
  - write_barrier
  - root_scope
  - pin/unpin
  - GC-facing hook vocabulary

Rules:
- `gc` is capability substrate, not semantic owner.
- Keep this directory limited to truthful GC hook vocabulary.

Non-goals:
- No allocator state machine here.
- No collector backend here.
- No final native GC integration here.
- No `root_scope` / `pin` / `unpin` here yet.
