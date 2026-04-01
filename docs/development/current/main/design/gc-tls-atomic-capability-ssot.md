---
Status: SSOT
Decision: provisional
Date: 2026-03-23
Scope: `phase-29ct` の C4 として、`hako.atomic` / `hako.tls` / `hako.gc` を docs-first で固定する。
Related:
  - CURRENT_TASK.md
  - docs/development/current/main/10-Now.md
  - docs/development/current/main/phases/phase-29ct/README.md
  - docs/development/current/main/design/substrate-capability-ladder-ssot.md
  - docs/development/current/main/design/atomic-tls-gc-truthful-native-seam-inventory.md
  - docs/development/current/main/design/thread-and-tls-capability-ssot.md
  - docs/development/current/main/design/raw-array-substrate-ssot.md
  - docs/development/current/main/design/raw-map-substrate-ssot.md
  - docs/development/current/main/design/hako-alloc-policy-state-contract-ssot.md
  - lang/src/runtime/substrate/README.md
  - lang/src/runtime/substrate/atomic/README.md
  - lang/src/runtime/substrate/tls/README.md
  - lang/src/runtime/substrate/gc/README.md
---

# GC/TLS/Atomic Capability Widening (SSOT)

## Goal

- `RawArray` / `RawMap` の次に必要な capability widening を docs-first で固定する。
- current repo reading treats this doc as the capability-widening subset inside `K2-wide`, not as a separate `K3`.
- `hako.atomic` / `hako.tls` / `hako.gc` の責務を分けて、allocator/runtime policy owner と混ざらないようにする。
- `Hakozuna portability layer` の前提になる最小 capability vocabulary を決める。
- `hako.sys` のような catch-all unsafe shelf は作らず、capability family のまま widening する。

## Fixed Order

この widening wave の順番は次で固定する。

1. `hako.atomic`
2. `hako.tls`
3. `hako.gc`
4. `hako.osvm`

## Current Implementation Order

current implementation order is seam-first:

1. truthful native seam inventory
2. `hako.gc` first live row
3. helper-shaped first truthful `hako.tls` / `hako.atomic` rows
4. generic `atomic/tls` vocabulary remains parked until truthful seams exist

## Module Roles

### `hako.atomic`

- owns:
  - load/store
  - CAS
  - fetch_add
  - fence
  - pause/yield hint
- does not own:
  - allocator policy
  - TLS cache policy
  - GC barrier policy

### `hako.tls`

- owns:
  - thread/task-local slot
  - cache-slot primitive
  - locality-facing substrate vocabulary
- does not own:
  - allocator policy
  - atomic memory ordering policy
  - final platform TLS fallback

### `hako.gc`

- owns:
  - write_barrier
  - root_scope
  - pin/unpin
  - GC-facing hook vocabulary
- does not own:
  - object policy owner
  - final collector backend
  - allocator state machine

## Reading

- current wave is not docs-first only anymore
- current first live subset is:
  - `AtomicCoreBox.fence_i64()`
  - `TlsCoreBox.last_error_text_h()`
  - `GcCoreBox.write_barrier_i64(handle_or_ptr)`
- `hako.osvm` remains part of the same capability family even when its first truthful rows are still docs-first / narrow
- `atomic` / `tls` / `gc` は substrate capability であり、semantic owner ではない
- `hako_kernel` / `hako_substrate` と競合する owner noun にしない
- allocator/TLS/GC policy-owner widening lives beside this wave under `hako_alloc` policy/state rows; do not merge that owner reading into capability modules
- truthful seam guard now lives in:
  - `atomic-tls-gc-truthful-native-seam-inventory.md`
- final TLS end-state guard now lives in:
  - `thread-and-tls-capability-ssot.md`

## Physical Staging

current staging roots are reserved at:

- [`lang/src/runtime/substrate/atomic/README.md`](/home/tomoaki/git/hakorune-selfhost/lang/src/runtime/substrate/atomic/README.md)
- [`lang/src/runtime/substrate/tls/README.md`](/home/tomoaki/git/hakorune-selfhost/lang/src/runtime/substrate/tls/README.md)
- [`lang/src/runtime/substrate/gc/README.md`](/home/tomoaki/git/hakorune-selfhost/lang/src/runtime/substrate/gc/README.md)

## Non-Goals

- allocator state machine migration
- `RawArray` / `RawMap` implementation body
- moving collection owner logic into `runtime/substrate/`
- final metal split
- OS VM rewrite beyond the narrow `hako.osvm` capability surface
- final allocator backend rewrite
- unrestricted unsafe surface
- minimum verifier broadening beyond the current docs lock
- broad `atomic` widening beyond `fence_i64`
- broad `tls` widening beyond `last_error_text_h`
- broad `gc` widening beyond `write_barrier_i64`
- perf lane reopen

## Follow-Up

After this docs lock, the next widening remains:

1. generic TLS end-state design (`thread_local` / `TlsCell<T>`) stays docs-first until lowering exists
2. truthful generic `atomic` / `tls` seam extraction
3. broad `gc` widening after new native seam exists
4. `hako.osvm` truthful seam extraction stays in the same `K2-wide` pack
5. allocator/TLS/GC policy-owner widening stays in `hako_alloc-policy-state-contract-ssot.md`
