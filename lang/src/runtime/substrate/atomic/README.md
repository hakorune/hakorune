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
  - `order_relaxed_i64()` / `order_acquire_i64()` /
    `order_release_i64()` / `order_acq_rel_i64()` / `order_seq_cst_i64()`
  - `is_valid_order_i64(order)`
  - `fence_order_i64(order)`
  - truthful fence rows over `hako_barrier_touch_i64`
  - memory-order values are integer vocabulary only; they do not make generic
    load/store/CAS/fetch_add live

Non-goals:
- No generic atomic API in this wave.
- No load/store/CAS/fetch_add in this row.
- No TLS policy here.
- No GC policy here.
- No final platform atomics fallback here.
