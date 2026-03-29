---
Status: SSOT
Decision: accepted
Date: 2026-03-23
Scope: `phase-29ct` の I9 として、current `RawMap` widening を `std::collections::HashMap` backend の truthful seam に揃え、live 語彙と parked 語彙を分離する。
Related:
  - CURRENT_TASK.md
  - docs/development/current/main/10-Now.md
  - docs/development/current/main/phases/phase-29ct/README.md
  - docs/development/current/main/design/raw-map-substrate-ssot.md
  - lang/src/runtime/substrate/raw_map/README.md
  - lang/src/runtime/substrate/raw_map/raw_map_core_box.hako
  - lang/src/runtime/collections/map_core_box.hako
  - src/boxes/map_box.rs
  - crates/nyash_kernel/src/plugin/map_substrate.rs
---

# RawMap Truthful Native Seam Inventory

## Goal

- current `RawMap` substrate widening を native backend の truth に合わせて固定する。
- `rehash` / `tombstone` のような vocabulary を、truthful seam がないまま live contract に混ぜない。
- 次の `RawMap` widening を、truth がある narrow slice だけに限定する。

## Backend Reality

- current native backend is:
  - `src/boxes/map_box.rs`
  - `std::collections::HashMap<String, Box<dyn NyashBox>>`
- therefore current backend truth is:
  - key/value lookup by normalized string key
  - presence probe
  - insert/update
  - entry count
  - capacity observer
  - visible `clear` / `delete` helpers
- therefore current backend does **not** truthfully expose:
  - tombstone count
  - explicit rehash trigger
  - bucket walk / bucket-by-index layout
  - stable load-factor control contract

## Current Truth Classes

### A. Live substrate seams

These are truthful today and are already exported/live:

- `nyash.map.entry_count_i64`
- `nyash.map.cap_h`
- `nyash.map.probe_hi`
- `nyash.map.probe_hh`
- `nyash.map.slot_load_hi`
- `nyash.map.slot_load_hh`
- `nyash.map.slot_store_hih`
- `nyash.map.slot_store_hhh`

These correspond to:

- `MapBox.entry_count_i64()`
- `MapBox.capacity_i64()`
- `MapBox.contains_key_str(...)`
- `MapBox.get_opt_key_str(...)`
- `MapBox.insert_key_str(...)`

Compat wrapper still present for historical callers:

- `nyash.map.entry_count_h`

### B. Truthful native helpers, but not substrate exports yet

These are truthful at the native helper level, but are still owned by visible/owner-facing contracts and are not yet clean substrate rows:

- `MapBox.clear()`
  - truthful operation exists
  - current return contract is visible-owner shaped (`"Map cleared"`)
  - if widened later, use a dedicated raw helper / raw ABI row instead of reusing the visible contract
- `MapBox.delete(...)`
  - truthful delete exists
  - current interface still mixes key normalization and visible `"Key not found"` / deleted-value behavior
  - do not promote as raw substrate without a dedicated raw helper
- `MapBox.keys()` / `MapBox.values()`
  - truthful visible helpers
  - not a good `RawMap` substrate face in the current ladder

### C. Parked vocabulary

These stay parked until a truthful native seam exists:

- `tombstone_count`
- `rehash`
- `bucket_count`
- `bucket_walk`
- `bucket_slot_load`
- `bucket_slot_store`
- explicit load-factor trigger / threshold contract

Reason:

- the current backend is `HashMap`, not an exposed open-addressed table substrate
- exporting these names now would create a false contract

## Current Reading Of RawMap

- `RawMapCoreBox` is currently truthful for:
  - observer
  - capacity observer
  - probe
  - slot load
  - slot store
- `RawMapCoreBox` is **not yet** the owner of:
  - rehash policy
  - tombstone policy
  - bucket-layout mechanics

## Next Exact Slice

If `RawMap` widens again before `GC/TLS/atomic`, the next truthful candidate is:

1. `clear`
   - add a dedicated raw helper in `MapBox`
   - export a dedicated raw ABI row
   - keep visible owner contract separate

The next candidate after that is:

2. `remove/delete`
   - only after a dedicated raw delete helper exists
   - keep visible `"missing/deleted"` message contract out of the substrate row

## Non-Goals

- inventing `rehash/tombstone` rows on top of `HashMap`
- widening `RawMap` with visible-owner strings/messages
- changing current `MapCoreBox` user-visible semantics
- changing current ABI symbol names for live rows

## Decision

- `RawMap` widening is now constrained by truthful native seam inventory.
- `rehash/tombstone` remain parked by design.
- the next allowed widening is `truthful narrow widening only`.
