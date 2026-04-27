---
Status: Landed
Date: 2026-04-27
Scope: Inventory MapLookupFusionRoute public field consumers before boundary cleanup
Related:
  - CURRENT_TASK.md
  - docs/development/current/main/CURRENT_STATE.toml
  - docs/development/current/main/phases/phase-291x/291x-484-next-lane-selection-card.md
  - src/mir/map_lookup_fusion_plan.rs
  - src/runner/mir_json_emit/root.rs
  - src/runner/mir_json_emit/tests/map_lookup_fusion_routes.rs
---

# 291x-485: MapLookupFusionRoute Field Inventory

## Goal

Separate the stable JSON read contract from the MIR-owned
`MapLookupFusionRoute` construction and field layout.

This is BoxShape-only. It must not change MapLookup fusion detection, refresh
ordering, JSON field names, route ids, effect tags, proof strings, helper
behavior, `.inc` readers, or lowering tiers.

## Findings

Owner module:

- `src/mir/map_lookup_fusion_plan.rs`
  - owns `MapLookupFusionRoute` derivation from `generic_method_routes`
  - sorts by `block` and `get_instruction_index`
  - has owner-local tests that inspect fields

External field reader:

- `src/runner/mir_json_emit/root.rs`
  - reads every public field directly to emit `map_lookup_fusion_routes`
  - only needs stable read accessors/string tags

External manual construction consumer:

- `src/runner/mir_json_emit/tests/map_lookup_fusion_routes.rs`
  - imports `MapLookupFusionRoute`, route enums, generic-method facts, and
    `ValueId`
  - manually constructs one JSON fixture by public field literal

No external route planner or lowering code needs to construct this route record
directly.

## Cleaner Boundary

```text
src/mir/map_lookup_fusion_plan.rs
  owns fields and construction
  exposes stable accessors for JSON emission
  exposes cfg(test) fixture builders for JSON tests

src/runner/mir_json_emit/root.rs
  reads only accessors

src/runner/mir_json_emit/tests/*
  imports only owner fixture support
```

## Acceptance For Next Card

- Make `MapLookupFusionRoute` fields private.
- Add public read accessors for the JSON emitter.
- Add `#[cfg(test)] pub(crate) mod test_support` with a JSON fixture builder.
- Update `src/runner/mir_json_emit/root.rs` to use accessors only.
- Update the runner JSON fixture test to import only `test_support`.
- Preserve all JSON assertions and route values.
