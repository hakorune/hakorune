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
  - fixed-slot `cas_i64`, `load_i64`, `store_i64`, and `fetch_add_i64`
  - truthful fence rows over `hako_barrier_touch_i64`
  - memory-order values are integer vocabulary only; M33 reserves ordered
    fixed-slot operation names but does not make ordered methods live

Non-goals:
- No generic atomic API in this wave.
- No ordered load/store/CAS/fetch_add implementation in this row.
- No pointer atomic API in this wave.
- No TLS policy here.
- No GC policy here.
- No final platform atomics fallback here.
