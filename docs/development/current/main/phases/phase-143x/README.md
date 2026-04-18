# Phase 143x: map owner cutover implementation

- Status: Landed
- 目的: `phase-140x` で固定した Map seam を implementation authority に進め、`.hako` 側へ visible owner behavior を寄せる。

## Success

- `MapBox.set` visible semantics are explicitly owned by `.hako` first
- `MapBox.get/has` visible semantics are expressed through `.hako` owner helpers, not embedded inline in dispatch branches
- visible `MapBox.{get,set,has,len/length/size}` behavior is expressed through `.hako` owner helpers, not embedded inline in dispatch branches
- Rust `map_runtime_data.rs` / `map_compat.rs` remain forwarding-only and shrink-only
- Rust `map_aliases.rs` stays thin facade
- raw probe/load/store leaves remain Rust-owned
- next lane is `phase-144x string semantic owner follow-up`

## Decision Now

- `.hako` owner implementation:
  - `map_core_box.hako`
  - `map_state_core_box.hako`
- substrate below owner:
  - `raw_map_core_box.hako`
- Rust thin facade:
  - `map_aliases.rs`
- Rust observer/compat forwarding:
  - `map_substrate.rs`
  - `map_runtime_data.rs`
  - `map_compat.rs`
- Rust accelerators:
  - `map_probe.rs`
  - `map_slot_load.rs`
  - `map_slot_store.rs`

## Fresh Read

- this lane is not about moving raw map probe/load/store leaves out of Rust
- this lane is about making `.hako` the actual owner of visible Map semantics
- first exact cutover unit is `MapBox.set`; `get/has` are landed; final exact unit `MapBox.len/length/size` is landed
- Rust should retain capability, forwarding core, and isolated thin facade surfaces only

## Next

1. visible `MapBox.{set,get,has,len/length/size}` behavior now sits on `.hako` owner helpers
2. Rust map forwarding/facade surfaces stay thin and non-owning
3. hand off to `phase-144x string semantic owner follow-up`
