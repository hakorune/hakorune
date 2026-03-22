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
  - truthful host helpers, but not stable `.hako substrate` rows yet
- runtime internal atomics
  - `fetch_add` / `compare_exchange` / fence usage exists inside Rust runtime
  - truthful as implementation detail, not yet as exported `hako.atomic` vocabulary

### C. Parked vocabulary

These remain parked until a truthful exported/native seam exists:

- `hako.atomic.load/store`
- `hako.atomic.compare_exchange`
- `hako.atomic.fetch_add`
- `hako.atomic.fence`
- `hako.atomic.pause/yield`
- `hako.tls.slot_get/set`
- `hako.tls.cache_slot`
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
  3. `atomic` / `tls` remain parked until truthful seams exist

## Decision

- current first live slice is `hako.gc.write_barrier_i64`.
- `atomic` and `tls` are not implemented in `.hako` in this wave.
- no false `atomic/tls` substrate rows are introduced just to satisfy the conceptual order.
