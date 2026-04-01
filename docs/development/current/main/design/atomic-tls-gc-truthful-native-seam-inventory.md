---
Status: SSOT
Decision: accepted
Date: 2026-04-01
Scope: `K2-wide` metal keep review の truthful seam inventory として、`hako.atomic` / `hako.tls` / `hako.gc` / `hako.osvm` の live 面と parked 面を固定する。
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

# Atomic/TLS/GC/OSVM Truthful Native Seam Inventory

## Goal

- `hako.atomic` / `hako.tls` / `hako.gc` / `hako.osvm` を docs 名だけで live 化せず、current native seam の truth に合わせて widening 順を固定する。
- `GC` は current truthful seam があるので first live slice に進める。
- `atomic` / `tls` / `osvm` は truthful seam から narrow rows だけ live にし、 broad vocabulary は parked に保つ。
- `hako_alloc` policy/state rows は sibling SSOT で管理し、この inventory では capability/native seam だけを扱う。

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
- `hako_osvm_reserve_bytes_i64`
  - implemented at `lang/c-abi/shims/hako_kernel.c`
  - suitable as the first reserve-only `hako.osvm` live row

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
- broad `hako.osvm.commit/decommit/page_size`

## Implementation Reading

- conceptual widening order remains:
  1. `atomic`
  2. `tls`
  3. `gc`
  4. `osvm`
- current implementation order is seam-first:
  1. truthful seam inventory
  2. first truthful rows (`gc`, helper-shaped `tls` / `atomic`, reserve-only `osvm`)
  3. generic `atomic` / `tls` remain parked until truthful seams exist
  4. `hako_alloc` policy/state rows widen beside this inventory, not inside it

## Decision

- current live slices are:
  - `hako.atomic.fence_i64`
  - `hako.tls.last_error_text_h`
  - `hako.gc.write_barrier_i64`
  - `hako.osvm.reserve_bytes_i64`
- generic atomics, final language-level TLS, and broad OS VM vocabulary are not implemented in `.hako` in this wave.
- no false generic `atomic/tls/osvm` substrate rows are introduced just to satisfy the conceptual order.
