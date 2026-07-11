# 194x-90 Remaining Oversized Module Split SSOT

Decision
- this phase is a single BoxShape series for the remaining oversized modules.
- each split keeps the existing public API and existing behavior.
- module boundaries follow responsibility, not benchmark or route names.

Targets
- `src/boxes/array/mod.rs`
  - split storage helpers, visible ops, trait impls, and tests
- `src/runner/mir_json_emit/mod.rs`
  - split root orchestration, metadata serialization, plan serialization, ordering, io, and tests
- `src/mir/string_corridor_placement.rs`
  - split candidate schema types, relation-carry helpers, plan derivation, candidate derivation, and tests

Rules
- no semantic widening in the same commit as a split
- keep restart pointers updated after the series lands
- if a helper becomes a shared owner, re-export from the facade rather than changing downstream imports broadly
