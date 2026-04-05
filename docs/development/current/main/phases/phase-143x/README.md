# Phase 143x: map owner cutover implementation

- Status: Successor
- 目的: `phase-140x` で固定した Map seam を implementation authority に進め、`.hako` 側へ visible owner behavior を寄せる。

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
  - `map_runtime_facade.rs`
- Rust accelerators:
  - `map_probe.rs`
  - `map_slot_load.rs`
  - `map_slot_store.rs`

## Next

1. follow `phase-142x` Array cutover
2. shift visible Map semantics to `.hako`
3. hand off to `phase-144x string semantic owner follow-up`
