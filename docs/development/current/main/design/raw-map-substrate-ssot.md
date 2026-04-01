---
Status: SSOT
Decision: provisional
Date: 2026-03-23
Scope: `phase-29ct` の C3 として、`RawMapCoreBox` を capability substrate の次の consumer として first live `observer + probe/load/store + cap observer` slice まで固定する。
Related:
  - CURRENT_TASK.md
  - docs/development/current/main/10-Now.md
  - docs/development/current/main/phases/phase-29ct/README.md
  - docs/development/current/main/design/substrate-capability-ladder-ssot.md
  - docs/development/current/main/design/raw-array-substrate-ssot.md
  - docs/development/current/main/design/minimum-verifier-ssot.md
  - docs/development/current/main/design/raw-map-truthful-native-seam-inventory.md
  - docs/development/current/main/design/collection-raw-substrate-contract-ssot.md
  - lang/src/runtime/substrate/README.md
  - lang/src/runtime/substrate/raw_map/README.md
---

# RawMap Substrate (SSOT)

## Goal

- `RawMapCoreBox` を `hako_substrate` の次の concrete box として固定する。
- `RawArray` のあとに来る hash/probe substrate の役割を、まず `observer + probe/load/store` slice で live にする。
- `MapCoreBox` の visible owner を崩さず、future low-level map policy の受け皿だけを決める。

## Reading

- `RawMap` は semantic owner ではない。
- `RawMap` は capability substrate と minimum verifier を使う algorithm substrate である。
- current repo reading treats this doc as the first concrete `K2-wide` substrate slice after `K2-core`.
- current phase では first live slice を landed とし、`MapCoreBox.size_i64` が `RawMapCoreBox.entry_count_i64` を通る。
  - `RawMapCoreBox.entry_count_i64` now routes to `nyash.map.entry_count_i64`; `nyash.map.entry_count_h` remains a compat alias only.
- current widening also lands `probe/load/store` façade methods under `RawMapCoreBox`.
- current live observer subset also lands `cap_i64(handle)`.
- truthful widening guard now lives in:
  - `raw-map-truthful-native-seam-inventory.md`

## Fixed Dependencies

`RawMap` の前提は次で固定する。

1. `hako.mem`
2. `hako.buf`
3. `hako.ptr`
4. minimum verifier
   - `bounds`
   - `initialized-range`
   - `ownership`

`RawMap` はこの substrate 群の consumer であり、provider ではない。

## RawMap Roles

- current live slice:
  - `entry_count_i64`
  - `cap_i64`
  - `probe_i64` / `probe_any`
  - `slot_load_i64` / `slot_load_any`
  - `slot_store_i64_any` / `slot_store_any`
- future target roles, only when truthful native seams exist:
  - bucket/capacity shape
  - probe walk
  - tombstone handling
  - rehash trigger vocabulary
  - slot load/store vocabulary for bucket entries
- does not own:
  - user-visible `MapBox` semantics
  - missing-key / visible fallback policy
  - ABI manifest truth
  - final allocator backend
  - GC/TLS/atomic policy

## Difference From RawArray

- `RawArray` は contiguous `ptr/cap/len` shape を主に扱う。
- `RawMap` は probe/rehash/tombstone を主に扱う。
- 両者とも semantic owner ではないが、`RawMap` は hash table mechanics の受け皿である。

## Relationship To Current Owner Boxes

- current semantic owner remains:
  - `runtime/collections/map_core_box.hako`
- current native metal helpers remain:
  - `crates/nyash_kernel/src/plugin/map*.rs`
  - `crates/nyash_kernel/src/plugin/handle_cache.rs`
- `RawMap` is the future algorithm substrate box that may sit between those layers later.
- `RawMapCoreBox` is now the first live map substrate box between those layers for `size_i64`.
- `MapCoreBox` now uses the raw map facade for raw receiver-handle `set/get/has` paths while keeping stateful owner fast paths local.
- `RawMapCoreBox` now also owns the first truthful capacity observer route via `nyash.map.cap_h`.

## Physical Staging

current staging root is reserved at:

- [`lang/src/runtime/substrate/raw_map/README.md`](/home/tomoaki/git/hakorune-selfhost/lang/src/runtime/substrate/raw_map/README.md)
- [`lang/src/runtime/substrate/raw_map/raw_map_core_box.hako`](/home/tomoaki/git/hakorune-selfhost/lang/src/runtime/substrate/raw_map/raw_map_core_box.hako)

This phase now lands the first substrate slice through `observer + probe/load/store + cap observer`; rehash/tombstone shape stays future-facing.

## Non-Goals

- additional `.hako` `RawMap` expansion beyond the first substrate slice
- allocator state machine
- TLS / atomic / GC implementation
- OS VM / final allocator / final ABI stub
- unrestricted raw pointer
- perf lane reopen

## Follow-Up

After this live observer slice, the next widening remains:

1. truthful RawMap narrow widening only after inventory review
2. `K2-wide` capability widening packs:
   - `hako.atomic`
   - `hako.tls`
   - `hako.gc`
   - `hako.osvm`
3. `hako_alloc` policy/state rows plus allocator/TLS/GC policy-owner widening

docs/task lock now lives at:

- [`gc-tls-atomic-capability-ssot.md`](/home/tomoaki/git/hakorune-selfhost/docs/development/current/main/design/gc-tls-atomic-capability-ssot.md)
- [`hako-alloc-policy-state-contract-ssot.md`](/home/tomoaki/git/hakorune-selfhost/docs/development/current/main/design/hako-alloc-policy-state-contract-ssot.md)
