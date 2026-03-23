# lang/src/runtime/substrate/atomic — Atomic Capability Staging

Responsibilities:
- Current home for the first truthful `hako.atomic` row.
- Future home for:
  - load/store
  - CAS
  - fetch_add
  - fence
  - pause/yield hint

Rules:
- `atomic` is capability substrate, not semantic owner.

Current live subset:
- `atomic_core_box.hako`
  - `fence_i64()`
  - truthful row over `hako_barrier_touch_i64`

Non-goals:
- No generic atomic API in this wave.
- No TLS policy here.
- No GC policy here.
- No final platform atomics fallback here.
