---
Status: Landed
Date: 2026-04-24
Scope: Promote the proven RuntimeData MapBox-origin `has(i64_const)` route to the i64-key MapHas ABI helper.
Related:
  - docs/development/current/main/CURRENT_STATE.toml
  - docs/development/current/main/design/hotline-core-method-contract-ssot.md
  - docs/development/current/main/phases/phase-291x/291x-138-hcm7-maphas-preflight-evidence-card.md
  - docs/development/current/main/phases/phase-291x/291x-139-receiver-origin-proof-metadata-card.md
  - docs/development/current/main/phases/phase-291x/291x-140-key-route-value-demand-metadata-card.md
  - src/mir/generic_method_route_plan.rs
  - lang/c-abi/shims/hako_llvmc_ffi_generic_method_has_policy.inc
---

# 291x-141 MapHas I64 Route Card

## Goal

Land the smallest no-regress MapHas promotion after `receiver_origin_box` and
`key_route` proof became visible in MIR metadata.

## Implementation

- Added `GenericMethodRouteKind::MapContainsI64`.
- `RuntimeDataBox.has(key)` now promotes to `CoreMethodOp::MapHas` only when:
  - `receiver_origin_box == MapBox`
  - `key_route == i64_const`
- The helper route becomes:

```text
route_kind = map_contains_i64
helper_symbol = nyash.map.probe_hi
```

- The `.inc` consumer now accepts `map_contains_i64` as metadata and emits
  `nyash.map.probe_hi`.
- The fallback `.inc` method-name classifier was not extended.
- `get` routes are unchanged.

## Observed Metadata

For `bench_kilo_leaf_map_getset_has.hako`:

```text
box_name = RuntimeDataBox
receiver_origin_box = MapBox
key_route = i64_const
core_method = MapHas
route_kind = map_contains_i64
helper_symbol = nyash.map.probe_hi
value_demand = read_ref
```

## Perf Gate

Baseline from `291x-138`:

```text
ny_aot_instr=2590470563
ny_aot_cycles=578941277
```

After this card:

```text
ny_aot_instr=2424470642
ny_aot_cycles=567199249
aot_status=ok
```

This is a keeper for the route slice:

```text
instructions: -165,999,921
cycles:       -11,742,028
```

## Remaining Hot Owner

`bench_micro_aot_asm.sh kilo_leaf_map_getset_has ny_main 3` still shows the
main hot owners are string conversion and hashing. The `has` call now resolves
to the i64-key map probe path, while `get` remains on the RuntimeData facade:

```text
call nyash.runtime_data.get_hh
call nyash.map.probe_hi
```

Symbol folding may display the i64 probe alias as `nyash.map.has_h` in the
objdump snippet; the perf report attributes the route to `nyash.map.probe_hi`.

## Boundary

- This card does not add hot inline lowering.
- This card does not promote `RuntimeDataBox.get`.
- This card does not change unknown-key `has`; it remains on the conservative
  `runtime_data_contains_any` path.

## Next

- Add a separate `MapGet` value-demand / return-shape proof before touching
  `get` lowering.
- Keep `get` promotion gated by semantic parity and perf evidence because the
  RuntimeData facade and map slot-load helpers do not currently publish exactly
  the same return-shape contract for every value.
