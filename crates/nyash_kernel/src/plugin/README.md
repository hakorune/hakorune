# plugin module notes

Status: runtime ABI facade / native substrate leaves.

This module is the kernel-side plugin runtime surface. Keep it as a thin
execution boundary: route external ABI calls, normalize raw arguments, and
delegate to the narrow leaf that owns the requested operation. Do not move
language-level collection or string semantics into this module.

## Public Surface

- `mod.rs` owns the public re-export list.
- `array.rs`, `map.rs`, `runtime_data.rs`, `string.rs`, `birth.rs`,
  `future.rs`, `invoke.rs`, `invoke_core.rs`, `console.rs`, `instance.rs`,
  and `intarray.rs` are the public plugin entry modules.
- `map_compat.rs` is compatibility quarantine. Keep it callable only through
  explicit compat exports/tests; do not re-export it through `map::*`.
- `module_string_dispatch/` is compiled-stage1 compat quarantine. It is
  shrink-only and must not gain new semantic ownership.

## Collection Routes

- Array runtime-data route:
  `runtime_data.rs -> array_runtime_any.rs -> array_runtime_facade.rs`.
- Array raw leaves:
  `array_slot_load.rs`, `array_slot_store.rs`, `array_slot_append.rs`,
  `array_slot_capacity.rs`, and `array_runtime_substrate.rs`.
- Map runtime-data route:
  `runtime_data.rs -> map_runtime_data.rs`.
- Map raw leaves:
  `map_slot_load.rs`, `map_slot_store.rs`, `map_slot_mutate.rs`,
  `map_probe.rs`, and `map_substrate.rs`.
- `map_aliases.rs` owns the canonical map ABI alias surface.

`RuntimeData` is a dispatch/facade boundary only. It must not absorb Array or
Map semantics.

## Cache And Value Boundaries

- `handle_cache.rs` owns generic handle cache helpers and typed Array/Map or
  Instance route classification that is not array-index specific.
- `array_handle_cache.rs` owns the array fast path that requires
  `NonNull<ArrayBox>` cache behavior.
- `value_codec/` owns plugin value encode/decode and text-slot publication
  boundaries. Do not duplicate publication decisions in callers.

## Change Rule

- Prefer a new narrow leaf or owner-local helper over widening a facade.
- Do not add public ABI, TextLane behavior, or compatibility forwarding from
  this README alone; update the relevant SSOT first.
- Keep `map_compat.rs` until a caller-proofed retirement task explicitly
  removes every remaining compat route.
