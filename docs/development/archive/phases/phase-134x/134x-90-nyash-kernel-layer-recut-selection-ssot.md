# Phase 134x-90: nyash_kernel layer recut selection SSOT

## Goal

Re-cut `crates/nyash_kernel` into four responsibility buckets before the next optimization wave.

## Four Buckets

1. `keep`
   - host/kernel keep
   - lifetime-sensitive hot leaf keep
2. `thin keep`
   - narrow ABI alias / forwarder surface
3. `compat glue`
   - frozen bridge / invoke / future / surrogate glue
4. `substrate candidate`
   - owner-level logic that can move toward `.hako` after boundaries are clean

## Current First Slices

- `crates/nyash_kernel/src/exports/string.rs` split landed
- `crates/nyash_kernel/src/plugin/map_substrate.rs` thin-alias recut landed

## Constraints

- Keep the fixed order:
  1. `leaf-proof micro`
  2. `micro kilo`
  3. `main kilo`
- Do not reopen vm cleanup.
- Do not start broad `.hako` migration before `ABI / glue / substrate` is separated in Rust.
- Do not destabilize current perf baselines while re-cutting responsibilities.

## Success Condition

- The four-bucket inventory is recorded and source-backed.
- The first two source slices have landed.
- `phase-137x main kilo reopen selection` remains the next optimization lane.
