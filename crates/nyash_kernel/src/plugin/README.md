# plugin module notes

Status: runtime ABI routes / direct leaves.

This module is the kernel-side plugin runtime route layer. Keep it as a thin
execution boundary: route external ABI calls, normalize raw arguments, and
delegate to the narrow leaf that owns the requested operation. Do not move
language-level collection or string route semantics into this module.

## Public Surface

- `mod.rs` owns the public re-export list.
- `array.rs`, `map.rs`, `runtime_data.rs`, `string.rs`, `birth.rs`,
  `future.rs`, `invoke.rs`, `invoke_core.rs`, `console.rs`, `instance.rs`,
  and `intarray.rs` are the public plugin entry modules.
- Current crate-root compatibility re-exports are glob-based for these public
  entry modules: `array`, `birth`, `console`, `future`, `instance`,
  `intarray`, `invoke`, `invoke_core`, `map`, `runtime_data`, `semantics`,
  and `string`.
- Do not prune a glob re-export until the target module has an explicit symbol
  inventory or wiring test. `wiring_tests` currently pins the future/invoke
  public ABI surface.
- `map_compat.rs` owns the alternate map ABI route group and stays
  separate from `map::*` routes.
- `module_string_dispatch/` owns the compiled-stage1 route table for
  `using_resolver`, `BuildBox`, MIR-builder, and the llvm backend route.
  Keep it local to those routes; do not widen it into a broader route
  table.

## Collection Routes

- Array runtime-data route:
  `runtime_data.rs -> array_slot_*` direct leaves.
- Array raw leaves:
  `array_slot_load.rs`, `array_slot_store.rs`, `array_slot_append.rs`, and
  `array_slot_capacity.rs`.
- Array string-slot write routes:
  `array_string_slot.rs -> array_string_slot_write.rs`, with
  `array_text_write_txn.rs` as the resident/fallback transaction boundary.
- Map runtime-data route:
  `runtime_data.rs -> map_runtime_data.rs`.
- Map raw leaves:
  `map_slot_load.rs`, `map_slot_store.rs`, `map_slot_mutate.rs`,
  `map_probe.rs`.
- `map_aliases.rs` owns the canonical map ABI alias surface.

`RuntimeData` is a dispatch boundary only. It must not absorb Array or
Map route semantics.

## Cache And Value Boundaries

- `handle_cache.rs` owns generic handle cache helpers and typed Array/Map or
  Instance route classification that is not array-index specific.
- `array_handle_cache.rs` owns the array fast path that requires
  `NonNull<ArrayBox>` cache behavior.
- `array_runtime_aliases.rs` owns the alias-facing array/string entry
  routes, including the text-slot write routes and the string suffix /
  indexof helpers. It routes through `array_string_slot_write.rs`, which
  uses `array_text_write_txn.rs` for the resident/fallback transaction
  boundary. Keep the `Resident/Fallback` contract local to text slot
  updates; do not duplicate it in alias modules.
- `value_codec/` owns plugin value encode/decode.
- `value_codec/borrowed_handle.rs` owns borrowed-alias encode.
- `value_codec/text_carrier.rs` owns the read-only text view and owned
  text buffer.
- `value_codec/string_materialize.rs` owns the text-slot publication
  boundary. Do not duplicate publication decisions in callers.
- `value_demand.rs` owns the runtime-private vocabulary for value/storage/
  publish/mutation demands. Callers should select a named `DemandSet`; storage
  code should not recreate the same demand condition with ad hoc booleans.
- `value_lane.rs` maps selected `DemandSet` values to concrete storage
  actions. It does not decide route legality or provenance.

## Change Rule

- Prefer a new narrow leaf or module-local helper over widening a route or
  helper.
- Do not add public ABI, TextLane behavior, or forwarding from this README
  alone; update the relevant SSOT first.
