---
Status: SSOT
Decision: accepted
Date: 2026-03-23
Scope: `phase-29ct` の I10 として、`hako.atomic` / `hako.tls` / `hako.gc` の truthful native seam を棚卸しし、live に進める面と parked に残す面を固定する。
Related:
  - CURRENT_TASK.md
  - docs/development/current/main/10-Now.md
  - docs/development/current/main/phases/phase-29ct/README.md
  - docs/development/current/main/design/gc-tls-atomic-capability-ssot.md
  - docs/development/current/main/design/thread-and-tls-capability-ssot.md
  - docs/development/current/main/design/substrate-capability-ladder-ssot.md
  - lang/src/runtime/substrate/gc/README.md
  - lang/src/runtime/substrate/tls/README.md
  - lang/src/runtime/substrate/atomic/README.md
  - crates/nyash_kernel/src/exports/runtime.rs
  - crates/nyash_kernel/src/exports/string_span_cache.rs
  - lang/c-abi/README.md
---

# Atomic/TLS/GC Truthful Native Seam Inventory

## Goal

- `hako.atomic` / `hako.tls` / `hako.gc` を docs 名だけで live 化せず、current native seam の truth に合わせて widening 順を固定する。
- `GC` は current truthful seam があるので first live slice に進める。
- `atomic` / `tls` は current backend/helper reality のまま parked に保つ。

## Current Truth Classes

### A. Live now

These have a truthful substrate-facing seam today:

- `hako_barrier_touch_i64`
  - implemented at `lang/c-abi/shims/hako_kernel.c`
  - suitable as the first helper-shaped `hako.atomic` live row
- `hako_last_error`
  - implemented at `lang/c-abi/shims/hako_diag_mem_shared_impl.inc`
  - suitable as the first helper-shaped `hako.tls` live row
- `nyash.gc.barrier_write`
  - implemented at `crates/nyash_kernel/src/exports/runtime.rs`
  - forwards to runtime GC hooks
  - suitable as the first `hako.gc` live row

### B. Truthful native helpers, but not substrate rows yet

- `hako_gc_stats`
- `hako_gc_roots_snapshot`
  - current C ABI read-only observer helpers
  - truthful, but not the first `hako.gc` substrate vocabulary for this lane
- TLS-backed caches/helpers
  - `crates/nyash_kernel/src/plugin/handle_cache.rs`
  - `crates/nyash_kernel/src/exports/string_span_cache.rs`
  - truthful host helpers, but not the public `.hako substrate` end-state
- runtime internal atomics
  - `fetch_add` / `compare_exchange` / fence usage exists inside Rust runtime
  - truthful as implementation detail, not yet as generic exported `hako.atomic` vocabulary

### C. Parked vocabulary

These remain parked until a truthful exported/native seam exists:

- `hako.atomic.load/store`
- `hako.atomic.compare_exchange`
- `hako.atomic.fetch_add`
- `hako.atomic.pause/yield`
- language-level `thread_local` lowering
- `TlsCell<T>`
- raw `hako.tls.slot_get/set`
- raw `hako.tls.cache_slot`
- `hako.gc.root_scope`
- `hako.gc.pin/unpin`
- `hako.gc.collect/start/stop`

## Implementation Reading

- conceptual widening order remains:
  1. `atomic`
  2. `tls`
  3. `gc`
- current implementation order is seam-first:
  1. truthful seam inventory
  2. `gc` first live row
  3. helper-shaped first truthful `tls` / `atomic` rows
  4. generic `atomic` / `tls` remain parked until truthful seams exist

## Decision

- current live slices are:
  - `hako.atomic.fence_i64`
  - `hako.tls.last_error_text_h`
  - `hako.gc.write_barrier_i64`
- generic atomics and final language-level TLS are not implemented in `.hako` in this wave.
- no false generic `atomic/tls` substrate rows are introduced just to satisfy the conceptual order.
