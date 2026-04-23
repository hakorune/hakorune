---
Status: Landed
Date: 2026-04-24
Scope: Clarify the remaining Map compat/source boundary after the std scaffold and vm-hako route cleanup landed.
Related:
  - docs/development/current/main/phases/phase-291x/README.md
  - docs/development/current/main/phases/phase-291x/291x-93-mapbox-surface-task-board.md
  - docs/development/current/main/phases/phase-291x/291x-94-map-std-prelude-cleanup-card.md
  - docs/development/current/main/phases/phase-291x/291x-95-mapbox-hako-extended-route-cleanup-card.md
  - apps/selfhost-runtime/ops_calls.hako
  - crates/nyash_kernel/src/plugin/map_compat.rs
---

# Map Compat/Source Cleanup Card

## Decision

Keep the remaining Map compatibility split explicit:

```text
Rust catalog / route owner
  -> src/boxes/map_surface_catalog.rs
  -> Route::Unified MapBox rows

Source-level selfhost/runtime wrapper
  -> apps/selfhost-runtime/ops_calls.hako
  -> only OpsCalls.map_has(...) remains for pref == "ny"

Compat-only legacy ABI quarantine
  -> crates/nyash_kernel/src/plugin/map_compat.rs
  -> historical pure/JIT export names only
```

This card is boundary cleanup only. It does not change Map behavior, compat ABI
symbols, or vm-hako state-owner semantics.

## Current Facts

- `apps/lib/boxes/map_std.hako` is already deleted.
- `OpsCalls.map_has(...)` is the only remaining Map-specific wrapper in the
  selfhost runtime `pref == "ny"` lane.
- `size/get/set/toString` no longer need a Map-only wrapper because the generic
  receiver path already owns them.
- `crates/nyash_kernel/src/plugin/map_compat.rs` still exports historical
  `nyash.map.*` symbols for compatibility callers.
- `MapCoreBox` / `MapStateCoreBox` already own the active `.hako` direct/S0
  source routes; this card must not reopen that owner split.

## Implementation Slice

- add one-screen comments that explain why `OpsCalls.map_has(...)` still exists
- add one-screen comments that mark `map_compat.rs` as legacy quarantine, not a
  forward authoring point
- sync phase/current pointers so the blocker moves to the next card:
  `MapBox.get(existing-key) typing`

## Non-Goals

- do not delete `map_compat.rs`
- do not add new `pref == "ny"` Map wrappers
- do not change `MapBox.get(existing-key)` typing in this card
- do not touch `MapCoreBox` / `MapStateCoreBox` semantics
- do not widen this cleanup into route or ABI redesign

## Acceptance

```bash
cargo test -p nyash_kernel --lib map_set_h_legacy_completion_code_and_mutation_roundtrip --quiet
cargo test -p nyash_kernel --lib map_invalid_handle_fail_safe_contract --quiet
cargo test --lib map_surface_catalog_preserves_current_slots_and_aliases --quiet
cargo test --lib invoke_surface_routes_current_map_rows --quiet
bash tools/checks/current_state_pointer_guard.sh
bash tools/smokes/v2/profiles/integration/apps/phase291x_mapbox_surface_catalog_vm.sh
```

## Exit Condition

The repo shows one clear Map boundary story:
catalog-owned runtime routes, one explicit selfhost-runtime `map_has` wrapper,
and one compat-only legacy ABI quarantine with no ambiguity about the next work.
