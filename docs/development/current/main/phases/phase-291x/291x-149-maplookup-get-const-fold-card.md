---
Status: Landed
Date: 2026-04-24
Scope: Probe the next smallest MapLookupSameKey lowering by folding the proven get result to its stored scalar constant.
Related:
  - docs/development/current/main/CURRENT_STATE.toml
  - docs/development/current/main/design/hotline-core-method-contract-ssot.md
  - docs/development/current/main/design/perf-owner-first-optimization-ssot.md
  - docs/development/current/main/phases/phase-291x/291x-147-mapget-maphas-fusion-metadata-card.md
  - docs/development/current/main/phases/phase-291x/291x-148-mapget-maphas-fusion-has-const-probe-card.md
  - src/mir/map_lookup_fusion_plan.rs
  - lang/c-abi/shims/hako_llvmc_ffi_map_lookup_fusion_metadata.inc
  - lang/c-abi/shims/hako_llvmc_ffi_generic_method_get_policy.inc
  - lang/c-abi/shims/hako_llvmc_ffi_generic_method_has_policy.inc
---

# 291x-149 MapLookupSameKey Get-Const Fold Card

## Goal

Consume landed `MapLookupSameKey` metadata one step further:

```text
MapLookupSameKey.get_result -> stored_value_const
MapLookupSameKey.has_result -> constant 1
```

This targets the remaining `RuntimeDataBox.get` key conversion/hash owner for
the measured scalar-constant pair without changing MapBox storage and without
adding a runtime helper.

## Baseline

Current H148 keeper result on `kilo_leaf_map_getset_has`:

```text
ny_aot_instr = 1300470606
ny_aot_cycles = 275315781
ny_aot_ms = 54
ny_aot_ipc = 4.72
```

Current `ny_main` loop still calls:

```text
nyash.runtime_data.get_hh
```

The removed call remains removed:

```text
nyash.map.has_h
nyash.map.probe_hi
```

Top owner family after H148:

```text
spec_to_string = 56.51%
hash_one       = 25.52%
map_runtime_data_get_any_key = 1.90%
MapBox::get_opt_key_str      = 1.89%
```

## Design

Only fold `get` when MIR metadata proves a concrete stored scalar constant:

```text
route_id = map_lookup.same_key
fusion_op = MapLookupSameKey
receiver_origin_box = MapBox
key_route = i64_const
get_return_shape = scalar_i64_or_missing_zero
get_value_demand = scalar_i64
get_publication_policy = no_publication
has_result_shape = presence_bool
stored_value_const = <i64>
proof = same_receiver_same_i64_key_scalar_get_has
lowering_tier = cold_fallback
block / get_instruction_index match the current call
receiver_value / key_value / get_result_value match the current operands
```

If matched:

```llvm
%get = add i64 0, <stored_value_const>
```

If metadata is missing, existing generic `get` policy remains unchanged. If
metadata is malformed for the current site, fail fast.

## Boundary

- Do not add native i64-key MapBox storage.
- Do not add a runtime helper.
- Do not lower non-constant stored scalar values in this card.
- Do not lower mixed RuntimeData `get` rows.
- Do not infer method semantics from names in `.inc`; consume
  `map_lookup_fusion_routes` only.
- Keep `RuntimeDataBox.get` mixed semantics unchanged.

## Acceptance

- `ny_main` loop no longer calls `nyash.runtime_data.get_hh` for the fused
  constant site.
- `nyash.map.has_h` / `nyash.map.probe_hi` remain absent from the fused loop.
- Keeper requires cycles improvement and no IPC collapse against H148.
- If cycles regress, close as rejected probe and revert code.

## Reject Seams

- Folding a get without `stored_value_const`.
- Folding a mixed `RuntimeDataBox.get`.
- Adding a `.inc` source scanner or method-name semantic planner.
- Adding a storage lane in the same card.

## Probe Result

Implementation:

```text
MapLookupSameKey.get_result -> add i64 0, stored_value_const
MapLookupSameKey.has_result -> add i64 0, 1
```

Observed result:

```text
baseline ny_aot_instr  = 1300470606
baseline ny_aot_cycles = 275315781
baseline ny_aot_ms     = 54
baseline ny_aot_ipc    = 4.72

probe ny_aot_instr     = 6470561
probe ny_aot_cycles    = 2742460
probe ny_aot_ms        = 4
probe ny_aot_ipc       = 2.36
```

Asm confirms the hot loop no longer calls:

```text
nyash.runtime_data.get_hh
nyash.map.has_h
nyash.map.probe_hi
```

The remaining `nyash.map.slot_load_hh` call is outside the loop at the final
post-loop read boundary.

The IPC ratio is no longer comparable to H148 as a primary keeper signal
because the measured hot loop work was removed; the remaining profile is
startup / MapBox creation / final read dominated. The keeper signal is the
cycles/ms collapse plus the disappearance of the target storage calls from the
loop.

## Closeout

- Keep the shared `.inc` reader in
  `hako_llvmc_ffi_map_lookup_fusion_metadata.inc`.
- `get_policy.inc` and `has_policy.inc` may consume that reader, but must not
  rediscover MapGet/MapHas legality.
- Native i64-key MapBox storage remains deferred until a broader front proves
  string-key compatibility work is worth the storage change.
