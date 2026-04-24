---
Status: Landed
Date: 2026-04-24
Scope: Probe the smallest MapLookupSameKey lowering by folding the proven has result to true.
Related:
  - docs/development/current/main/CURRENT_STATE.toml
  - docs/development/current/main/design/hotline-core-method-contract-ssot.md
  - docs/development/current/main/design/perf-owner-first-optimization-ssot.md
  - docs/development/current/main/phases/phase-291x/291x-147-mapget-maphas-fusion-metadata-card.md
  - src/mir/map_lookup_fusion_plan.rs
  - lang/c-abi/shims/hako_llvmc_ffi_generic_method_has_policy.inc
---

# 291x-148 MapGet/MapHas Fusion Has-Const Probe Card

## Goal

Consume landed `MapLookupSameKey` metadata in the smallest possible lowering
probe:

```text
MapLookupSameKey.has_result -> constant 1
MapLookupSameKey.get_result -> unchanged RuntimeData get
```

This removes the second lookup/probe call from the measured loop without adding
a new runtime helper or changing MapBox storage.

## Baseline

Current H147 baseline on `kilo_leaf_map_getset_has`:

```text
ny_aot_instr = 2424471199
ny_aot_cycles = 561577904
ny_aot_ms = 106
ny_aot_ipc = 4.32
```

Current `ny_main` loop still calls:

```text
nyash.runtime_data.get_hh
nyash.map.has_h
```

Top owner family:

```text
spec_to_string = 48.39%
hash_one       = 32.06%
nyash.map.probe_hi = 7.28%
nyash.runtime_data.get_hh = 1.86%
```

## Design

Only fold `has` when MIR metadata proves:

```text
route_id = map_lookup.same_key
fusion_op = MapLookupSameKey
receiver_origin_box = MapBox
key_route = i64_const
get_return_shape = scalar_i64_or_missing_zero
get_value_demand = scalar_i64
get_publication_policy = no_publication
has_result_shape = presence_bool
proof = same_receiver_same_i64_key_scalar_get_has
lowering_tier = cold_fallback
block / has_instruction_index match the current call
receiver_value / key_value / has_result_value match the current operands
```

If matched:

```llvm
%has = add i64 0, 1
```

If metadata is missing, existing generic `has` policy remains unchanged. If
metadata is malformed for the current site, fail fast.

## Boundary

- Do not lower `get` in this card.
- Do not add a runtime helper.
- Do not add native i64-key storage.
- Do not infer method semantics from names in `.inc`; consume
  `map_lookup_fusion_routes` only.
- Keep `RuntimeDataBox.get` mixed semantics unchanged.

## Acceptance

- `ny_main` loop no longer calls `nyash.map.has_h` / `nyash.map.probe_hi` for
  the fused site.
- `ny_main` still calls `nyash.runtime_data.get_hh`.
- Keeper requires cycles improvement and no IPC collapse.
- If cycles regress or owner family remains unchanged enough to be a wash,
  close as rejected probe and revert code.

## Probe Result

Implementation:

```text
MapLookupSameKey.has_result -> add i64 0, 1
MapLookupSameKey.get_result -> existing nyash.runtime_data.get_hh
```

Observed result:

```text
baseline ny_aot_instr  = 2424471199
baseline ny_aot_cycles = 561577904
baseline ny_aot_ms     = 106
baseline ny_aot_ipc    = 4.32

probe ny_aot_instr     = 1300470606
probe ny_aot_cycles    = 275315781
probe ny_aot_ms        = 54
probe ny_aot_ipc       = 4.72
```

Asm confirms the hot loop no longer calls:

```text
nyash.map.has_h
nyash.map.probe_hi
```

The loop still calls:

```text
nyash.runtime_data.get_hh
```

Top owner family after the probe:

```text
spec_to_string = 56.51%
hash_one       = 25.52%
map_runtime_data_get_any_key = 1.90%
MapBox::get_opt_key_str      = 1.89%
```

This is a keeper because cycles improve, IPC improves, and the second lookup
owner is removed without changing RuntimeData get semantics.

## Closeout

- Keep the `.inc` consumer narrow: it reads `map_lookup_fusion_routes` only.
- Do not expand this into method-name semantic planning.
- Next performance seam is the remaining single `RuntimeDataBox.get` lookup
  owner: i64 key conversion/hash in the get path.
